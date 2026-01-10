//! Layout calculation for sequence diagrams

use mermaid_parser::common::ast::{Participant, SequenceDiagram, SequenceStatement};

use crate::layout::{
    calculate_text_box_height, calculate_text_box_width, split_by_line_breaks, text_width,
    ContentBounds,
};

use super::constants::*;
use super::types::{Layout, ParticipantLayout};

/// Calculate layout and content bounds (first pass - no rendering)
pub fn calculate_layout(diagram: &SequenceDiagram, font_size: u32) -> Layout {
    let mut bounds = ContentBounds::new();

    // Calculate participant dimensions (widths, heights, and lines)
    let (participant_widths, participant_heights, participant_lines) =
        calculate_participant_dimensions(&diagram.participants, font_size);

    // Use the maximum participant width and height for consistent box sizing
    let participant_width = participant_widths
        .iter()
        .copied()
        .fold(MIN_PARTICIPANT_WIDTH, f64::max);
    let participant_height = participant_heights
        .iter()
        .copied()
        .fold(MIN_PARTICIPANT_HEIGHT, f64::max);

    // Create uniform widths vector for gap spacing calculation
    let uniform_widths: Vec<f64> = vec![participant_width; diagram.participants.len()];

    // Calculate dynamic gap spacings based on message lengths and uniform participant widths
    let gap_spacings = calculate_gap_spacings(
        &diagram.participants,
        &uniform_widths,
        &diagram.statements,
        font_size,
    );

    // Calculate participant layouts with uniform width
    let participants = calculate_participant_layouts(
        &diagram.participants,
        &uniform_widths,
        &participant_lines,
        &gap_spacings,
    );

    // Calculate participant bounds
    for p in &participants {
        // Top participant box
        bounds.include_rect(p.left_edge(), PADDING, p.width, participant_height);

        // Top participant name (centered text) - use widest line for bounds
        let max_line_width = p
            .lines
            .iter()
            .map(|line| text_width(line, font_size))
            .fold(0.0_f64, f64::max);
        bounds.include_text(
            p.center_x,
            PADDING + participant_height,
            max_line_width,
            "middle",
        );
    }

    // Calculate message bounds and total height
    let mut message_y = PADDING + participant_height + MESSAGE_SPACING;

    for statement in &diagram.statements {
        if let SequenceStatement::Message(msg) = statement {
            let from_x = find_participant_center(&participants, &msg.from);
            let to_x = find_participant_center(&participants, &msg.to);

            if let (Some(fx), Some(tx)) = (from_x, to_x) {
                if msg.from == msg.to {
                    // Self-message bounds
                    let loop_right = fx + SELF_LOOP_WIDTH;
                    bounds.include_point(loop_right, message_y + SELF_MESSAGE_HEIGHT);

                    // Self-message text (starts after loop)
                    let msg_width = text_width(&msg.text, font_size);
                    bounds.include_text(
                        fx + SELF_LOOP_TEXT_OFFSET,
                        message_y + SELF_MESSAGE_HEIGHT,
                        msg_width,
                        "start",
                    );

                    message_y += MESSAGE_SPACING + SELF_MESSAGE_HEIGHT;
                } else {
                    // Regular message bounds
                    bounds.include_point(fx.max(tx), message_y);

                    // Message text (centered between participants)
                    let text_x = (fx + tx) / 2.0;
                    let msg_width = text_width(&msg.text, font_size);
                    bounds.include_text(text_x, message_y, msg_width, "middle");

                    message_y += MESSAGE_SPACING;
                }
            }
        }
    }

    // Bottom participant boxes
    let bottom_box_y = message_y;
    for p in &participants {
        bounds.include_rect(p.left_edge(), bottom_box_y, p.width, participant_height);

        let max_line_width = p
            .lines
            .iter()
            .map(|line| text_width(line, font_size))
            .fold(0.0_f64, f64::max);
        bounds.include_text(
            p.center_x,
            bottom_box_y + participant_height,
            max_line_width,
            "middle",
        );
    }

    Layout {
        bounds,
        participants,
        participant_height,
        bottom_box_y,
    }
}

/// Find participant center X position by name
pub fn find_participant_center(participants: &[ParticipantLayout], name: &str) -> Option<f64> {
    participants
        .iter()
        .find(|p| p.name == name)
        .map(|p| p.center_x)
}

// =============================================================================
// Internal Helper Functions
// =============================================================================

