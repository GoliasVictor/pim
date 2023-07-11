mod commands;
pub mod enviroment;
mod environment_type;
pub mod mapper;
pub mod metadata;
pub mod providers;
use clap::{command, Parser};
use commands::Commands;
use enviroment::Environment;
pub use environment_type::EnvironmentType;
pub use providers::*;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "pm")]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
    #[arg(short, long, global = true)]
    root: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::List {
            flat: _,
            r#type: _,
            max_depth: _,
        } => {
            let enviroments = mapper::map_directory(&args.root.expect(""));
            list(enviroments, "".to_string())
        }
        Commands::Dir { project } => {
            if let Some(env) = mapper::find_environment(&args.root.unwrap(), &project) {
                println!("{}", env.source.display());
            }
        }
        _ => (),
    }
}
fn list(enviroments: Vec<Environment>, prefix: String) {
    for env in enviroments {
        println!("{}{}", prefix, env.name);
        list(env.children, "-".to_string() + &prefix);
    }
}
