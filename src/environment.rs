use crate::prelude::*;

#[derive(Debug)]
pub enum EnvironmentDetails {
    Folder,
    Project {
        languages: Vec<String>,
        open_command: Option<String>,
        init_command: Option<String>,
    },
    SubProject {
        path: PathBuf,
    },
}

impl EnvironmentDetails {
    pub fn enviroment_type(&self) -> EnvironmentType{
        return match self { 
            EnvironmentDetails::Folder => EnvironmentType::Folder,
            EnvironmentDetails::Project {..} => EnvironmentType::Project,
            EnvironmentDetails::SubProject {..} => EnvironmentType::SubProject  
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
    pub scripts: HashMap<String, String>,
}

impl Environment {
    pub fn from_metadata(meta: Metadata) -> Result<Self> {
        return Ok(Self {
            name: meta.name.ok_or(anyhow!("name undefined"))?,
            description: meta.description,
            source: meta.source,
            script_interpreter: meta.script_interpreter,
            scripts: meta.scripts.unwrap_or(HashMap::new()),
            details: if let Some(environment_type) = meta.environment_type {
                match environment_type {
                    EnvironmentType::Folder => EnvironmentDetails::Folder,
                    EnvironmentType::Project => EnvironmentDetails::Project {
                        languages: meta.languages.unwrap_or(vec![]),
                        open_command: meta.open_command,
                        init_command: meta.init_command,
                    },
                    EnvironmentType::SubProject => EnvironmentDetails::SubProject {
                        path: meta.path.ok_or(anyhow!("undefined path of subproject"))?,
                    },
                }
            } else {
                EnvironmentDetails::Project {
                    languages: meta.languages.unwrap_or(vec![]),
                    open_command: meta.open_command,
                    init_command: meta.init_command,
                }
            },
            children: meta.children.map_or(vec![], |sps| {
                sps.into_iter()
                    .filter_map(|m| Environment::from_metadata(m).ok())
                    .collect()
            }),
        });
    }
}
