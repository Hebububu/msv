//! SVG document builder
//!
//! Provides a builder pattern for constructing SVG documents with
//! theme-aware styling and optional transparent backgrounds.

use std::fmt;

use crate::options::ThemeColors;

/// Builder for constructing SVG documents
///
/// Collects SVG elements and renders them into a complete SVG document
/// with proper XML structure, dimensions, and background handling.
pub struct SvgBuilder {
    width: u32,
    height: u32,
    elements: Vec<String>,
    colors: ThemeColors,
    transparent: bool,
}

impl SvgBuilder {
    /// Creates a new SVG builder with specified dimensions and theme
    ///
    /// # Arguments
    ///
    /// * `width` - Width of the SVG canvas in pixels
    /// * `height` - Height of the SVG canvas in pixels
    /// * `colors` - Theme colors for styling elements
    /// * `transparent` - If `true`, omits the background rectangle
    pub fn new(width: u32, height: u32, colors: ThemeColors, transparent: bool) -> Self {
        Self {
            width,
            height,
            elements: Vec::new(),
            colors,
            transparent,
        }
    }

    /// Returns a reference to the theme colors
    #[allow(dead_code)]
    pub fn colors(&self) -> &ThemeColors {
        &self.colors
    }

    /// Adds an SVG element string to the document
    ///
    /// Elements are rendered in the order they are added.
    pub fn add_element(&mut self, element: String) {
        self.elements.push(element);
    }
}

impl fmt::Display for SvgBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let elements_str = self.elements.join("\n  ");

        let background = if self.transparent {
            String::new()
        } else {
            format!(
                r#"<rect width="100%" height="100%" fill="{}"/>"#,
                self.colors.background
            )
        };

        if background.is_empty() {
            write!(
                f,
                r#"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" viewBox="0 0 {} {}">
  {}
</svg>"#,
                self.width, self.height, self.width, self.height, elements_str
            )
        } else {
            write!(
                f,
                r#"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" viewBox="0 0 {} {}">
  {}
  {}
</svg>"#,
                self.width, self.height, self.width, self.height, background, elements_str
            )
        }
    }
}
