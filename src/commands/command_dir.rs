use clap::{Args, command};

use crate::{prelude::*, mapper::find_environment};

/// Show the directory of project
#[derive(Debug, Args, Clone)]
#[command(arg_required_else_help = true)]
pub struct CommandDir {
	project: String,
}

impl CommandDir {
	pub fn execute(self, root : &Path){
		if let Some(env) = find_environment(root, &self.project){
			println!("{}", env.source.display())
		}
	}
}