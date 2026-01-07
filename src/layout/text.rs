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
}
