use crate::{
    metadata::MetadataScript::{Scalar, Struct, self},
    prelude::*,
    terminal::{default_shell_interpreter, Script},
};

#[derive(Debug)]
pub enum EnvironmentDetails {
    Folder,
    Project { languages: Vec<String> },
    SubProject { path: PathBuf },
}

impl EnvironmentDetails {
    pub fn enviroment_type(&self) -> EnvironmentType {
        return match self {
            EnvironmentDetails::Folder => EnvironmentType::Folder,
            EnvironmentDetails::Project { .. } => EnvironmentType::Project,
            EnvironmentDetails::SubProject { .. } => EnvironmentType::SubProject,
        };
    }
}

#[derive(Debug)]
pub struct Environment {
    pub name: String,
    pub description: Option<String>,
    pub source: PathBuf,
    pub children: Vec<Environment>,
    pub details: EnvironmentDetails,
    pub script_interpreter: Option<String>,
    pub scripts: HashMap<String, Script>,
}

impl Environment {
    pub fn from_metadata(meta: Metadata) -> Result<Self> {
        return Ok(Self {
            name: meta.name.clone().ok_or(anyhow!("name undefined"))?,
            description: meta.description.clone(),
            source: meta.source.clone(),
            script_interpreter: meta.script_interpreter.clone(),
            scripts: meta
                .scripts
                .clone()
                .unwrap_or(HashMap::new())
                .into_iter()
                .map(|(k, v)| (k, metadatascript_to_script(v, &meta)))
                .collect(),
            details: if let Some(environment_type) = meta.environment_type {
                match environment_type {
                    EnvironmentType::Folder => EnvironmentDetails::Folder,
                    EnvironmentType::Project => EnvironmentDetails::Project {
                        languages: meta.languages.unwrap_or(vec![]),
                    },
                    EnvironmentType::SubProject => EnvironmentDetails::SubProject {
                        path: meta.path.context("undefined path of subproject")?,
                    },
                }
            } else {
                EnvironmentDetails::Project {
                    languages: meta.languages.unwrap_or(vec![]),
                }
            },
            children: meta.children.map_or(vec![], |sps| {
                sps.into_iter()
                    .filter_map(|m| Environment::from_metadata(m).ok())
                    .collect()
            }),
        });
    }

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
                open_command.to_process()?.status()?;
            }
            EnvironmentDetails::SubProject { .. } => Err(anyhow!("the path is a subproject"))?,
        }
        Ok(())
    }
    pub fn run_script(self, name_script: &str) -> Result<()> {
        self.scripts
            .get(name_script)
            .context("script not found")?
            .to_process()?
            .status()
            .context("failed to execute script")?;
        Ok(())
    }
}

fn metadatascript_to_script(v: MetadataScript, meta: &Metadata) -> Script {
    match v {
        Struct {
            script,
            interpreter,
            directory,
        } => Script {
            value: script.clone(),
            interpreter: interpreter
                .clone()
                .or(meta.script_interpreter.clone())
                .unwrap_or_else(default_shell_interpreter),
            directory: meta.source.join(directory.clone().unwrap_or(PathBuf::from("."))),
        }
        .clone(),
        Scalar(value) => Script {
            interpreter: default_shell_interpreter(),
            value: value.clone(),
            directory: meta.source.clone(),
        },
    }
}
