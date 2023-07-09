use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnvironmentType {
	Folder,
	Project,
	SubProject
}