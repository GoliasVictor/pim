use clap::{Args, command};

use crate::{prelude::*, mapper};

/// Find environments who match the filters  
#[derive(Debug, Args, Clone)]
#[command(arg_required_else_help = true)]
pub struct CommandFind {
	/// Part of the environment name
	#[arg(short, long)]
	pub name: Option<String>,
	/// Environment language 
	#[arg(short, long)]
	pub language: Option<String>,
	///  Environment category 
	#[arg(short, long)]
	pub categories: Option<String>,
	/// Environment type
    #[arg(short, long)]
    pub r#type: Option<EnvironmentType>,
	/// Maximum depth to find 
    #[arg(short, long)]
    pub max_depth: Option<u32>,
}

impl CommandFind {
	/// Execute the command
	pub fn execute(self, root : &Path) -> Result<()>{
        let environments = mapper::map_directory(root).context("fails to find environments")?;
		self.print_flat(environments, 0);
		Ok(())
	}
	/// Check whether to print based on filters
	fn should_print(&self, env: &Environment, depth: u32) -> bool {
		
		let ctype = env.details.enviroment_type();
		let mut should = true;
		if let Some(name) = &self.name {
			should &= env.name.contains(name);
		}
		if let Some(env_type) = self.r#type {
			should &= ctype as u8 <= env_type as u8 
		}
		if let Some(max_depth) = self.max_depth{
			should &= max_depth > depth;
		}
		if let Some(language) =  &self.language{
			if let EnvironmentDetails::Project{languages} = &env.details {
				should &=  languages.contains(&language.to_ascii_lowercase());		
			} else {
				should = false;
			}
		}

		if let Some(category) =  &self.categories{
			should &=  env.categories.contains(&category.to_ascii_lowercase());		
		}

		should
    }

	fn print_flat(&self, environments: Vec<Environment>, depth: u32) {
        for env in environments {
            if self.should_print(&env, depth) {
                println!("{}", env.name);
            }
			self.print_flat(env.children, depth + 1);
        }
    }
}