use crate::prelude::*;
use std::fs::File;

pub fn get_meta(path: &Path) -> Result<Metadata> {
    let meta_path = path.join(".meta");
    if !meta_path.exists() {
        return Err(anyhow!(
            "diretory {0} no has .meta file",
            path.to_string_lossy()
        ));
    }
    let file = File::open(&meta_path)
        .with_context(|| format!("failed to open file {0}", meta_path.to_string_lossy()))?;
    return serde_yaml::from_reader::<File, Metadata>(file).with_context(|| {
        format!(
            "failed to deserialize the file {0}",
            meta_path.to_string_lossy()
        )
    });
}
