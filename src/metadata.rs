use std::collections::HashSet;

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
    pub languages: Option<HashSet<String>>,    
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<HashSet<String>>,
    
    #[serde(alias = "sub_projects")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Metadata>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_interpreter: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scripts: Option<HashMap<String, MetadataScript>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<PathBuf>,
    
    #[serde(skip)]
    pub source: PathBuf,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum MetadataScript{
    Struct {
        interpreter: Option<String>,
        script: String,
        directory: Option<PathBuf>,
    },
    Scalar(String)
}