use crate::commands::*;
use clap::Subcommand;


#[derive(Debug, Subcommand)]

pub enum CommandGlobal {
    Open(CommandOpen),
    Dir(CommandDir),
    List(CommandList),
    Run(CommandRun)
}
