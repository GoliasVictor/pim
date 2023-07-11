use crate::{prelude::*, commands::*};
use clap::{Subcommand};



#[derive(Debug, Subcommand)]
pub enum CommandGlobal {
    #[command(arg_required_else_help = true)]
    Open(CommandOpen),
    #[command(arg_required_else_help = true)]
    New { project: String },
    #[command(arg_required_else_help = true)]
    Dir(CommandDir),
    List(CommandList),
    Run(CommandRun)
}