/// Parse participant display text into lines
fn get_participant_lines(participant: &Participant) -> Vec<String> {
    let display = participant.alias.as_ref().unwrap_or(&participant.actor);
    split_by_line_breaks(display)
}

/// Calculate participant box width based on widest line
fn calculate_participant_width(lines: &[String], font_size: u32) -> f64 {
    calculate_text_box_width(lines, font_size, PARTICIPANT_PADDING).max(MIN_PARTICIPANT_WIDTH)
}

/// Calculate participant box height based on number of lines
fn calculate_participant_height(num_lines: usize) -> f64 {
    calculate_text_box_height(num_lines, LINE_HEIGHT, PARTICIPANT_VERTICAL_PADDING)
        .max(MIN_PARTICIPANT_HEIGHT)
}

/// Calculate all participant widths and heights
fn calculate_participant_dimensions(
    participants: &[Participant],
    font_size: u32,
) -> (Vec<f64>, Vec<f64>, Vec<Vec<String>>) {
    let all_lines: Vec<Vec<String>> = participants.iter().map(get_participant_lines).collect();

    let widths: Vec<f64> = all_lines
        .iter()
        .map(|lines| calculate_participant_width(lines, font_size))
        .collect();

    let heights: Vec<f64> = all_lines
        .iter()
        .map(|lines| calculate_participant_height(lines.len()))
        .collect();

    (widths, heights, all_lines)
}

/// Find participant index by name
fn find_participant_index(participants: &[Participant], name: &str) -> Option<usize> {
    participants.iter().position(|p| p.actor == name)
}

/// Calculate dynamic spacing for each gap between participants based on message lengths
fn calculate_gap_spacings(
    participants: &[Participant],
    participant_widths: &[f64],
    statements: &[SequenceStatement],
    font_size: u32,
) -> Vec<f64> {
    let num_gaps = participants.len().saturating_sub(1);
    if num_gaps == 0 {
        return vec![];
    }

    // Initialize gaps with minimum spacing based on participant widths
    let mut spacings: Vec<f64> = (0..num_gaps)
        .map(|i| {
            let left_half = participant_widths[i] / 2.0;
            let right_half = participant_widths[i + 1] / 2.0;
            left_half + MIN_PARTICIPANT_SPACING + right_half
        })
        .collect();

    for statement in statements {
        if let SequenceStatement::Message(msg) = statement {
            // Skip self-messages (they don't affect gap spacing)
            if msg.from == msg.to {
                continue;
            }

            let from_idx = find_participant_index(participants, &msg.from);
            let to_idx = find_participant_index(participants, &msg.to);

            if let (Some(from_idx), Some(to_idx)) = (from_idx, to_idx) {
                let (min_idx, max_idx) = if from_idx < to_idx {
                    (from_idx, to_idx)
                } else {
                    (to_idx, from_idx)
                };

                // Calculate required width for this message
                let required_width = text_width(&msg.text, font_size) + MESSAGE_TEXT_MARGIN;

                // Calculate current total span across the gaps this message crosses
                let current_span: f64 = spacings[min_idx..max_idx].iter().sum();

                if required_width > current_span {
                    // Need to expand - distribute extra width across spanned gaps
                    let extra = required_width - current_span;
                    let gaps_count = max_idx - min_idx;
                    let extra_per_gap = extra / gaps_count as f64;

                    for spacing in spacings.iter_mut().take(max_idx).skip(min_idx) {
                        *spacing += extra_per_gap;
                    }
                }
            }
        }
    }

    spacings
}

/// Calculate participant layouts from gap spacings, widths, and lines
fn calculate_participant_layouts(
    participants: &[Participant],
    participant_widths: &[f64],
    participant_lines: &[Vec<String>],
    gap_spacings: &[f64],
) -> Vec<ParticipantLayout> {
    let mut layouts = Vec::new();
    let mut center_x = PADDING
        + participant_widths
            .first()
            .copied()
            .unwrap_or(MIN_PARTICIPANT_WIDTH)
            / 2.0;

    for (i, participant) in participants.iter().enumerate() {
        let width = participant_widths
            .get(i)
            .copied()
            .unwrap_or(MIN_PARTICIPANT_WIDTH);
        let lines = participant_lines
            .get(i)
            .cloned()
            .unwrap_or_else(|| vec![participant.actor.clone()]);

        layouts.push(ParticipantLayout {
            name: participant.actor.clone(),
            lines,
            center_x,
            width,
        });

        if i < gap_spacings.len() {
            center_x += gap_spacings[i];
        }
    }

    layouts
}
