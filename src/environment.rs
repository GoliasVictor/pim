//! Environment structs and related functions
use std::collections::HashSet;
use crate::{
    metadata::MetadataScript::{self, Scalar, Struct},
    prelude::*,
    terminal::{default_shell_interpreter, Script},
};

/// Environment details for each type
#[derive(Debug)]
pub enum EnvironmentDetails {
    /// Folder: an environment that groups other environments
    Folder,
    /// Project: an environment that runs and has source code and can be used alone
    Project {
        /// The languages used in the project
        languages: HashSet<String>,
    },
    /// Subproject: a part within a project, a module, a layer, a library, etc. an internal separation within a project
    SubProject {
        /// Relative path from the main project to the subproject folder
        path: PathBuf,
    },
}

impl EnvironmentDetails {
    /// return the struct `EnvirnomentType` based of the details
    pub fn enviroment_type(&self) -> EnvironmentType {
        match self {
            EnvironmentDetails::Folder => EnvironmentType::Folder,
            EnvironmentDetails::Project { .. } => EnvironmentType::Project,
            EnvironmentDetails::SubProject { .. } => EnvironmentType::SubProject,
        }
    }
}
/// Environment, a generalization of a folder where there is source code
///
/// It can either be something that just stores other environments within it (folder),
/// or the entire source code of a project (project) and smaller parts of a project (subproject).
#[derive(Debug)]
pub struct Environment {
    /// Environment name
    pub name: String,
    /// Description of what is in this environment or what it is for or whatever is necessary to describe
    pub description: Option<String>,
    /// Folder where the environment is
    pub source: PathBuf,
    /// Environment categories
    pub categories: HashSet<String>,
    /// Environments directly contained by the environment
    pub children: Vec<Environment>,
    /// Environment details for the type of environment this environment is
    pub details: EnvironmentDetails,
    /// Default script interpreter that will be used to execute the scripts
    pub script_interpreter: Option<String>,
    /// Dictionary as the name of the script as the key, and the value is the scripts that can be executed from the environment
    pub scripts: HashMap<String, Script>,
}

impl Environment {
    /// If is the metadata is valid,return the
    pub fn from_metadata(meta: Metadata) -> Result<Self> {
        Ok(Self {
            scripts: meta
                .scripts
                .clone()
                .unwrap_or(HashMap::new())
                .into_iter()
                .map(|(k, v)| (k, metadatascript_to_script(v, &meta)))
                .collect(),
            name: meta.name.ok_or(anyhow!("name undefined"))?,
            description: meta.description,
            source: meta.source,
            script_interpreter: meta.script_interpreter,
            categories: meta.categories.unwrap_or(HashSet::default()),
            details: if let Some(environment_type) = meta.environment_type {
                match environment_type {
                    EnvironmentType::Folder => EnvironmentDetails::Folder,
                    EnvironmentType::Project => EnvironmentDetails::Project {
                        languages: meta.languages.unwrap_or(HashSet::new()),
                    },
                    EnvironmentType::SubProject => EnvironmentDetails::SubProject {
                        path: meta.path.context("undefined path of subproject")?,
                    },
                }
            } else {
                EnvironmentDetails::Project {
                    languages: meta.languages.unwrap_or(HashSet::new()),
                }
            },
            children: meta.children.map_or(vec![], |sps| {
                sps.into_iter()
                    .filter_map(|m| Environment::from_metadata(m).ok())
                    .collect()
            }),
        })
    }
    /// If the open script exists, execute it, if not, execute the default open script, if is a `project`
    pub fn open(self: Environment) -> Result<()> {
        match self.details {
            EnvironmentDetails::Folder => Err(anyhow!("the environment is a folder"))?,
            EnvironmentDetails::Project { .. } => {
                let open_command = self
                    .scripts
                    .get("open")
                    .map(|s| s.to_owned())
                    .unwrap_or_else(|| {
                        Script::new(None, format!("code {}", self.source.display()), self.source)
                    });
                println!("running: {}", &open_command.value);
                open_command.to_command()?.status()?;
            }
            EnvironmentDetails::SubProject { .. } => Err(anyhow!("the path is a subproject"))?,
        }
        Ok(())
    }
    /// If a script named `name_script` exists, execute the script
    pub fn run_script(self, name_script: &str) -> Result<()> {
        self.scripts
            .get(name_script)
            .context("script not found")?
            .to_command()?
            .status()?;
        Ok(())
    }
}

impl From<Environment> for Metadata {
    fn from(env: Environment) -> Metadata {
        let environment_type = Some(env.details.enviroment_type());
        let (path, languages) = match env.details {
            EnvironmentDetails::Project { languages } => (None, Some(languages)),
            EnvironmentDetails::SubProject { path } => (Some(path), None),
            EnvironmentDetails::Folder => (None, None),
        };
        Metadata {
            environment_type,
            languages,
            path,
            name: Some(env.name),
            description: env.description,
            categories: Some(env.categories),
            source: env.source,
            script_interpreter: env.script_interpreter,
            children: Some(
                env.children
                    .into_iter()
                    .map(Metadata::from)
                    .collect(),
            ),
            scripts: Some(
                env.scripts
                    .into_iter()
                    .map(|(str, script)| (str, MetadataScript::from(script)))
                    .collect(),
            ),
        }
    }
} 
/// convert a script of metadata to a script 
fn metadatascript_to_script(v: MetadataScript, meta: &Metadata) -> Script {
    match v {
        Struct {
            script,
            interpreter,
            directory,
        } => Script {
            value: script,
            interpreter: interpreter
                .or(meta.script_interpreter.clone())
                .unwrap_or_else(default_shell_interpreter),
            directory: meta.source.join(directory.unwrap_or(PathBuf::from("."))),
        },
        Scalar(value) => Script {
            interpreter: default_shell_interpreter(),
            value,
            directory: meta.source.clone(),
        },
    }
}
