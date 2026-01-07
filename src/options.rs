//! Render options and theme configuration
//!
//! This module provides configuration types for customizing diagram rendering,
//! including theme selection and color schemes.

/// Theme for rendering diagrams
///
/// Controls the overall color scheme of the rendered SVG.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Theme {
    /// Light theme with white background and dark text
    #[default]
    Light,
    /// Dark theme with dark background and light text
    Dark,
}

/// Colors used for rendering a specific theme
///
/// Contains all color values needed to render diagram elements consistently.
#[derive(Debug, Clone)]
pub struct ThemeColors {
    /// Background color of the SVG canvas
    pub background: String,
    /// Primary text color
    pub text: String,
    /// Color for lines and arrows
    pub line: String,
    /// Background color for participant boxes
    pub participant_bg: String,
    /// Border color for participant boxes
    pub participant_border: String,
}

impl ThemeColors {
    /// Returns the light theme color palette
    pub fn light() -> Self {
        Self {
            background: "#ffffff".to_string(),
            text: "#333333".to_string(),
            line: "#333333".to_string(),
            participant_bg: "#ecf0f1".to_string(),
            participant_border: "#333333".to_string(),
        }
    }

    /// Returns the dark theme color palette
    pub fn dark() -> Self {
        Self {
            background: "#1a1a2e".to_string(),
            text: "#eaeaea".to_string(),
            line: "#eaeaea".to_string(),
            participant_bg: "#16213e".to_string(),
            participant_border: "#eaeaea".to_string(),
        }
    }
}

/// Configuration options for rendering diagrams
///
/// Use the builder pattern methods to customize rendering:
///
/// ```rust
/// use mermaid_svg_render::{RenderOptions, Theme};
///
/// let options = RenderOptions::with_theme(Theme::Dark)
///     .transparent();
/// ```
#[derive(Debug, Clone)]
pub struct RenderOptions {
    /// The color theme to use for rendering
    pub theme: Theme,
    /// Optional fixed width in pixels (auto-calculated if `None`)
    pub width: Option<u32>,
    /// Optional fixed height in pixels (auto-calculated if `None`)
    pub height: Option<u32>,
    /// Padding around the diagram content in pixels
    pub padding: u32,
    /// Font family for text rendering
    pub font_family: String,
    /// Font size in pixels
    pub font_size: u32,
    /// Whether to use a transparent background instead of solid color
    pub transparent_bg: bool,
}

impl Default for RenderOptions {
    fn default() -> Self {
        Self {
            theme: Theme::Light,
            width: None,
            height: None,
            padding: 20,
            font_family: "Arial, sans-serif".to_string(),
            font_size: 14,
            transparent_bg: false,
        }
    }
}

impl RenderOptions {
    /// Creates render options with the specified theme
    ///
    /// All other options use default values.
    pub fn with_theme(theme: Theme) -> Self {
        Self {
            theme,
            ..Default::default()
        }
    }

    /// Returns the color palette for the current theme
    pub fn colors(&self) -> ThemeColors {
        match self.theme {
            Theme::Light => ThemeColors::light(),
            Theme::Dark => ThemeColors::dark(),
        }
    }

    /// Enables transparent background (builder pattern)
    ///
    /// When enabled, the SVG will have no background rectangle,
    /// allowing the underlying page color to show through.
    pub fn transparent(mut self) -> Self {
        self.transparent_bg = true;
        self
    }
}
