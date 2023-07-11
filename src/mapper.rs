use crate::prelude::*;
use crate::providers;
use std::fs::{read_dir, DirEntry};


pub fn get_dirs(path: &Path) -> impl IntoIterator<Item = DirEntry> {
    return read_dir(path)
        .unwrap()
        .filter_map(Result::ok)
        .filter(|entry| entry.metadata().unwrap().is_dir());
}
pub fn map_directory(path: &Path) -> Vec<Environment> {
    let mut enviroments: Vec<Environment> = vec![];

    for dir in get_dirs(path) {
        if let Some(mut env) = providers::get_environment(&dir.path()) {
            if let EnvironmentDetails::Folder = env.details {
                env.children = map_directory(&env.source);
            }
            enviroments.push(env);
        }
    }
    enviroments.sort_by(|a, b| a.name.cmp(&b.name));
    return enviroments;
}

pub fn find_environment(path: &Path, name: &str) -> Option<Environment> {
    let name = name.to_lowercase();
    let mut folders: Vec<Environment> = vec![];
    for dir in get_dirs(path) {
        if let Some(mut env) = providers::get_environment(&dir.path()){
            if let EnvironmentDetails::Folder = env.details {
                env.children = map_directory(&env.source);
            }

            if env.name.to_lowercase() == name {
                return Some(env);
            }
            folders.push(env)
        }
    };
    for folder in folders {
        if let Some(env) = find_environment(&folder.source, &name) {
            return Some(env);
        }
    }

    return None;
}

pub fn find_parent_environment(path: &PathBuf) -> Option<Environment> {
    let mut path = path.clone();
    loop {
        if let Some(env) = providers::get_environment(&path){
            return Some(env);
        };
        if !path.pop(){
            return None;
        };
    }
}