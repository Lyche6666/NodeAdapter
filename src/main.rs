// src/main.rs
/*
 * Main executable for NodeAdapter
 */

use clap::Parser;
use nodeadapter::{Result, run};

#[derive(Parser)]
#[command(version, about = "NodeAdapter - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
    
    /// Input file path
    #[arg(short, long)]
    input: Option<String>,
    
    /// Output file path
    #[arg(short, long)]
    output: Option<String>,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose, args.input, args.output)
}
