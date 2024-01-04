use clap::{command, Args, ValueEnum, Subcommand};
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
    #[command(subcommand)]
    command: InfoSubcommands,
    #[arg(short, long, global=true)]
    environment: Option<String>,
    #[arg(short, long, global=true)]
    path: Option<PathBuf>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum InfoSubcommands {
    IsEnv,
    Property {
        property : MetadataProperty
    },
}


#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum MetadataProperty {
    EnvironmentType,
    Languages,
    Path,
    Description,
    ScriptInterpreter,
    Name,
    Directory,
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
            Self::EnvironmentType => {
                self.check_undefined(metadata.environment_type)?.to_string()
            }
            Self::Languages => self.check_undefined(metadata.languages.clone())?.into_iter().collect::<Vec<String>>().join(" "),
            Self::Path => self
                .check_undefined(metadata.path.clone())?
                .to_string_lossy()
                .to_string(),
            Self::Directory => metadata.source.to_string_lossy().to_string(),
            Self::Description => self.check_undefined(metadata.description.clone())?,
            Self::ScriptInterpreter => {
                self.check_undefined(metadata.script_interpreter.clone())?
            }
            Self::Name => self.check_undefined(metadata.name.clone())?,
        })
    }
}

impl CommandInfo {
    pub fn execute(self, root: &Path) -> Result<()> {
        let env = if let Some(environment) = self.environment {
            find_environment(root, &environment).context("environment not found")
        } else if let Some(mut path) = self.path {
            path = fs::canonicalize(path).context("invalid path")?;
            find_parent_environment(&path).context("environment not found")
        } else {
            let path = std::env::current_dir().context("failed to get current directory")?;
            find_parent_environment(&path).context("actual directory isn't a environment")
        };
        match self.command {
            InfoSubcommands::IsEnv => {
                println!("{}", env.is_ok());
            },
            InfoSubcommands::Property { property } => {
                println!("{}", property.get_data(&env?.to_metadata())?);
            },
        }
        
        Ok(())
    }
}
