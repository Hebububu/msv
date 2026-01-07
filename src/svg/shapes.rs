//! SVG shape primitives with composable arrow rendering

/// Line style for arrows
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LineStyle {
    /// Continuous solid line
    Solid,
    /// Dashed/dotted line pattern
    Dotted,
}

/// End/head style for arrows
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EndStyle {
    /// No arrowhead (open line)
    None,
    /// Filled triangle arrowhead (closed)
    Closed,
    /// V-shape open arrowhead (async)
    Open,
    /// X-shape cross
    Cross,
}

// =============================================================================
// Core Composable Functions
// =============================================================================

/// Create a line segment with specified style
pub fn create_line(x1: f64, y1: f64, x2: f64, y2: f64, stroke: &str, style: LineStyle) -> String {
    let dash = match style {
        LineStyle::Dotted => r#" stroke-dasharray="5,5""#,
        LineStyle::Solid => "",
    };
    format!(
        r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="{}" stroke-width="1"{}/>"#,
        x1, y1, x2, y2, stroke, dash
    )
}

/// Create an arrow end/head at a specific point with direction
///
/// # Arguments
/// * `x`, `y` - The tip position of the arrowhead
/// * `angle` - Direction the arrow is pointing (in radians)
/// * `stroke` - Color/stroke style
/// * `style` - Type of end marker to draw
pub fn create_end(x: f64, y: f64, angle: f64, stroke: &str, style: EndStyle) -> String {
    match style {
        EndStyle::None => String::new(),
        EndStyle::Closed => create_end_closed(x, y, angle, stroke),
        EndStyle::Open => create_end_open(x, y, angle, stroke),
        EndStyle::Cross => create_end_cross(x, y, stroke),
    }
}

/// Create a filled triangle arrowhead
fn create_end_closed(x: f64, y: f64, angle: f64, stroke: &str) -> String {
    let arrow_length = 10.0;
    let arrow_angle = 0.5; // ~30 degrees

    let ax1 = x - arrow_length * (angle - arrow_angle).cos();
    let ay1 = y - arrow_length * (angle - arrow_angle).sin();
    let ax2 = x - arrow_length * (angle + arrow_angle).cos();
    let ay2 = y - arrow_length * (angle + arrow_angle).sin();

    format!(
        r#"<polygon points="{},{} {},{} {},{}" fill="{}"/>"#,
        x, y, ax1, ay1, ax2, ay2, stroke
    )
}

/// Create a V-shape open arrowhead (async style)
fn create_end_open(x: f64, y: f64, angle: f64, stroke: &str) -> String {
    let arrow_length = 10.0;
    let arrow_angle = 0.5;

    let ax1 = x - arrow_length * (angle - arrow_angle).cos();
    let ay1 = y - arrow_length * (angle - arrow_angle).sin();
    let ax2 = x - arrow_length * (angle + arrow_angle).cos();
    let ay2 = y - arrow_length * (angle + arrow_angle).sin();

    format!(
        r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="{}" stroke-width="1"/>
<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="{}" stroke-width="1"/>"#,
        ax1, ay1, x, y, stroke, ax2, ay2, x, y, stroke
    )
}

/// Create an X-shape cross marker
fn create_end_cross(x: f64, y: f64, stroke: &str) -> String {
    let cross_size = 6.0;

    format!(
        r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="{}" stroke-width="1"/>
<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="{}" stroke-width="1"/>"#,
        x - cross_size,
        y - cross_size,
        x + cross_size,
        y + cross_size,
        stroke,
        x - cross_size,
        y + cross_size,
        x + cross_size,
        y - cross_size,
        stroke
    )
}

// =============================================================================
// High-Level Composer Functions
// =============================================================================

