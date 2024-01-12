//! Data about environments not yet validated extracted from providers
use std::collections::HashSet;

use crate::prelude::*;
use serde::{Deserialize, Serialize};

/// Metadata of an environment
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Metadata {
    /// Type of environments
    #[serde(alias = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_type: Option<EnvironmentType>,
    /// Environment name (if it is empty, use the environment folder name)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Environment description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    
    /// Environment languages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub languages: Option<HashSet<String>>,    
    /// Enviromnet Categories 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<HashSet<String>>,
    
    /// Environments directly within the environment
    #[serde(alias = "sub_projects")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Metadata>>,

    /// Command that will be used by default to execute the scripts, the first part is the program, the rest is the arguments
    /// # Examples:
    /// - "bash -c"
    /// - "zsh -c"
    /// - "python -c"
    /// - "./my-own-interpreter"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_interpreter: Option<String>,
    
    /// The environment scripts, the name of the script is the key, and the value is the script, which can be just a string of the command, or an object with the details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scripts: Option<HashMap<String, MetadataScript>>,
    
    /// Relative path to the subproject from the main project directory
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<PathBuf>,
    /// Path to the environment directory
    #[serde(skip)]
    pub source: PathBuf,
}

/// Script to run
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum MetadataScript{
    /// Detailed script
    Struct {
        /// Interpreter that will interpret the script
        interpreter: Option<String>,
        /// The script string 
        script: String,
        /// Relative path where the script will be executed
        directory: Option<PathBuf>,
    },
    /// Only string script
    Scalar(String)
}