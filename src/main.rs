mod commands;
mod environment_type;
pub mod metadata;
pub mod source;
pub use source::*;
pub use environment_type::EnvironmentType;
use std::{path::PathBuf, str::FromStr, error::Error};
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

    if let Commands::Open { project } =  args.command {
        let provider = MetadataProvider::new();
    
        println!("{:?}", provider.get_meta(&PathBuf::from_str(&project).expect("")))
    }
    
}
