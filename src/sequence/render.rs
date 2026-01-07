//! Rendering functions for sequence diagrams

use mermaid_parser::common::ast::{ArrowType, SequenceDiagram, SequenceStatement};

use crate::options::RenderOptions;
use crate::svg::{
    create_arrow, create_line, create_self_loop, draw_multiline_text, draw_rect, draw_text,
    EndStyle, LineStyle, SvgBuilder,
};

use super::constants::*;
use super::layout::find_participant_center;
use super::types::ParticipantLayout;

/// Draw all participants (boxes at top and bottom, lifelines)
pub fn draw_participants(
    builder: &mut SvgBuilder,
    participants: &[ParticipantLayout],
    options: &RenderOptions,
    participant_height: f64,
    bottom_box_y: f64,
) {
    let colors = options.colors();

    for p in participants {
        // Top participant box
        builder.add_element(draw_rect(
            p.left_edge(),
            PADDING,
            p.width,
            participant_height,
            &colors.participant_bg,
            &colors.participant_border,
        ));

        // Top participant name (single or multi-line)
        let center_y = PADDING + participant_height / 2.0;
        if p.lines.len() == 1 {
            builder.add_element(draw_text(
                p.center_x,
                center_y + 5.0, // Baseline adjustment
                &p.lines[0],
                &colors.text,
                options.font_size,
                "middle",
            ));
        } else {
            builder.add_element(draw_multiline_text(
                p.center_x,
                center_y,
                &p.lines,
                &colors.text,
                options.font_size,
                LINE_HEIGHT,
                "middle",
            ));
        }

        // Lifeline
        let lifeline_start = PADDING + participant_height;
        let lifeline_end = bottom_box_y;
        builder.add_element(create_line(
            p.center_x,
            lifeline_start,
            p.center_x,
            lifeline_end,
            &colors.line,
            LineStyle::Solid,
        ));

        // Bottom participant box
        builder.add_element(draw_rect(
            p.left_edge(),
            bottom_box_y,
            p.width,
            participant_height,
            &colors.participant_bg,
            &colors.participant_border,
        ));

        // Bottom participant name (single or multi-line)
        let bottom_center_y = bottom_box_y + participant_height / 2.0;
        if p.lines.len() == 1 {
            builder.add_element(draw_text(
                p.center_x,
                bottom_center_y + 5.0, // Baseline adjustment
                &p.lines[0],
                &colors.text,
                options.font_size,
                "middle",
            ));
        } else {
            builder.add_element(draw_multiline_text(
                p.center_x,
                bottom_center_y,
                &p.lines,
                &colors.text,
                options.font_size,
                LINE_HEIGHT,
                "middle",
            ));
        }
    }
}

/// Draw all messages between participants
pub fn draw_messages(
    builder: &mut SvgBuilder,
    diagram: &SequenceDiagram,
    participants: &[ParticipantLayout],
    options: &RenderOptions,
    participant_height: f64,
    bottom_box_y: f64,
) {
    let colors = options.colors();
    let mut message_y = PADDING + participant_height + MESSAGE_SPACING;

    for statement in &diagram.statements {
        if let SequenceStatement::Message(msg) = statement {
            let from_x = find_participant_center(participants, &msg.from);
            let to_x = find_participant_center(participants, &msg.to);

            if let (Some(fx), Some(tx)) = (from_x, to_x) {
                if msg.from == msg.to {
                    // Self-message
                    if message_y + SELF_MESSAGE_HEIGHT <= bottom_box_y {
                        let line_style = if is_dotted_arrow(&msg.arrow_type) {
                            LineStyle::Dotted
                        } else {
                            LineStyle::Solid
                        };

                        builder.add_element(create_self_loop(
                            fx,
                            message_y,
                            &colors.line,
                            line_style,
                        ));

                        builder.add_element(draw_text(
                            fx + SELF_LOOP_TEXT_OFFSET,
                            message_y + SELF_MESSAGE_HEIGHT / 2.0,
                            &msg.text,
                            &colors.text,
                            options.font_size,
                            "start",
                        ));
                    }
                    message_y += MESSAGE_SPACING + SELF_MESSAGE_HEIGHT;
                } else {
                    // Normal message
                    let (line_style, start_end, end_end) = arrow_type_to_styles(&msg.arrow_type);

                    builder.add_element(create_arrow(
                        fx,
                        message_y,
                        tx,
                        message_y,
                        &colors.line,
                        line_style,
                        start_end,
                        end_end,
                    ));

                    let text_x = (fx + tx) / 2.0;
                    builder.add_element(draw_text(
                        text_x,
                        message_y - 10.0,
                        &msg.text,
                        &colors.text,
                        options.font_size,
                        "middle",
                    ));

                    message_y += MESSAGE_SPACING;
                }
            }
        }
    }
}

// =============================================================================
// Arrow Type Conversion
// =============================================================================

/// Convert mermaid ArrowType to our composable line and end styles
fn arrow_type_to_styles(arrow_type: &ArrowType) -> (LineStyle, EndStyle, EndStyle) {
    match arrow_type {
        ArrowType::SolidOpen => (LineStyle::Solid, EndStyle::None, EndStyle::None),
        ArrowType::SolidClosed => (LineStyle::Solid, EndStyle::None, EndStyle::Closed),
        ArrowType::Cross => (LineStyle::Solid, EndStyle::None, EndStyle::Cross),
        ArrowType::Point => (LineStyle::Solid, EndStyle::None, EndStyle::Open),
        ArrowType::BiDirectionalSolid => (LineStyle::Solid, EndStyle::Closed, EndStyle::Closed),
        ArrowType::DottedOpen => (LineStyle::Dotted, EndStyle::None, EndStyle::None),
        ArrowType::DottedClosed => (LineStyle::Dotted, EndStyle::None, EndStyle::Closed),
        ArrowType::BiDirectionalDotted => (LineStyle::Dotted, EndStyle::Closed, EndStyle::Closed),
    }
}

/// Check if an arrow type uses dotted line style
fn is_dotted_arrow(arrow_type: &ArrowType) -> bool {
    matches!(
        arrow_type,
        ArrowType::DottedOpen | ArrowType::DottedClosed | ArrowType::BiDirectionalDotted
    )
}
