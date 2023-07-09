use std::{path::PathBuf, collections::HashMap};
use serde::{Serialize, Deserialize};
use crate::EnvironmentType;


#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
	#[serde(alias="type")]
	pub environment_type : EnvironmentType,
	pub name : Option<String>,
	pub description : Option<String>, 
	pub languages : Option<Vec<String>>,
	pub open_command : Option<String>,
	pub init_command : Option<String>,
	pub sub_projects : Option<Vec<Metadata>>, 
	pub scripts : Option<HashMap<String, String>>,
	#[serde(skip)]
	pub path : PathBuf,
	#[serde(skip)]
	pub source : PathBuf,
}


