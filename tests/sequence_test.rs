//! Integration tests for sequence diagram rendering

use mermaid_svg_render::{render_sequence_diagram, RenderOptions, Theme};
use std::env;
use std::fs;
use std::path::Path;

const OUTPUT_DIR: &str = "target/test_svg";

/// Save SVG to file if SAVE_SVG=1 is set
fn maybe_save_svg(svg: &str, theme: &str, name: &str) {
    if env::var("SAVE_SVG").is_ok() {
        let dir = Path::new(OUTPUT_DIR).join(theme);
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join(format!("{}.svg", name)), svg).unwrap();
        println!("Saved: {}/{}/{}.svg", OUTPUT_DIR, theme, name);
    }
}

// ============================================
// Basic Functionality Tests
// ============================================

#[test]
fn test_two_participants_light() {
    let input = r#"
sequenceDiagram
    Alice->>Bob: Hello Bob!
    Bob-->>Alice: Hello Alice!
"#;
    let options = RenderOptions::with_theme(Theme::Light);
    let svg = render_sequence_diagram(input, &options).unwrap();

    maybe_save_svg(&svg, "light", "two_participants");

    assert!(svg.contains("<svg"));
    assert!(svg.contains("Alice"));
    assert!(svg.contains("Bob"));
    assert!(svg.contains("Hello Bob!"));
}

#[test]
fn test_two_participants_dark() {
    let input = r#"
sequenceDiagram
    Alice->>Bob: Hello Bob!
    Bob-->>Alice: Hello Alice!
"#;
    let options = RenderOptions::with_theme(Theme::Dark);
    let svg = render_sequence_diagram(input, &options).unwrap();

    maybe_save_svg(&svg, "dark", "two_participants");

    assert!(svg.contains("<svg"));
    assert!(svg.contains("#1a1a2e")); // dark background
}

#[test]
fn test_single_participant() {
    let input = r#"
sequenceDiagram
    participant Alice
"#;
    let options = RenderOptions::default();
    let svg = render_sequence_diagram(input, &options).unwrap();

    maybe_save_svg(&svg, "light", "single_participant");

    assert!(svg.contains("<svg"));
    assert!(svg.contains("Alice"));
}

#[test]
fn test_multiple_messages() {
    let input = r#"
sequenceDiagram
    Alice->>Bob: Message 1
    Bob->>Charlie: Message 2
    Charlie->>Alice: Message 3
    Alice->>Charlie: Message 4
    Charlie-->>Bob: Message 5
"#;
    let options = RenderOptions::default();
    let svg = render_sequence_diagram(input, &options).unwrap();

    maybe_save_svg(&svg, "light", "multiple_messages");

    assert!(svg.contains("Alice"));
    assert!(svg.contains("Bob"));
    assert!(svg.contains("Charlie"));
    assert!(svg.contains("Message 1"));
    assert!(svg.contains("Message 5"));
}

#[test]
fn test_dotted_arrow() {
    let input = r#"
sequenceDiagram
    Alice-->>Bob: Dotted response
"#;
    let options = RenderOptions::default();
    let svg = render_sequence_diagram(input, &options).unwrap();

    maybe_save_svg(&svg, "light", "dotted_arrow");

    assert!(svg.contains("stroke-dasharray")); // dotted line
}

// ============================================
// SVG Output Validation Tests
// ============================================

#[test]
fn test_svg_structure() {
    let input = r#"
sequenceDiagram
    Alice->>Bob: Test
"#;
    let svg = render_sequence_diagram(input, &RenderOptions::default()).unwrap();

    maybe_save_svg(&svg, "light", "svg_structure");

    // Validate SVG structure
    assert!(svg.starts_with("<svg"));
    assert!(svg.ends_with("</svg>"));
    assert!(svg.contains("xmlns=\"http://www.w3.org/2000/svg\""));
    assert!(svg.contains("<rect")); // participant boxes
    assert!(svg.contains("<line")); // lifelines
    assert!(svg.contains("<text")); // labels
}

