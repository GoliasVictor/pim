mod commands;
mod environment;
mod environment_type;
mod mapper;
mod metadata;
mod prelude;
mod providers;

use clap::Parser;
use commands::*;

fn main() {
    Cli::parse().execute();
}
