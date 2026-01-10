//! Shared layout utilities for diagram rendering

mod bounds;
mod text;

pub use bounds::ContentBounds;
pub use text::{
    calculate_text_box_height, calculate_text_box_width, split_by_line_breaks, text_width,
};
