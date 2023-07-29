use crate::prelude::*;
use std::{process, str};

pub fn get_meta(path: &Path) -> Result<Metadata> {
    let output = process::Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .current_dir(path)
        .output()
        .context("path is not a git repository or verification failed")?;
	let is_repo = str::from_utf8(&output.stdout).context("check if it's repository fails")?.trim();
	if is_repo != "true" { 
		return Err(anyhow!("path is not a git repository"));
	}
	return Ok(Metadata::default());
}
