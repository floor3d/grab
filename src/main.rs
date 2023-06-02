use clap::Parser;
use anyhow::{Context, Result};
use ansi_term::Style;
use std::fs::File;
use std::io::{BufRead, BufReader};
#[derive(Parser)]
struct CLArgs {
    pattern: String,
    path: std::path::PathBuf
}
fn main() -> Result<()> {
    let clargs = CLArgs::parse();
    let file = File::open(&clargs.path)
                         .with_context(|| format!("failed reading {}", &clargs.path.display()))?;
    let reader = BufReader::new(file);
    for (line_num, line) in reader.lines().enumerate() {
        if let Ok(line) = line {
            if line.contains(&clargs.pattern) {
                let num_str = &(line_num + 1).to_string();
                println!("[{}]: {}", Style::new().bold().paint(num_str), line);
            }
        }
    }
    Ok(())
}
