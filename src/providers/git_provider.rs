use crate::prelude::*;
use std::{process, str};

pub fn get_meta(path: &Path) -> Result<Metadata> {
    let output = process::Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .current_dir(path)
        .output()
        .context("path is not a git repository or verification failed")?;
	let path_git_root = str::from_utf8(&output.stdout).context("check if it's repository fails")?.trim();
	let root  = PathBuf::from(path_git_root);
	if root != path { 
		return Err(anyhow!("path is not a root of a git repository"));
	}
	Ok(Metadata::default())
}
