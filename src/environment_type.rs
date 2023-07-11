use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EnvironmentType {
    Folder = 1,
    Project = 2,
    SubProject = 3,
}

