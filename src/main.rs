mod commands;
mod environment;
mod environment_type;
mod mapper;
mod metadata;
mod prelude;
mod providers;
mod terminal;

use clap::Parser;
use commands::*;
use prelude::*;

fn main() -> Result<()>{
    Cli::parse().execute()
}
