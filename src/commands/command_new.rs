use std::{
    env::current_dir,
    fs::{self, File}
};

use crate::{prelude::*, providers, terminal::get_command};
use clap::Args;
use directories::ProjectDirs;
use serde::Deserialize;

#[derive(Deserialize)]
struct Template {
    command: String,
}

/// Create a new environment
#[derive(Debug, Args, Clone)]
#[command()]
pub struct CommandNew {
    /// template of environment
    pub template: Option<String>,
    /// Path to environment
    #[arg(short, long)]
    pub path: Option<PathBuf>,
}

impl CommandNew {
    /// Execute the command
    pub fn execute(self) -> Result<()> {
        let Some(proj_dirs) = ProjectDirs::from("", "GoliasVictor", "pm") else {
			return Ok(());
		};
        let config_dir = proj_dirs.config_dir();
        let templates_dir = config_dir.join("templates");
        if !templates_dir.exists() {
            return Err(anyhow!("Template directory not exists"));
        }

        let template_path = templates_dir.join(self.template.unwrap_or("default".to_string()));
        if !template_path.is_file() {
            return Err(anyhow!("Template not exists"));
        }

        let template_file = File::open(template_path).context("could not open template")?;
        let template =
            serde_yaml::from_reader::<File, Template>(template_file).context("invalid template")?;
        let dir = self.path.or_else(|| current_dir().ok()).unwrap();
        if !dir.exists() {
            fs::create_dir_all(&dir).context("could not create directory")?;
        }
        get_command(&template.command, &dir)?
            .status()
            .context(format!("failed to execute comand:{0}", template.command))?;

        let meta_path = dir.join(".meta");
        if !meta_path.exists() {
            let file = File::create(meta_path)?;
            serde_yaml::to_writer(file, &providers::get_meta(&dir)?)?;
        }
        Ok(())
    }
}
