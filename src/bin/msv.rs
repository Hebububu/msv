//! msv - Mermaid SVG Render CLI
//!
//! A command-line tool to render Mermaid diagrams to SVG.

use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;
use colored::Colorize;

use mermaid_svg_render::{render_sequence_diagram, RenderOptions, Theme};

/// Exit codes
const EXIT_SUCCESS: u8 = 0;
const EXIT_GENERAL_ERROR: u8 = 1;
const EXIT_PARSE_ERROR: u8 = 2;

/// Mermaid SVG Render - Convert Mermaid diagrams to SVG
#[derive(Parser, Debug)]
#[command(name = "msv")]
#[command(version, about, long_about = None)]
#[command(arg_required_else_help = true)]
struct Args {
    /// Input .mmd file path
    #[arg(value_name = "INPUT")]
    input: PathBuf,

    /// Output SVG file (default: stdout)
    #[arg(short, long, value_name = "FILE")]
    output: Option<PathBuf>,

    /// Theme: light or dark
    #[arg(short, long, value_name = "THEME", default_value = "light")]
    theme: String,

    /// Use transparent background
    #[arg(long)]
    transparent: bool,
}

fn main() -> ExitCode {
    let args = Args::parse();

    if let Err(code) = run(args) {
        return ExitCode::from(code);
    }

    ExitCode::from(EXIT_SUCCESS)
}

fn run(args: Args) -> Result<(), u8> {
    // Read input file
    let input = read_input(&args.input)?;

    // Parse theme
    let theme = parse_theme(&args.theme)?;

    // Build render options
    let mut options = RenderOptions::with_theme(theme);
    if args.transparent {
        options = options.transparent();
    }

    // Render the diagram
    let svg = render_diagram(&input, &options)?;

    // Write output
    write_output(&svg, args.output.as_ref())?;

    Ok(())
}

fn read_input(path: &PathBuf) -> Result<String, u8> {
    fs::read_to_string(path).map_err(|e| {
        eprintln!(
            "{} Failed to read '{}': {}",
            "error:".red().bold(),
            path.display(),
            e
        );
        EXIT_GENERAL_ERROR
    })
}

fn parse_theme(theme_str: &str) -> Result<Theme, u8> {
    match theme_str.to_lowercase().as_str() {
        "light" => Ok(Theme::Light),
        "dark" => Ok(Theme::Dark),
        _ => {
            eprintln!(
                "{} Invalid theme '{}'. Use 'light' or 'dark'.",
                "error:".red().bold(),
                theme_str
            );
            Err(EXIT_GENERAL_ERROR)
        }
    }
}

fn render_diagram(input: &str, options: &RenderOptions) -> Result<String, u8> {
    render_sequence_diagram(input, options).map_err(|e| {
        let error_msg = e.to_string();
        if error_msg.contains("parse") || error_msg.contains("Parse") {
            eprintln!("{} {}", "parse error:".red().bold(), error_msg);
            EXIT_PARSE_ERROR
        } else {
            eprintln!("{} {}", "error:".red().bold(), error_msg);
            EXIT_GENERAL_ERROR
        }
    })
}

fn write_output(svg: &str, output: Option<&PathBuf>) -> Result<(), u8> {
    match output {
        Some(path) => {
            fs::write(path, svg).map_err(|e| {
                eprintln!(
                    "{} Failed to write '{}': {}",
                    "error:".red().bold(),
                    path.display(),
                    e
                );
                EXIT_GENERAL_ERROR
            })?;
            eprintln!(
                "{} Wrote SVG to '{}'",
                "success:".green().bold(),
                path.display()
            );
        }
        None => {
            io::stdout().write_all(svg.as_bytes()).map_err(|e| {
                eprintln!("{} Failed to write to stdout: {}", "error:".red().bold(), e);
                EXIT_GENERAL_ERROR
            })?;
        }
    }
    Ok(())
}
