use crate::metadata::Metadata;
use serde::Deserialize;
use std::{
    fs::{self, File},
    path::{Path, PathBuf},
};

#[derive(Debug, Deserialize)]
struct Folder {
    name: Option<String>,
    path: String,
}
#[derive(Debug, Deserialize)]
struct VsCodeMeta {
    pub folders: Vec<Folder>,
}
pub fn get_meta(path: &Path) -> Option<Metadata> {
    let dir = fs::read_dir(path)
        .unwrap()
        .filter(|e| {
            e.as_ref()
                .unwrap()
                .path()
                .extension()
                .and_then(|e| e.to_str())
                .is_some_and(|e| e == "code-workspace")
        })
        .next()
        .and_then(|r| r.ok());
    if let Some(e) = dir {
        return File::open(e.path())
            .ok()
            .and_then(|f| serde_jsonrc::from_reader::<File, VsCodeMeta>(f).ok())
            .map(|vsm| Metadata {
                name: e
                    .path()
                    .file_stem()
                    .and_then(|n| n.to_str())
                    .map(|n| n.to_string()),
                children: Some(
                    vsm.folders
                        .into_iter()
                        .map(|folder| Metadata {
                            name: folder.name,
                            path: Some(PathBuf::from(folder.path)),
                            ..Default::default()
                        })
                        .collect(),
                ),
                ..Default::default()
            });
    }
    return None;
}
