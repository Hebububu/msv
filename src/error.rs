//! Error types for rendering operations

use std::fmt;

/// Result type for rendering operations
pub type RenderResult<T> = std::result::Result<T, RenderError>;

/// Errors that can occur during rendering
#[derive(Debug, Clone, PartialEq)]
pub enum RenderError {
    /// Error parsing the input diagram
    ParseError(String),
    /// The diagram type is not supported for rendering
    UnsupportedDiagram(String),
    /// Error during SVG generation
    SvgError(String),
    /// Invalid render options
    InvalidOptions(String),
}

impl fmt::Display for RenderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RenderError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            RenderError::UnsupportedDiagram(msg) => write!(f, "Unsupported diagram: {}", msg),
            RenderError::SvgError(msg) => write!(f, "SVG error: {}", msg),
            RenderError::InvalidOptions(msg) => write!(f, "Invalid options: {}", msg),
        }
    }
}

impl std::error::Error for RenderError {}
