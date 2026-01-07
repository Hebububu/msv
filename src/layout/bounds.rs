//! Content bounds tracking for SVG rendering

/// Tracks the bounding box of rendered content
#[derive(Debug, Clone)]
pub struct ContentBounds {
    max_x: f64,
    max_y: f64,
}

impl ContentBounds {
    /// Create a new empty bounds tracker
    pub fn new() -> Self {
        Self {
            max_x: 0.0,
            max_y: 0.0,
        }
    }

    /// Expand bounds to include a point
    pub fn include_point(&mut self, x: f64, y: f64) {
        self.max_x = self.max_x.max(x);
        self.max_y = self.max_y.max(y);
    }

    /// Expand bounds to include a rectangle
    pub fn include_rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.include_point(x + width, y + height);
    }

    /// Expand bounds to include text (anchor: start, middle, end)
    pub fn include_text(&mut self, x: f64, y: f64, text_width: f64, anchor: &str) {
        let right_edge = match anchor {
            "start" => x + text_width,
            "middle" => x + text_width / 2.0,
            "end" => x,
            _ => x + text_width,
        };
        self.include_point(right_edge, y);
    }

    /// Get final SVG dimensions with padding
    pub fn svg_size(&self, padding: f64) -> (u32, u32) {
        let width = (self.max_x + padding).ceil() as u32;
        let height = (self.max_y + padding).ceil() as u32;
        (width, height)
    }
}

impl Default for ContentBounds {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bounds_new() {
        let bounds = ContentBounds::new();
        assert_eq!(bounds.svg_size(0.0), (0, 0));
    }

    #[test]
    fn test_bounds_include_point() {
        let mut bounds = ContentBounds::new();
        bounds.include_point(100.0, 200.0);
        assert_eq!(bounds.svg_size(0.0), (100, 200));
    }

    #[test]
    fn test_bounds_include_rect() {
        let mut bounds = ContentBounds::new();
        bounds.include_rect(10.0, 20.0, 100.0, 50.0);
        assert_eq!(bounds.svg_size(0.0), (110, 70));
    }

    #[test]
    fn test_bounds_with_padding() {
        let mut bounds = ContentBounds::new();
        bounds.include_point(100.0, 100.0);
        assert_eq!(bounds.svg_size(20.0), (120, 120));
    }
}
