use anyhow::{Context, Result};
use clap::Parser;

fn main() -> Result<()> {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.filename)
        .with_context(|| format!("Failed to read input file: {:?}", &args.filename))?;

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{line}");
        }
    }

    Ok(())
}

/// Search for a `pattern` in a file and display the lines that contain it.
#[derive(Debug, Parser)]
struct Cli {
    /// The pattern to look for
    #[arg(short = 'p', long = "pattern")]
    pattern: String,

    /// The path to the file to read
    #[arg(short = 'f', long = "filename")]
    filename: std::path::PathBuf,
}
