mod environment_type;
mod commands;
mod prelude;
mod enviroment;
mod mapper;
mod metadata;
mod providers;

use clap::{command, Parser, Args};
use commands::*;
use environment_type::EnvironmentType;
use std::{path::PathBuf, env};

#[derive(Args, Debug)]
struct GlobalArgs {
    #[arg(short, long, global = true)]
    root: Option<PathBuf>,
}
#[derive(Parser, Debug)]
#[command(name = "pm")]
#[command(author, version, about, long_about = None)]
struct MyArgs {
    #[command(subcommand)]
    command: CommandGlobal,
    #[command(flatten)]
    global : GlobalArgs
}

fn main() {
    let args = MyArgs::parse();
    
    let root =  args.global.root.or(env::var("DEV_DIR").map(|p|PathBuf::from(&p)).ok()).expect("root and develoonebt dir is undefined");
    match args.command {
        CommandGlobal::List(command) => {
            command.execute(&root);
        },
        CommandGlobal::Run(command) => {
            command.execute(&root);
        }
        CommandGlobal::Dir(command)=> {
            command.execute(&root)
        },
        CommandGlobal::Open(command)=>{
            command.execute(&root)
        }
    }
}
