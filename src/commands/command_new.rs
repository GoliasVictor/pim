use std::{
    env::current_dir,
    fs::{self, File},
    process,
};

use crate::{mapper::find_environment, prelude::*, providers};
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
    template: Option<String>,
    /// Path to environment
    #[arg(short, long)]
    path: Option<PathBuf>,
}

impl CommandNew {
    pub fn execute(self) {
        if let Some(proj_dirs) = ProjectDirs::from("", "GoliasVictor", "pm") {
            let config_dir = proj_dirs.config_dir();
            let templates_dir = config_dir.join("templates");
            if !templates_dir.exists() {
                panic!("Template directory not exists");
            }

            let template_path = templates_dir.join(self.template.unwrap_or("default".to_string()));
            if !template_path.is_file() {
                panic!("Template not exists ");
            }

            let template = File::open(template_path)
                .or(Err("Could not open template"))
                .and_then(|file| {
                    serde_yaml::from_reader::<File, Template>(file).or(Err("invalid template"))
                })
                .unwrap();
            let argv = shlex::split(&template.command).unwrap();
            let dir = self.path.or_else(|| current_dir().ok()).unwrap();
            if !dir.exists() {
                fs::create_dir_all(&dir).expect("Could not create directory");
            }
            let _ = process::Command::new(&argv[0])
                .args(&argv[1..])
                .current_dir(&dir)
                .status();

            let meta_path = dir.join(".meta");
            if !meta_path.exists() {
                if let Ok(file) = File::create(meta_path) {
                    let _ = serde_yaml::to_writer(file, &providers::get_meta(&dir));
                }
            }
        };
    }
}