// ============================================
// Phase 1: Arrow Type Tests
// ============================================

#[test]
fn test_all_arrow_types() {
    // Note: Dotted cross (--x) and dotted async (--)) are not supported by mermaid-parser
    let input = r#"
sequenceDiagram
    participant A
    participant B
    A->>B: Solid with arrowhead
    B-->>A: Dotted with arrowhead
    A->B: Solid without arrowhead
    B-->A: Dotted without arrowhead
    A-xB: Solid with cross
    A-)B: Solid async (open arrow)
"#;
    let options = RenderOptions::default();
    let svg = render_sequence_diagram(input, &options).unwrap();

    maybe_save_svg(&svg, "light", "all_arrow_types");

    assert!(svg.contains("<svg"));
    assert!(svg.contains("Solid with arrowhead"));
    assert!(svg.contains("Dotted with arrowhead"));
}

#[test]
fn test_solid_closed_arrow() {
    let input = r#"
sequenceDiagram
    A->>B: Solid closed arrow
"#;
    let svg = render_sequence_diagram(input, &RenderOptions::default()).unwrap();

    maybe_save_svg(&svg, "light", "arrow_solid_closed");

    // Should have arrow polygon (filled arrowhead)
    assert!(svg.contains("<polygon"));
    // The message arrow line should NOT be dashed (lifelines are dashed, that's ok)
    assert!(svg.contains("Solid closed arrow"));
}

#[test]
fn test_dotted_closed_arrow() {
    let input = r#"
sequenceDiagram
    A-->>B: Dotted closed arrow
"#;
    let svg = render_sequence_diagram(input, &RenderOptions::default()).unwrap();

    maybe_save_svg(&svg, "light", "arrow_dotted_closed");

    // Should have arrow polygon
    assert!(svg.contains("<polygon"));
    // Should be dashed
    assert!(svg.contains("stroke-dasharray"));
}

#[test]
fn test_solid_open_arrow() {
    let input = r#"
sequenceDiagram
    A->B: Solid open (no arrowhead)
"#;
    let svg = render_sequence_diagram(input, &RenderOptions::default()).unwrap();

    maybe_save_svg(&svg, "light", "arrow_solid_open");

    assert!(svg.contains("Solid open"));
}

#[test]
fn test_dotted_open_arrow() {
    let input = r#"
sequenceDiagram
    A-->B: Dotted open (no arrowhead)
"#;
    let svg = render_sequence_diagram(input, &RenderOptions::default()).unwrap();

    maybe_save_svg(&svg, "light", "arrow_dotted_open");

    assert!(svg.contains("Dotted open"));
}

#[test]
fn test_cross_arrow() {
    // Note: Dotted cross (--x) is not supported by mermaid-parser
    let input = r#"
sequenceDiagram
    A-xB: Solid with cross
"#;
    let svg = render_sequence_diagram(input, &RenderOptions::default()).unwrap();

    maybe_save_svg(&svg, "light", "arrow_cross");

    assert!(svg.contains("Solid with cross"));
}

#[test]
fn test_async_arrow() {
    // Note: Dotted async (--)) is not supported by mermaid-parser
    let input = r#"
sequenceDiagram
    A-)B: Solid async
"#;
    let svg = render_sequence_diagram(input, &RenderOptions::default()).unwrap();

    maybe_save_svg(&svg, "light", "arrow_async");

    assert!(svg.contains("Solid async"));
}

#[test]
fn test_bidirectional_solid_arrow() {
    let input = r#"
sequenceDiagram
    A<<->>B: Bidirectional solid
"#;
    let svg = render_sequence_diagram(input, &RenderOptions::default()).unwrap();

    maybe_save_svg(&svg, "light", "arrow_bidirectional_solid");

    assert!(svg.contains("Bidirectional solid"));
    // Should have two arrowheads (polygons)
    let polygon_count = svg.matches("<polygon").count();
    assert!(
        polygon_count >= 2,
        "Bidirectional arrow should have 2 arrowheads, found {}",
        polygon_count
    );
}

