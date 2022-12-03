use anyhow::{Context, Result};
use clap::Parser;

fn main() -> Result<()> {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.filename)
        .with_context(|| format!("Failed to read input file: {:?}", &args.filename))?;

    find_matches(&args.pattern, &content, &mut std::io::stdout());

    Ok(())
}

#[derive(Debug, Parser)]
struct Cli {
    #[arg(short = 'p', long = "pattern")]
    pattern: String,

    #[arg(short = 'f', long = "filename")]
    filename: std::path::PathBuf,
}

fn find_matches(pattern: &str, content: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line);
        }
    }
}

#[test]
fn testcase01_find_matches() {
    let mut result = Vec::new();
    find_matches("Rust", "Command Line\n Applications\n in \nRust", &mut result);
    assert_eq!(result, b"Rust\n");
}

#[test]
fn testcase02_find_matches() {
    let mut result = Vec::new();
    find_matches("Rust", "Command Line Applications in Rust", &mut result);
    assert_ne!(result, b"Rust\n");
}
