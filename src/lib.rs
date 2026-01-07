//! # mermaid-svg-render
//!
#![warn(missing_docs)]

//! Parse and render Mermaid diagrams to SVG in pure Rust.
//!
//! This crate provides SVG rendering capabilities for Mermaid diagrams,
//! building on top of the `mermaid-parser` crate for parsing.
//!
//! ## Features
//!
//! - Pure Rust implementation (no JavaScript/npm dependencies)
//! - Light and dark theme support
//! - Sequence diagram rendering (more diagram types coming soon)
//!
//! ## Quick Start
//!
//! ```rust,ignore
//! use mermaid_svg_render::{render_sequence_diagram, RenderOptions, Theme};
//!
//! let input = r#"
//! sequenceDiagram
//!     Alice->>Bob: Hello Bob!
//!     Bob-->>Alice: Hello Alice!
//! "#;
//!
//! let options = RenderOptions::with_theme(Theme::Light);
//! let svg = render_sequence_diagram(input, &options).unwrap();
//! println!("{}", svg);
//! ```

pub mod error;
pub mod layout;
pub mod options;
pub mod sequence;
pub mod svg;

pub use error::{RenderError, RenderResult};
pub use options::{RenderOptions, Theme, ThemeColors};

// Re-export mermaid-parser for convenience
pub use mermaid_parser::{parse_diagram, DiagramType, ParseError};

/// Render a sequence diagram from Mermaid source text to SVG
///
/// This is a convenience function that parses and renders in one step.
///
/// # Arguments
///
/// * `input` - The Mermaid sequence diagram source text
/// * `options` - Rendering options (theme, dimensions, etc.)
///
/// # Returns
///
/// Returns a `RenderResult<String>` containing the SVG markup.
///
/// # Example
///
/// ```rust,ignore
/// use mermaid_svg_render::{render_sequence_diagram, RenderOptions, Theme};
///
/// let input = r#"
/// sequenceDiagram
///     Alice->>Bob: Hello!
///     Bob-->>Alice: Hi!
/// "#;
///
/// let svg = render_sequence_diagram(input, &RenderOptions::default()).unwrap();
/// ```
pub fn render_sequence_diagram(input: &str, options: &RenderOptions) -> RenderResult<String> {
    let diagram = parse_diagram(input).map_err(|e| RenderError::ParseError(e.to_string()))?;

    match diagram {
        DiagramType::Sequence(seq) => sequence::render(&seq, options),
        _ => Err(RenderError::UnsupportedDiagram(
            "Expected a sequence diagram".to_string(),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_simple_sequence() {
        let input = r#"
sequenceDiagram
    Alice->>Bob: Hello Bob!
    Bob-->>Alice: Hello Alice!
"#;
        let options = RenderOptions::default();
        let result = render_sequence_diagram(input, &options);
        assert!(result.is_ok());
        let svg = result.unwrap();
        assert!(svg.contains("<svg"));
        assert!(svg.contains("Alice"));
        assert!(svg.contains("Bob"));
    }

    #[test]
    fn test_dark_theme() {
        let input = r#"
sequenceDiagram
    A->>B: Test
"#;
        let options = RenderOptions::with_theme(Theme::Dark);
        let result = render_sequence_diagram(input, &options);
        assert!(result.is_ok());
    }
}