/// Create a complete arrow with line and optional end markers
///
/// # Arguments
/// * `x1`, `y1` - Start point
/// * `x2`, `y2` - End point
/// * `stroke` - Color/stroke style
/// * `line_style` - Solid or dotted line
/// * `start_end` - End marker at start point
/// * `end_end` - End marker at end point
#[allow(clippy::too_many_arguments)]
pub fn create_arrow(
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    stroke: &str,
    line_style: LineStyle,
    start_end: EndStyle,
    end_end: EndStyle,
) -> String {
    let line = create_line(x1, y1, x2, y2, stroke, line_style);

    // Calculate angle from start to end
    let angle = (y2 - y1).atan2(x2 - x1);
    let reverse_angle = angle + std::f64::consts::PI;

    let end_marker = create_end(x2, y2, angle, stroke, end_end);
    let start_marker = create_end(x1, y1, reverse_angle, stroke, start_end);

    // Combine parts, filtering empty strings
    [line, end_marker, start_marker]
        .into_iter()
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

/// Create a self-referencing loop arrow (for self-messages)
///
/// # Arguments
/// * `x` - X position (participant center)
/// * `y` - Y position (message row)
/// * `stroke` - Color/stroke style
/// * `line_style` - Solid or dotted line
pub fn create_self_loop(x: f64, y: f64, stroke: &str, line_style: LineStyle) -> String {
    let dash = match line_style {
        LineStyle::Dotted => r#" stroke-dasharray="5,5""#,
        LineStyle::Solid => "",
    };

    let loop_width = 40.0;
    let loop_height = 30.0;

    // Quadratic bezier curves for oval shape
    format!(
        r#"<path d="M {} {} Q {} {} {} {} Q {} {} {} {}" fill="none" stroke="{}" stroke-width="1"{}/>
<polygon points="{},{} {},{} {},{}" fill="{}"/>"#,
        // Start point
        x,
        y,
        // First control point (top-right), end at right-middle
        x + loop_width,
        y,
        x + loop_width,
        y + loop_height / 2.0,
        // Second control point (bottom-right), end at left-bottom
        x + loop_width,
        y + loop_height,
        x,
        y + loop_height,
        stroke,
        dash,
        // Arrowhead pointing left at the end
        x,
        y + loop_height,
        x + 8.0,
        y + loop_height - 5.0,
        x + 8.0,
        y + loop_height + 5.0,
        stroke
    )
}

// =============================================================================
// Basic Shape Primitives
// =============================================================================

/// Draw a rectangle
pub fn draw_rect(x: f64, y: f64, width: f64, height: f64, fill: &str, stroke: &str) -> String {
    format!(
        r#"<rect x="{}" y="{}" width="{}" height="{}" fill="{}" stroke="{}" stroke-width="1" rx="4"/>"#,
        x, y, width, height, fill, stroke
    )
}

/// Draw text
pub fn draw_text(x: f64, y: f64, text: &str, fill: &str, font_size: u32, anchor: &str) -> String {
    format!(
        r#"<text x="{}" y="{}" fill="{}" font-size="{}" font-family="Arial, sans-serif" text-anchor="{}">{}</text>"#,
        x,
        y,
        fill,
        font_size,
        anchor,
        escape_xml(text)
    )
}

/// Draw multi-line text centered vertically
///
/// SVG text y coordinate is the baseline, so we need to adjust for visual centering.
/// The baseline adjustment accounts for the fact that text renders above its y coordinate.
pub fn draw_multiline_text(
    x: f64,
    center_y: f64,
    lines: &[String],
    fill: &str,
    font_size: u32,
    line_height: f64,
    anchor: &str,
) -> String {
    if lines.is_empty() {
        return String::new();
    }

    // Baseline adjustment: text renders above y, so shift down to visually center
    // Approximately 0.35 * font_size works well for most fonts
    let baseline_adjustment = font_size as f64 * 0.35;

    // Total height from first baseline to last baseline
    let total_height = (lines.len() - 1) as f64 * line_height;

    // Start y: center the text block, then add baseline adjustment
    let start_y = center_y - total_height / 2.0 + baseline_adjustment;

    lines
        .iter()
        .enumerate()
        .map(|(i, line)| {
            let y = start_y + i as f64 * line_height;
            format!(
                r#"<text x="{}" y="{}" fill="{}" font-size="{}" font-family="Arial, sans-serif" text-anchor="{}">{}</text>"#,
                x, y, fill, font_size, anchor, escape_xml(line)
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Escape XML special characters
fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

// =============================================================================
// Legacy API (for backwards compatibility during transition)
// =============================================================================

/// Draw a line (legacy wrapper)
pub fn draw_line(x1: f64, y1: f64, x2: f64, y2: f64, stroke: &str, dashed: bool) -> String {
    let style = if dashed {
        LineStyle::Dotted
    } else {
        LineStyle::Solid
    };
    create_line(x1, y1, x2, y2, stroke, style)
}
