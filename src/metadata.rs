use std::{path::PathBuf, collections::HashMap};
use serde::{Serialize, Deserialize};
use crate::EnvironmentType;


#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Metadata {
	#[serde(alias="type")]
	pub environment_type : Option<EnvironmentType>,
	pub name : Option<String>,
	pub description : Option<String>, 
	pub languages : Option<Vec<String>>,
	pub open_command : Option<String>,
	pub init_command : Option<String>,
	#[serde(alias="sub_projects")]
	pub children : Option<Vec<Metadata>>, 
	pub scripts : Option<HashMap<String, String>>,
	pub path : Option<PathBuf>,
	#[serde(skip)]
	pub source : PathBuf,
}
