//! Type definitions for sequence diagram layout

use crate::layout::ContentBounds;

/// Layout information for a single participant
#[derive(Debug, Clone)]
pub struct ParticipantLayout {
    /// Actor name (for lookup)
    pub name: String,
    /// Display lines (split by line breaks)
    pub lines: Vec<String>,
    /// Center X position
    pub center_x: f64,
    /// Box width (uniform across all participants)
    pub width: f64,
}

impl ParticipantLayout {
    /// Get the left edge X position of the participant box
    pub fn left_edge(&self) -> f64 {
        self.center_x - self.width / 2.0
    }
}

/// Calculated layout information for rendering
pub struct Layout {
    /// Content bounds for SVG sizing
    pub bounds: ContentBounds,
    /// Participant layouts
    pub participants: Vec<ParticipantLayout>,
    /// Uniform height for all participant boxes
    pub participant_height: f64,
    /// Y position of bottom participant boxes
    pub bottom_box_y: f64,
}