#[test]
fn test_bidirectional_dotted_arrow() {
    let input = r#"
sequenceDiagram
    A<<-->>B: Bidirectional dotted
"#;
    let svg = render_sequence_diagram(input, &RenderOptions::default()).unwrap();

    maybe_save_svg(&svg, "light", "arrow_bidirectional_dotted");

    assert!(svg.contains("Bidirectional dotted"));
    assert!(svg.contains("stroke-dasharray")); // dotted line
}

// ============================================
// Phase 2: Self-Message Tests
// ============================================

#[test]
fn test_self_message() {
    let input = r#"
sequenceDiagram
    Alice->>Alice: Self call
    Bob->>Bob: Internal process
"#;
    let svg = render_sequence_diagram(input, &RenderOptions::default()).unwrap();

    maybe_save_svg(&svg, "light", "self_message");

    assert!(svg.contains("Self call"));
    assert!(svg.contains("Internal process"));
}

// ============================================
// Complex Diagram Tests
// ============================================

#[test]
fn test_many_participants() {
    let input = r#"
sequenceDiagram
    participant Client
    participant Gateway
    participant AuthService
    participant UserService
    participant Database
    Client->>Gateway: Request
    Gateway->>AuthService: Validate Token
    AuthService->>Database: Check User
    Database-->>AuthService: User Data
    AuthService-->>Gateway: Valid
    Gateway->>UserService: Get Profile
    UserService->>Database: Query
    Database-->>UserService: Profile Data
    UserService-->>Gateway: Profile
    Gateway-->>Client: Response
"#;
    let svg = render_sequence_diagram(input, &RenderOptions::default()).unwrap();

    maybe_save_svg(&svg, "light", "many_participants");

    assert!(svg.contains("Client"));
    assert!(svg.contains("Gateway"));
    assert!(svg.contains("AuthService"));
    assert!(svg.contains("UserService"));
    assert!(svg.contains("Database"));
}

#[test]
fn test_participant_aliases() {
    let input = r#"
sequenceDiagram
    participant C as Client Browser
    participant S as Backend Server
    participant D as PostgreSQL DB
    C->>S: HTTP Request
    S->>D: SQL Query
    D-->>S: Result Set
    S-->>C: JSON Response
"#;
    let svg = render_sequence_diagram(input, &RenderOptions::default()).unwrap();

    maybe_save_svg(&svg, "light", "participant_aliases");

    // Should show aliases, not actor names
    assert!(svg.contains("Client Browser"));
    assert!(svg.contains("Backend Server"));
    assert!(svg.contains("PostgreSQL DB"));
}

#[test]
fn test_special_characters() {
    let input = r#"
sequenceDiagram
    A->>B: Hello <World> & "Quotes"
    B-->>A: Response with 'apostrophe'
"#;
    let svg = render_sequence_diagram(input, &RenderOptions::default()).unwrap();

    maybe_save_svg(&svg, "light", "special_characters");

    // Special chars should be escaped
    assert!(svg.contains("&lt;World&gt;")); // < and > escaped
    assert!(svg.contains("&amp;")); // & escaped
}

// ============================================
// Dynamic Spacing Tests
// ============================================

#[test]
fn test_long_message_expands_gap() {
    let input = r#"
sequenceDiagram
    Alice->>Bob: This is a very long message that should expand the gap between participants
"#;
    let svg = render_sequence_diagram(input, &RenderOptions::default()).unwrap();

    maybe_save_svg(&svg, "light", "long_message");

    assert!(svg.contains("Alice"));
    assert!(svg.contains("Bob"));
    assert!(svg.contains("This is a very long message"));

    // The SVG width should be larger than default (290 for 2 participants)
    // Extract width from SVG
    let width: u32 = svg
        .split("width=\"")
        .nth(1)
        .and_then(|s| s.split('"').next())
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);

    assert!(
        width > 290,
        "Expected width > 290 for long message, got {}",
        width
    );
}

