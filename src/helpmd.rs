//! A project manager
#![warn(missing_docs)]


pub mod commands;
pub mod environment;
pub mod environment_type;
pub mod mapper;
pub mod metadata;
pub mod prelude;
pub mod providers;
pub mod terminal;
 
use commands::*;

fn main() {
    println!("{}",clap_markdown::help_markdown::<Cli>());
}
