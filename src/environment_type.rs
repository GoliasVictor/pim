//! Type of environment
use core::fmt;
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
/// Type of environment, directly implies the purpose of existence and use
#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EnvironmentType {
    /// Folder: an environment that groups other environments
    Folder = 1,
    /// Project: an environment that runs and has source code and can be used alone
    Project = 2,
    /// Subproject: a part within a project, a module, a layer, a library, etc. an internal separation within a project
    SubProject = 3,
}

impl fmt::Display for EnvironmentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let variant_str = match self {
            EnvironmentType::Folder => "folder",
            EnvironmentType::Project => "project",
            EnvironmentType::SubProject => "sub_project",
        };
        write!(f, "{}", variant_str)
    }
}

