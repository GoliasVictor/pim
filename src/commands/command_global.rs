use crate::commands::*;
use crate::prelude::*;
use clap::{command, Args, Parser};
use clap::{CommandFactory, Subcommand};

#[derive(Subcommand)]

#[allow(missing_docs)]
pub enum Commands {
    Find(CommandFind),
    Open(CommandOpen),
    Dir(CommandDir),
    List(CommandList),
    Run(CommandRun),
    New(CommandNew),
    Info(CommandInfo),
    Completions {
        /// The shell to generate the completions for
        #[arg(value_enum)]
        shell: clap_complete_command::Shell,
    },
}
/// Global arguments 
#[derive(Args, Debug)]
pub struct GlobalArgs {
    /// The root folder with all environments to map and run commands
    #[arg(short, long, env("DEV_DIR"), global = true)]
    pub root: Option<PathBuf>,
}

/// Project Manager 
#[derive(Parser)]
#[command(
    name = "pim",
    id = "pim",
    infer_subcommands = true,
    infer_long_args = true
)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// The command to execute
    #[command(subcommand)]
    pub command: Commands,
    /// Global arguments 
    #[command(flatten)]
    pub global: GlobalArgs,
}

impl Cli {
    /// Execute the program
    pub fn execute(self) -> Result<()> {
        let root = self.global.root.context("root of dev dir is undefined")?;
        match self.command {
            Commands::Find(command) => command.execute(&root)?,
            Commands::List(command) => command.execute(&root)?,
            Commands::Run(command) => command.execute(&root)?,
            Commands::Dir(command) => command.execute(&root),
            Commands::Open(command) => command.execute(&root)?,
            Commands::New(command) => command.execute()?,
            Commands::Info(command) => command.execute(&root)?,
            Commands::Completions { shell } => {
                shell.generate(&mut Cli::command(), &mut std::io::stdout());
            }
        }
        Ok(())
    }
}
