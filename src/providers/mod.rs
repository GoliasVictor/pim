use crate::prelude::*; 
mod dotmeta_provider;
mod vscode_provider;

fn name_else_filename(opname: Option<String>, path: &Path) -> Option<String> {
    if let Some(name) = opname {
        if !name.trim().is_empty() {
            return Some(name);
        }
    }
    return path
        .file_name()
        .and_then(|s| s.to_str())
        .map(|s| s.to_string());
}
fn get_meta(path: &Path) -> Option<Metadata> {
    let mut op =  dotmeta_provider::get_meta(path).or(vscode_provider::get_meta(path));
    if let Some(ref mut meta) =  op {
        meta.source = path.to_path_buf();
        meta.name = name_else_filename(meta.name.clone(), &path);
        if let Some(EnvironmentType::Project) = meta.environment_type {
            if let Some(children) = &mut meta.children {
                for child in children.into_iter() {
                    if let Some(child_path) = child
                        .path
                        .as_ref()
                        .and_then(|p| meta.source.join(&p).canonicalize().ok())
                    {
                        child.name = name_else_filename(child.name.clone(), &child_path);
                    }
                    child.environment_type = Some(EnvironmentType::SubProject);
                }
            }
        }
    }
    return op;
}

pub fn get_environment(path: &Path) -> Option<Environment>{
    return get_meta(path).and_then(|m|Environment::from_metadata(m).ok())
}