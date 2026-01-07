//! Shared layout utilities for diagram rendering

mod bounds;
mod text;

pub use bounds::ContentBounds;
pub use text::{split_by_line_breaks, text_width};