#[test]
fn test_non_adjacent_long_message() {
    let input = r#"
sequenceDiagram
    participant Alice
    participant Bob
    participant Charlie
    Alice->>Charlie: This message spans Bob and should expand both gaps
"#;
    let svg = render_sequence_diagram(input, &RenderOptions::default()).unwrap();

    maybe_save_svg(&svg, "light", "non_adjacent_long_message");

    assert!(svg.contains("Alice"));
    assert!(svg.contains("Bob"));
    assert!(svg.contains("Charlie"));
    assert!(svg.contains("This message spans Bob"));
}

#[test]
fn test_mixed_message_lengths() {
    let input = r#"
sequenceDiagram
    participant A
    participant B
    participant C
    A->>B: Short
    B->>C: This is a much longer message that needs more space
    C->>A: Hi
"#;
    let svg = render_sequence_diagram(input, &RenderOptions::default()).unwrap();

    maybe_save_svg(&svg, "light", "mixed_message_lengths");

    assert!(svg.contains("Short"));
    assert!(svg.contains("This is a much longer message"));
    assert!(svg.contains("Hi"));
}

// ============================================
// Multi-line Participant Name Tests
// ============================================

#[test]
fn test_multiline_participant_names() {
    let input = r#"
sequenceDiagram
    participant A as Client<br>Browser
    participant B as Backend<br/>Server
    A->>B: HTTP Request
    B-->>A: JSON Response
"#;
    let svg = render_sequence_diagram(input, &RenderOptions::default()).unwrap();

    maybe_save_svg(&svg, "light", "multiline_participant_names");

    // Should contain both lines of each participant name
    assert!(svg.contains("Client"));
    assert!(svg.contains("Browser"));
    assert!(svg.contains("Backend"));
    assert!(svg.contains("Server"));
    // Should NOT contain the raw <br> tags
    assert!(!svg.contains("<br>"));
    assert!(!svg.contains("<br/>"));
}

#[test]
fn test_mixed_single_and_multiline_participants() {
    let input = r#"
sequenceDiagram
    participant A as Alice
    participant B as Backend<br>API<br>Server
    participant C as Charlie
    A->>B: Request
    B->>C: Forward
    C-->>B: Response
    B-->>A: Forward
"#;
    let svg = render_sequence_diagram(input, &RenderOptions::default()).unwrap();

    maybe_save_svg(&svg, "light", "mixed_single_multiline_participants");

    // Single line participant
    assert!(svg.contains("Alice"));
    // Multi-line participant (3 lines)
    assert!(svg.contains("Backend"));
    assert!(svg.contains("API"));
    assert!(svg.contains("Server"));
    // Single line participant
    assert!(svg.contains("Charlie"));
}

