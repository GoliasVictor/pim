mod commands;
use std::path::PathBuf;

use clap::{command, Parser};
use commands::Commands;

#[derive(Parser, Debug)]
#[command(name = "git")]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
    #[arg(short, long, global=true)]
    root : Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    println!("{:?}", args.command)
    
}
