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
	pub fn execute(self, root : &Path) -> Result<()> {
		if let Some(env) = find_environment(root, &self.project){
			return match env.details {
				EnvironmentDetails::Folder => Err(anyhow!("the environment is a folder")),
				EnvironmentDetails::Project{ open_command, ..} => {
					let open_command = open_command.unwrap_or_else(||format!("code {}", env.source.display()));
					println!("running: {open_command}");
					let argv = shlex::split(&open_command).context("invalid command")?;
					process::Command::new(&argv[0]).args(&argv[1..]).current_dir(env.source).spawn()?;
					Ok(())
				}
				EnvironmentDetails::SubProject { .. } => Err(anyhow!("the path is a subproject")),
			}
		}
		Err(anyhow!("environment not found"))
	}
}