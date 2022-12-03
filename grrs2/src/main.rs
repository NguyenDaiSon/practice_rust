use clap::Parser;
use anyhow::{Context, Result};


fn main() -> Result<()> {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path)
        .with_context(||format!("Failed to read: `{:?}`", &args.path))?;
    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("Match-line: {line}");
        }
    }

    Ok(())
}

#[derive(Debug, Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}
