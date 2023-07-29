use crate::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Metadata {
    #[serde(alias = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_type: Option<EnvironmentType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub languages: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_command: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init_command: Option<String>,
    #[serde(alias = "sub_projects")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Metadata>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_interpreter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scripts: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<PathBuf>,
    #[serde(skip)]
    pub source: PathBuf,
}