#[test]
fn test_uniform_participant_box_width() {
    let input = r#"
sequenceDiagram
    participant A as Short
    participant B as Very Long Participant Name
    participant C as Med
    A->>B: Request
    B->>C: Forward
"#;
    let svg = render_sequence_diagram(input, &RenderOptions::default()).unwrap();

    maybe_save_svg(&svg, "light", "uniform_width_participants");

    // All participant boxes should have the same width (based on longest name)
    // Extract all rect widths
    let width_pattern = regex::Regex::new(r#"<rect[^>]+width="(\d+)"[^>]+height="(\d+)""#).unwrap();
    let widths: Vec<u32> = width_pattern
        .captures_iter(&svg)
        .filter_map(|cap| cap.get(1).and_then(|m| m.as_str().parse().ok()))
        .collect();

    // Should have 6 participant boxes (3 top + 3 bottom)
    assert_eq!(widths.len(), 6, "Expected 6 participant boxes");

    // All widths should be the same
    let first_width = widths[0];
    for w in &widths {
        assert_eq!(
            *w, first_width,
            "All participant boxes should have uniform width"
        );
    }

    // Width should be larger than minimum (80) because of "Very Long Participant Name"
    assert!(
        first_width > 80,
        "Width should be > 80 for long name, got {}",
        first_width
    );
}

// ============================================
// Transparent Background Tests
// ============================================

#[test]
fn test_transparent_background() {
    let input = r#"
sequenceDiagram
    Alice->>Bob: Hello
"#;
    let options = RenderOptions::default().transparent();
    let svg = render_sequence_diagram(input, &options).unwrap();

    maybe_save_svg(&svg, "light", "transparent_background");

    // Should NOT contain background rect
    assert!(
        !svg.contains(r#"<rect width="100%" height="100%""#),
        "Transparent SVG should not contain background rect"
    );
    // Should still be valid SVG
    assert!(svg.contains("<svg"));
    assert!(svg.contains("</svg>"));
}

#[test]
fn test_solid_background_default() {
    let input = r#"
sequenceDiagram
    Alice->>Bob: Hello
"#;
    let options = RenderOptions::default();
    let svg = render_sequence_diagram(input, &options).unwrap();

    // Should contain background rect with light theme color
    assert!(
        svg.contains(r##"<rect width="100%" height="100%" fill="#ffffff""##),
        "Default SVG should contain white background rect"
    );
}

#[test]
fn test_solid_background_dark() {
    let input = r#"
sequenceDiagram
    Alice->>Bob: Hello
"#;
    let options = RenderOptions::with_theme(Theme::Dark);
    let svg = render_sequence_diagram(input, &options).unwrap();

    maybe_save_svg(&svg, "dark", "solid_background");

    // Should contain background rect with dark theme color
    assert!(
        svg.contains(r##"<rect width="100%" height="100%" fill="#1a1a2e""##),
        "Dark SVG should contain dark background rect"
    );
}

#[test]
fn test_transparent_background_dark() {
    let input = r#"
sequenceDiagram
    Alice->>Bob: Hello
"#;
    let options = RenderOptions::with_theme(Theme::Dark).transparent();
    let svg = render_sequence_diagram(input, &options).unwrap();

    maybe_save_svg(&svg, "dark", "transparent_background");

    // Should NOT contain background rect
    assert!(
        !svg.contains(r#"<rect width="100%" height="100%""#),
        "Transparent dark SVG should not contain background rect"
    );
    // Should still have dark theme colors for elements
    assert!(svg.contains("#eaeaea")); // dark theme text/line color
}

// ============================================
// Dark Theme Feature Tests
// ============================================

#[test]
fn test_self_message_dark() {
    let input = r#"
sequenceDiagram
    Alice->>Alice: Self call
    Bob->>Bob: Internal process
"#;
    let options = RenderOptions::with_theme(Theme::Dark);
    let svg = render_sequence_diagram(input, &options).unwrap();

    maybe_save_svg(&svg, "dark", "self_message");

    assert!(svg.contains("Self call"));
    assert!(svg.contains("Internal process"));
    assert!(svg.contains("#1a1a2e")); // dark background
}

#[test]
fn test_multiline_participant_names_dark() {
    let input = r#"
sequenceDiagram
    participant A as Client<br>Browser
    participant B as Backend<br/>Server
    A->>B: HTTP Request
    B-->>A: JSON Response
"#;
    let options = RenderOptions::with_theme(Theme::Dark);
    let svg = render_sequence_diagram(input, &options).unwrap();

    maybe_save_svg(&svg, "dark", "multiline_participant_names");

    // Should contain both lines of each participant name
    assert!(svg.contains("Client"));
    assert!(svg.contains("Browser"));
    assert!(svg.contains("Backend"));
    assert!(svg.contains("Server"));
    // Should have dark theme colors
    assert!(svg.contains("#1a1a2e")); // dark background
    assert!(svg.contains("#16213e")); // dark participant bg
}

#[test]
fn test_all_arrow_types_dark() {
    let input = r#"
sequenceDiagram
    participant A
    participant B
    A->>B: Solid with arrowhead
    B-->>A: Dotted with arrowhead
    A->B: Solid without arrowhead
    B-->A: Dotted without arrowhead
    A-xB: Solid with cross
    A-)B: Solid async (open arrow)
"#;
    let options = RenderOptions::with_theme(Theme::Dark);
    let svg = render_sequence_diagram(input, &options).unwrap();

    maybe_save_svg(&svg, "dark", "all_arrow_types");

    assert!(svg.contains("<svg"));
    assert!(svg.contains("Solid with arrowhead"));
    assert!(svg.contains("Dotted with arrowhead"));
    assert!(svg.contains("#1a1a2e")); // dark background
}

#[test]
fn test_many_participants_dark() {
    let input = r#"
sequenceDiagram
    participant Client
    participant Gateway
    participant AuthService
    participant UserService
    participant Database
    Client->>Gateway: Request
    Gateway->>AuthService: Validate Token
    AuthService->>Database: Check User
    Database-->>AuthService: User Data
    AuthService-->>Gateway: Valid
    Gateway->>UserService: Get Profile
    UserService->>Database: Query
    Database-->>UserService: Profile Data
    UserService-->>Gateway: Profile
    Gateway-->>Client: Response
"#;
    let options = RenderOptions::with_theme(Theme::Dark);
    let svg = render_sequence_diagram(input, &options).unwrap();

    maybe_save_svg(&svg, "dark", "many_participants");

    assert!(svg.contains("Client"));
    assert!(svg.contains("Gateway"));
    assert!(svg.contains("Database"));
    assert!(svg.contains("#1a1a2e")); // dark background
}

// ============================================
// Unicode and International Character Tests
// ============================================

#[test]
fn test_unicode_participant_names() {
    let input = r#"
sequenceDiagram
    participant 用户 as 用户
    participant サーバー as サーバー
    用户->>サーバー: こんにちは
    サーバー-->>用户: 你好
"#;
    let svg = render_sequence_diagram(input, &RenderOptions::default()).unwrap();

    maybe_save_svg(&svg, "light", "unicode_participants");

    assert!(svg.contains("用户"));
    assert!(svg.contains("サーバー"));
    assert!(svg.contains("こんにちは"));
    assert!(svg.contains("你好"));
}

#[test]
fn test_emoji_in_messages() {
    let input = r#"
sequenceDiagram
    Alice->>Bob: Hello! 👋
    Bob-->>Alice: Hi there! 😊
"#;
    let svg = render_sequence_diagram(input, &RenderOptions::default()).unwrap();

    maybe_save_svg(&svg, "light", "emoji_messages");

    assert!(svg.contains("👋"));
    assert!(svg.contains("😊"));
}

// ============================================
// Error Handling Tests
// ============================================

#[test]
fn test_invalid_diagram_type() {
    let input = r#"
flowchart LR
    A --> B
"#;
    let result = render_sequence_diagram(input, &RenderOptions::default());

    assert!(result.is_err());
}

#[test]
fn test_empty_input() {
    let input = "";
    let result = render_sequence_diagram(input, &RenderOptions::default());

    assert!(result.is_err());
}

#[test]
fn test_invalid_syntax() {
    // Note: The mermaid-parser is lenient and ignores unrecognized lines
    // rather than failing. This test verifies that behavior.
    let input = r#"
sequenceDiagram
    this is not valid syntax
"#;
    let result = render_sequence_diagram(input, &RenderOptions::default());

    // Parser ignores invalid lines and produces an empty diagram
    assert!(result.is_ok());
    let svg = result.unwrap();
    assert!(svg.contains("<svg"));
}
