use crate::{prelude::*, commands::*};
use clap::{Subcommand, Args};



#[derive(Debug, Subcommand)]
pub enum CommandGlobal {
    #[command(arg_required_else_help = true)]
    Open { project: String },
    #[command(arg_required_else_help = true)]
    New { project: String },
    #[command(arg_required_else_help = true)]
    Dir { project: String },
    List(CommandList),
    Run(CommandRun)
}
