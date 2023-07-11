use clap::Args;

use crate::{prelude::*, mapper::find_environment};


#[derive(Debug, Args, Clone)]
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