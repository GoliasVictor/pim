use clap::Args;

use crate::prelude::*;


#[derive(Debug, Args, Clone)]
pub struct CommandRun {
	script : Option<String>, 
	#[arg(short, long)]
	project: Option<String>,
}

impl CommandRun {
	pub fn execute(self, root : &Path){

	}
}