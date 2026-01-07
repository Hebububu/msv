//! Sequence diagram renderer

mod constants;
mod layout;
mod render;
mod types;

use mermaid_parser::common::ast::SequenceDiagram;

use crate::error::RenderResult;
use crate::options::RenderOptions;
use crate::svg::SvgBuilder;

use constants::PADDING;
use layout::calculate_layout;
use render::{draw_messages, draw_participants};

/// Render a sequence diagram to SVG
pub fn render(diagram: &SequenceDiagram, options: &RenderOptions) -> RenderResult<String> {
    let colors = options.colors();

    // First pass: calculate layout and bounds
    let layout = calculate_layout(diagram, options.font_size);
    let (width, height) = layout.bounds.svg_size(PADDING);

    // Second pass: render with calculated dimensions
    let mut builder = SvgBuilder::new(width, height, colors.clone(), options.transparent_bg);

    // Draw participants
    draw_participants(
        &mut builder,
        &layout.participants,
        options,
        layout.participant_height,
        layout.bottom_box_y,
    );

    // Draw messages
    draw_messages(
        &mut builder,
        diagram,
        &layout.participants,
        options,
        layout.participant_height,
        layout.bottom_box_y,
    );

    Ok(builder.to_string())
}
