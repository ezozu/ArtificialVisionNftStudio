// src/main.rs
/*
 * Main executable for ArtificialVisionNftStudio
 */

use clap::Parser;
use artificialvisionnftstudio::{Result, run};

#[derive(Parser)]
#[command(version, about = "ArtificialVisionNftStudio - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
