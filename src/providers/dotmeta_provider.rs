use crate::prelude::*;
use std::fs::File;

pub fn get_meta(path : &Path) -> Option<Metadata> {
	return File::open(path.join(".meta"))
		.ok()
		.and_then(|file| serde_yaml::from_reader::<File, Metadata>(file).ok());
}