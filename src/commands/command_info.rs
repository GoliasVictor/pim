use clap::{command, Args, ValueEnum};
use serde::{Deserialize, Serialize};
use std::fs;

use crate::{
    mapper::{find_environment, find_parent_environment},
    prelude::*,
};

/// Show the directory of project
#[derive(Debug, Args, Clone)]
#[command(arg_required_else_help = true)]
pub struct CommandInfo {
    property: MetadataProperty,
    #[arg(short, long)]
    environment: Option<String>,
    #[arg(short, long)]
    path: Option<PathBuf>,
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[allow(non_camel_case_types)]
pub enum MetadataProperty {
    environment_type,
    languages,
    path,
    description,
    script_interpreter,
    name,
    directory,
}
impl MetadataProperty {
    pub fn check_undefined<T, E, F>(&self, value: T) -> Result<E>
    where
        T: Context<E, F>,
    {
        value.context(format!("{:?} undefined", self))
    }
    pub fn get_data(&self, metadata: &Metadata) -> Result<String> {
        Ok(match self {
            Self::environment_type => {
                format!("{:?}", self.check_undefined(metadata.environment_type)?)
            }
            Self::languages => self.check_undefined(metadata.languages.clone())?.join(" "),
            Self::path => self
                .check_undefined(metadata.path.clone())?
                .to_string_lossy()
                .to_string(),
            Self::directory => metadata.source.to_string_lossy().to_string(),
            Self::description => self.check_undefined(metadata.description.clone())?,
            Self::script_interpreter => {
                self.check_undefined(metadata.script_interpreter.clone())?
            }
            Self::name => self.check_undefined(metadata.name.clone())?,
        })
    }
}

impl CommandInfo {
    pub fn execute(self, root: &Path) -> Result<()> {
        let env = if let Some(environment) = self.environment {
            find_environment(root, &environment).context("environment not found")?
        } else if let Some(mut path) = self.path {
            path = fs::canonicalize(path).context("invalid path")?;
            find_parent_environment(&path).context("environment not found")?
        } else {
            let path = std::env::current_dir().context("failed to get current directory")?;
            find_parent_environment(&path).context("actual directory isn't a environment")?
        };

        println!("{}", self.property.get_data(&env.to_metadata())?);
        Ok(())
    }
}
