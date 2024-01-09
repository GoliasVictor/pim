use clap::Args;
use tabled::{builder::Builder, settings::Style};

use crate::{
    mapper::{find_environment, find_parent_environment},
    prelude::*,
};

/// Run a script
#[derive(Debug, Args, Clone)]
pub struct CommandRun {
    /// Name of script
    script: Option<String>,

    /// Environmnet where is the script
    #[arg(short, long)]
    environmnet: Option<String>,

    /// Parameters added to the end of the script
    #[arg(last=true)]
    parameters: Vec<String>,
}

impl CommandRun {
    /// Execute the command
    pub fn execute(self, root: &Path) -> Result<()> {
        let env = self
            .environmnet
            .clone()
            .map_or_else(
                || find_parent_environment(&std::env::current_dir().expect("Erro")),
                |name| find_environment(root, &name),
            )
            .context("environmnet not found")?;

        if let Some(name_script) = self.script {
            env.run_script(&name_script)?;
        } else {
            self.print_scripts(env)
        }
        Ok(())
    }

    fn print_scripts(self, env: Environment) {
        if env.scripts.is_empty() {
            println!("environmnet has no scripts")
        } else {
            let mut builder = Builder::default();
            builder.set_header(vec!["Name", "Script"]);
            for (k, v) in env.scripts {
                builder.push_record([k, v.value]);
            }
            let table = builder.build().with(Style::sharp()).to_string();
            println!("{}", table);
        }
    }
}
