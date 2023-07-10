mod commands;
mod environment_type;
pub mod metadata;
pub mod providers;
pub mod mapper;
pub mod enviroment;
use enviroment::Environment;
pub use providers::*;
pub use environment_type::EnvironmentType;
use std::{path::PathBuf};
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

    if let Commands::List { flat: _, r#type: _, max_depth: _ } = args.command {
        let enviroments = mapper::map_directory(&args.root.expect(""));
        list(enviroments, "".to_string())
    }   
}
fn list(enviroments : Vec<Environment>, prefix : String){
    for env in enviroments {
        list(env.children,"-".to_string() + &prefix );
    }
}