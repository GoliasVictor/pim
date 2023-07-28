use crate::prelude::*;
use serde::Deserialize;
use std::{
    ffi::OsStr,
    fs::{self, File},
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
pub fn get_meta(path: &Path) -> Result<Metadata> {
    let entry = fs::read_dir(path)
        .context("failed to read directory")?
        .filter_map(Result::ok)
        .filter(|d| d.path().extension() == Some(OsStr::new("code-workspace")))
        .next()
        .context("directory has no .code-workspace file")?;

    let file = File::open(entry.path()).with_context(
            || format!("file to read configuration file  {0}", entry.path().to_string_lossy())
        )?;
    let vsm = serde_jsonrc::from_reader::<File, VsCodeMeta>(file)
        .with_context(
            || format!("failed to deserialize file {0}",entry.path().to_string_lossy())
        )?;
    let metadata = Metadata {
        name: entry
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
                    environment_type: Some(EnvironmentType::SubProject),
                    ..Default::default()
                })
                .collect(),
        ),
        ..Default::default()
    };
    return Ok(metadata);
}
