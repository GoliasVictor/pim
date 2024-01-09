#![warn(missing_docs)]

pub mod commands;
pub mod environment;
pub mod environment_type;
pub mod mapper;
pub mod metadata;
pub mod prelude;
pub mod providers;
pub mod terminal;

use clap::Parser;
use commands::*;
use prelude::*;

fn main() -> Result<()>{
    Cli::parse().execute()
}
