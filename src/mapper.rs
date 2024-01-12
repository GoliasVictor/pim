//! Functions to search, map and extract environments from the file system
use crate::prelude::*;
use crate::providers;
use std::fs::{read_dir, DirEntry};

/// If is a valid path return a iterator of all directoris in a path
pub fn get_dirs(path: &Path) -> Result<impl IntoIterator<Item = DirEntry>> {
    Ok(read_dir(path)
        .context(format!("failed to read directory {}", path.display()))?
        .filter_map(Result::ok)
        .filter(|entry| entry.metadata().map(|m| m.is_dir()).unwrap_or(false)))
}

/// if can read the dirs of the path, extract environments recursively from directories
pub fn map_directory(path: &Path) -> Result<Vec<Environment>> {
    let mut enviroments: Vec<Environment> = vec![];

    for dir in get_dirs(path)? {
        if let Ok(mut env) = providers::get_environment(&dir.path()) {
            if let EnvironmentDetails::Folder = env.details {
                env.children = map_directory(&env.source)
                    .context(format!("faild to map directory {}", path.display()))?;
            }
            enviroments.push(env);
        }
    }
    enviroments.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(enviroments)
}

/// From the path, recursively searches the environments within the folders for the environment with the given name
pub fn find_environment(path: &Path, name: &str) -> Option<Environment> {
    let name = name.to_lowercase();
    let mut folders: Vec<Environment> = vec![];
    let Ok(dirs) = get_dirs(path) else {
        return None;
    };
    for dir in dirs {
        if let Ok(mut env) = providers::get_environment(&dir.path()) {
            if let EnvironmentDetails::Folder = env.details {
                if let Ok(children) = map_directory(&env.source) {
                    env.children = children;
                };
            }
            if env.name.to_lowercase() == name {
                return Some(env);
            }
            folders.push(env)
        }
    }
    for folder in folders {
        if let Some(env) = find_environment(&folder.source, &name) {
            return Some(env);
        }
    }

    None
}
/// From the given path, go to the parent folders until you find an environment or reach the end(note: this ignores subprojects)
/// 
/// # Example
/// 
/// Considering a case where
/// - `/home/user/projects` is a folder environment
/// - `/home/user/projects/site` is a project environment
/// - `/home/user/projects/site/frontend` is a subproject environment
/// 
///  then call this function passing `/home/user/projects/site/frontend/src/css` as an argument
/// 
/// - It will first check if the folder `/home/user/projects/site/frontend/src/css` is an environment
/// - then the folder `/home/user/projects/site/frontend/src`
/// - then the folder `/home/user/projects/site/frontend` (will ignore because is a subproject)
/// - then the `/home/user/projects/site` folder, then it will stop and return because it found a environment
/// 
pub fn find_parent_environment(path: &Path) -> Option<Environment> {
    let mut path = path.to_path_buf();
    loop {
        if let Ok(env) = providers::get_environment(&path) {
            return Some(env);
        };
        if !path.pop() {
            return None;
        };
    }
}
