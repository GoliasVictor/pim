use clap::{Args, command};

use crate::{prelude::*, mapper::find_environment};

/// Show the directory of project
#[derive(Debug, Args, Clone)]
#[command(arg_required_else_help = true)]
pub struct CommandDir {
	/// Name of the environment from which the directory will be geted 
	environment: String,
}

impl CommandDir {
	/// Execute the command
	pub fn execute(self, root : &Path){
		if let Some(env) = find_environment(root, &self.environment){
			println!("{}", env.source.display())
		}
	}
}