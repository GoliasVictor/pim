use crate::commands::*;
use clap::{command, Args, Parser};
use clap::{CommandFactory, Subcommand};
use std::path::PathBuf;

#[derive(Subcommand)]

pub enum Commands {
    Open(CommandOpen),
    Dir(CommandDir),
    List(CommandList),
    Run(CommandRun),
    New(CommandNew),
    /// Generate shell completions
    Completions {
        /// The shell to generate the completions for
        #[arg(value_enum)]
        shell: clap_complete_command::Shell,
    },
}

#[derive(Args, Debug)]
pub struct GlobalArgs {
    #[arg(short, long, env("DEV_DIR"), global = true)]
    pub root: Option<PathBuf>,
}

#[derive(Parser)]
#[command(name = "pm", id="pm",  infer_subcommands=true)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
    #[command(flatten)]
    pub global: GlobalArgs,
}

impl Cli {
    pub fn execute(self) {
        let root = self.global.root.expect("root dir is undefined");
        match self.command {
            Commands::List(command) => {
                command.execute(&root);
            }
            Commands::Run(command) => {
                command.execute(&root);
            }
            Commands::Dir(command) => command.execute(&root),
            Commands::Open(command) => command.execute(&root),
            Commands::New(command) => command.execute(),
            Commands::Completions { shell } => {
                shell.generate(&mut Cli::command(), &mut std::io::stdout());
            }
        }
    }
}
