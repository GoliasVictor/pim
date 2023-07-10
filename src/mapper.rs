use std::{path::{Path}, fs::{read_dir}};

use crate::{providers, enviroment::{Environment, EnvironmentDetails}, EnvironmentType};

fn name_else_filename(opname : Option<String>, path : &Path) -> Option<String> {
	println!("{:?}",path);
	if let Some(name) = opname {
		if !name.trim().is_empty(){
			return Some(name);
		}
	}
	return path.file_name().and_then(|s|s.to_str()).map(|s| s.to_string());
	
}
pub fn map_directory(path : &Path) -> Vec<Environment> { 
	let mut enviroments : Vec<Environment> =  vec![];
	
	let dirs = read_dir(path)
		.unwrap()
		.filter_map(Result::ok)
		.filter(|entry| {
			entry.metadata().unwrap().is_dir()
		});
	for dir in dirs {

		if let Some(mut meta) = providers::get_meta(&dir.path()){
			meta.source = dir.path();
			//meta.name = name_else_filename(meta.name.clone(), &dir.path());
			if let Some(EnvironmentType::Project) = meta.environment_type {
				if let Some(children) = &mut meta.children {
					for child in children.into_iter() {
						if let Some(child_path) = child.path.as_ref().and_then(|p| meta.source.join(&p).canonicalize().ok()) {
							child.name = name_else_filename(child.name.clone(), &child_path);
						} 
						child.environment_type = Some(EnvironmentType::SubProject);
					} 
				}
				println!("{:#?}",meta);
			}
			if let Some(mut env) = Environment::from_metadata(meta).ok() {
				if let EnvironmentDetails::Folder = env.details {
					env.children = map_directory(&env.source);
				}
				enviroments.push(env);
			}
		}
	}
	return enviroments;
}