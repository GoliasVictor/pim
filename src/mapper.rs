
use crate::prelude::*;
use crate::providers;
use std::fs::{read_dir, DirEntry};

pub fn get_dirs(path: &Path) -> Result<impl IntoIterator<Item = DirEntry>> {
    Ok(read_dir(path)
        .context(format!("failed to read directory {}", path.display()))?
        .filter_map(Result::ok)
        .filter(|entry| entry.metadata().map(|m| m.is_dir()).unwrap_or(false)))
}
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
