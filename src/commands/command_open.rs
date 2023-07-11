use std::process;

use clap::Args;

use crate::{prelude::*, mapper::find_environment};

/// Open the project
#[derive(Debug, Args, Clone)]
#[command(arg_required_else_help = true)]
pub struct CommandOpen {
	/// Project to open
	project: String,
}

impl CommandOpen {
	pub fn execute(self, root : &Path){
		if let Some(env) = find_environment(root, &self.project){
			match env.details {
				EnvironmentDetails::Folder => eprintln!("the path is a folder"),
				EnvironmentDetails::Project{ open_command, ..} => {
					let open_command = open_command.unwrap_or_else(||format!("code {}", env.source.display()));
					let argv = shlex::split(&open_command).unwrap();
					process::Command::new(&argv[0]).args(&argv[1..]).current_dir(env.source).spawn().expect("falha ao executar");
				}
				
				EnvironmentDetails::SubProject { .. } => eprintln!("the path is a subproject"),
			}
		}
	}
}