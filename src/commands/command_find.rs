use clap::{Args, command};

use crate::{prelude::*, mapper};

#[derive(Debug, Args, Clone)]
#[command(arg_required_else_help = true)]
pub struct CommandFind {

	#[arg(short, long)]
	name: Option<String>,

	#[arg(short, long)]
	language: Option<String>,

	#[arg(short, long)]
	categories: Option<String>,

    #[arg(short, long)]
    pub r#type: Option<EnvironmentType>,

    #[arg(short, long)]
    pub max_depth: Option<u32>,

}

impl CommandFind {
	pub fn execute(self, root : &Path) -> Result<()>{
        let environments = mapper::map_directory(&root).context("fails to find environments")?;
		self.print_flat(environments, 0);
		Ok(())
	}

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

		return should;
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