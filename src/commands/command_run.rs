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

    /// Project where is the script
    #[arg(short, long)]
    project: Option<String>,

    /// Parameters added to the end of the script
    parameters: Vec<String>,
}

impl CommandRun {
    pub fn execute(self, root: &Path) -> Result<()> {
        let env = self
            .project
            .clone()
            .map_or_else(
                || find_parent_environment(&std::env::current_dir().expect("Erro")),
                |name| find_environment(root, &name),
            )
            .context("project not found")?;

        if let Some(name_script) = self.script {
            env.run_script(&name_script)?;
        } else {
            self.print_scripts(env)
        }
        return Ok(());
    }

    pub fn print_scripts(self, env: Environment) {
        if env.scripts.is_empty() {
            println!("project has no scripts")
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
