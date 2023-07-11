use std::process;

use clap::Args;

use crate::{prelude::*, mapper::find_environment};


#[derive(Debug, Args, Clone)]
pub struct CommandOpen {
	project: String,
}

impl CommandOpen {
	pub fn execute(self, root : &Path){
		if let Some(env) = find_environment(root, &self.project){
			match env.details {
				EnvironmentDetails::Folder => eprintln!("path is a folder"),
				EnvironmentDetails::Project{ open_command, ..} => {
					let open_command = open_command.unwrap_or_else(||format!("code {}", env.source.display()));
					let argv = shlex::split(&open_command).unwrap();
					process::Command::new(&argv[0]).args(&argv[1..]).current_dir(env.source).spawn().expect("falha ao executar");
				}
				EnvironmentDetails::SubProject { .. } => eprintln!("path is a subproject"),
			}
		}
	}
}