use std::{path::PathBuf, collections::HashMap};

use crate::{metadata::{Metadata}, EnvironmentType};

pub enum EnvironmentDetails {
	Folder,
	Project {
		languages : Vec<String>,
		open_command : Option<String>,
		init_command : Option<String>,
		scripts : HashMap<String, String>,
	},
	SubProject { 
		path : PathBuf
	}
}
pub struct Environment {
	pub name : String,
	pub description : Option<String>, 
	pub source : PathBuf,
	pub children : Vec<Environment>,
	pub details : EnvironmentDetails
}

impl Environment {
	pub fn from_metadata(meta : Metadata) -> Self {
		Self {
			name : meta.name.expect("Name undefined"),
			description : meta.description,
			source : meta.source,
			details : match meta.environment_type {
				EnvironmentType::Folder => {
					EnvironmentDetails::Folder
				},
				EnvironmentType::Project => {
					EnvironmentDetails::Project { 
						languages: meta.languages.unwrap_or(vec![]), 
						open_command: meta.open_command, 
						init_command: meta.init_command,
						scripts: meta.scripts.unwrap_or(HashMap::new())
					}
				}, 
				EnvironmentType::SubProject => {
					EnvironmentDetails::SubProject { 
						path: meta.path   
					}
				}
			},
			children : meta.sub_projects.map_or(vec![], |sps| sps.into_iter().map(Environment::from_metadata).collect()) 
		}
	}
}
