use core::fmt;
use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EnvironmentType {
    Folder = 1,
    Project = 2,
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

