mod commands_args;
mod environment_type;
mod commands;
pub mod enviroment;
pub mod mapper;
pub mod metadata;
pub mod providers;
use clap::{command, Parser};
use commands_args::CommandsArgs;
use enviroment::Environment;
pub use environment_type::EnvironmentType;
pub use providers::*;
use std::{path::PathBuf, env};

#[derive(Parser, Debug)]
#[command(name = "pm")]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: CommandsArgs,
    #[arg(short, long, global = true)]
    root: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();
    
    let root =  args.root.or(env::var("DEV_DIR").map(|p|PathBuf::from(&p)).ok()).expect("root and develoonebt dir is undefined");
    match args.command {
        CommandsArgs::List {folder, flat, r#type, max_depth} => {
            commands::command_list(&root,folder,flat, r#type, max_depth);
        }
        CommandsArgs::Dir { project } => {
            if let Some(env) = mapper::find_environment(&root, &project) {
                println!("{}", env.source.display());
            }
        }
        _ => (),
    }
}
