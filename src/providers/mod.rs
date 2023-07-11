use crate::metadata::Metadata;
use std::path::Path;
mod dotmeta_provider;
mod vscode_provider;

pub fn get_meta(path: &Path) -> Option<Metadata> {
    return dotmeta_provider::get_meta(path).or(vscode_provider::get_meta(path));
}
