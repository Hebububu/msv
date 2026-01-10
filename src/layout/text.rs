//! Text measurement utilities
//!
//! Provides approximate text width calculations for layout purposes.
//! These utilities help position text elements without requiring
//! actual font rendering.

/// Returns approximate character width for Arial font at 14px base size.
///
/// Based on common character width categories in proportional fonts.
fn char_width(c: char) -> f64 {
    match c {
        // Narrow characters
        'i' | 'j' | 'l' | '!' | '|' | '.' | ',' | ':' | ';' | '\'' | '`' => 4.0,
        'I' | 'f' | 't' | 'r' => 5.0,

        // Medium-narrow characters
        ' ' | '-' | '(' | ')' | '[' | ']' | '{' | '}' => 5.0,

        // Medium characters (most lowercase)
        'a' | 'b' | 'c' | 'd' | 'e' | 'g' | 'h' | 'k' | 'n' | 'o' | 'p' | 'q' | 's' | 'u' | 'v'
        | 'x' | 'y' | 'z' => 7.0,

        // Medium-wide characters
        'w' | 'm' => 10.0,

        // Uppercase (wider than lowercase)
        'A' | 'B' | 'C' | 'D' | 'E' | 'F' | 'G' | 'H' | 'K' | 'L' | 'N' | 'O' | 'P' | 'Q' | 'R'
        | 'S' | 'T' | 'U' | 'V' | 'X' | 'Y' | 'Z' => 9.0,
        'M' | 'W' => 11.0,

        // Numbers
        '0'..='9' => 7.0,

        // Wide special characters
        '@' | '#' | '$' | '%' | '&' | '+' | '=' | '<' | '>' | '?' | '/' | '\\' | '"' | '*' => 8.0,

        // Default for unknown characters
        _ => 7.0,
    }
}

/// Calculates approximate pixel width of text for layout purposes
///
/// Uses character-width approximations for Arial font family,
/// scaled proportionally to the specified font size.
///
/// # Arguments
///
/// * `text` - The text string to measure
/// * `font_size` - Font size in pixels (base calculation uses 14px)
///
/// # Returns
///
/// Approximate width in pixels
pub fn text_width(text: &str, font_size: u32) -> f64 {
    let base_width: f64 = text.chars().map(char_width).sum();
    base_width * (font_size as f64 / 14.0)
}

/// Splits text by HTML line break markers or newlines
///
/// Recognizes `<br>`, `<br/>`, and `\n` as line separators.
/// Trims whitespace from each line and filters empty lines.
///
/// # Arguments
///
/// * `text` - Text potentially containing line break markers
///
/// # Returns
///
/// Vector of trimmed, non-empty line strings
pub fn split_by_line_breaks(text: &str) -> Vec<String> {
    text.replace("<br/>", "\n")
        .replace("<br>", "\n")
        .split('\n')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

/// Calculates the width of a text box containing multiple lines
///
/// Finds the widest line and adds horizontal padding.
///
/// # Arguments
///
/// * `lines` - Lines of text to measure
/// * `font_size` - Font size in pixels
/// * `padding` - Total horizontal padding (both sides combined)
///
/// # Returns
///
/// Width in pixels (max line width + padding)
pub fn calculate_text_box_width(lines: &[String], font_size: u32, padding: f64) -> f64 {
    let max_line_width = lines
        .iter()
        .map(|line| text_width(line, font_size))
        .fold(0.0_f64, f64::max);
    max_line_width + padding
}

/// Calculates the height of a text box containing multiple lines
///
/// # Arguments
///
/// * `num_lines` - Number of text lines (minimum 1 for empty content)
/// * `line_height` - Height per line in pixels
/// * `padding` - Total vertical padding (top + bottom combined)
///
/// # Returns
///
/// Height in pixels (lines * line_height + padding)
pub fn calculate_text_box_height(num_lines: usize, line_height: f64, padding: f64) -> f64 {
    let effective_lines = if num_lines == 0 { 1 } else { num_lines };
    (effective_lines as f64) * line_height + padding
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_width_basic() {
        let width = text_width("Hello", 14);
        assert!(width > 0.0);
    }

    #[test]
    fn test_text_width_scales_with_font_size() {
        let width_14 = text_width("Test", 14);
        let width_28 = text_width("Test", 28);
        assert!((width_28 - width_14 * 2.0).abs() < 0.001);
    }

    #[test]
    fn test_split_by_line_breaks_br() {
        let lines = split_by_line_breaks("Hello<br>World");
        assert_eq!(lines, vec!["Hello", "World"]);
    }

    #[test]
    fn test_split_by_line_breaks_br_slash() {
        let lines = split_by_line_breaks("Hello<br/>World");
        assert_eq!(lines, vec!["Hello", "World"]);
    }

    #[test]
    fn test_split_by_line_breaks_newline() {
        let lines = split_by_line_breaks("Hello\nWorld");
        assert_eq!(lines, vec!["Hello", "World"]);
    }

    #[test]
    fn test_split_by_line_breaks_trims() {
        let lines = split_by_line_breaks("  Hello  <br>  World  ");
        assert_eq!(lines, vec!["Hello", "World"]);
    }

    #[test]
    fn test_text_box_width_single_line() {
        let lines = vec!["Hello".to_string()];
        let width = calculate_text_box_width(&lines, 14, 20.0);
        // Width should be text width + padding
        let text_w = text_width("Hello", 14);
        assert!((width - (text_w + 20.0)).abs() < 0.001);
    }

    #[test]
    fn test_text_box_width_uses_widest_line() {
        let lines = vec![
            "Hi".to_string(),
            "Hello World".to_string(),
            "Hey".to_string(),
        ];
        let width = calculate_text_box_width(&lines, 14, 20.0);
        let widest = text_width("Hello World", 14);
        assert!((width - (widest + 20.0)).abs() < 0.001);
    }

    #[test]
    fn test_text_box_width_empty_lines() {
        let lines: Vec<String> = vec![];
        let width = calculate_text_box_width(&lines, 14, 20.0);
        // Empty lines should return just padding
        assert!((width - 20.0).abs() < 0.001);
    }

    #[test]
    fn test_text_box_height_single_line() {
        let height = calculate_text_box_height(1, 18.0, 16.0);
        assert!((height - 34.0).abs() < 0.001); // 1 * 18 + 16
    }

    #[test]
    fn test_text_box_height_multiple_lines() {
        let height = calculate_text_box_height(3, 18.0, 16.0);
        assert!((height - 70.0).abs() < 0.001); // 3 * 18 + 16
    }

    #[test]
    fn test_text_box_height_zero_lines_defaults_to_one() {
        let height = calculate_text_box_height(0, 18.0, 16.0);
        assert!((height - 34.0).abs() < 0.001); // 1 * 18 + 16
    }
}
