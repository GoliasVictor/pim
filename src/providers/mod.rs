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
pub fn get_meta(path: &Path) -> Result<Metadata> {
    let mut metadata = dotmeta_provider::get_meta(path).or(vscode_provider::get_meta(path))?;
    metadata.source = path.to_path_buf();
    metadata.name = name_else_filename(metadata.name.clone(), &path);
    if let Some(EnvironmentType::Project) = metadata.environment_type {
        if let Some(children) = &mut metadata.children {
            for child in children.into_iter() {
                if let Some(child_path) = child
                    .path
                    .as_ref()
                    .and_then(|p| metadata.source.join(&p).canonicalize().ok())
                {
                    child.name = name_else_filename(child.name.clone(), &child_path);
                }
                child.environment_type = Some(EnvironmentType::SubProject);
            }
        }
    }
    return Ok(metadata);
}

pub fn get_environment(path: &Path) -> Result<Environment> {
    let meta = get_meta(path)?;
    return Environment::from_metadata(meta);
}
