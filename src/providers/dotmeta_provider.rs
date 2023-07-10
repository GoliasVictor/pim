use std::{fs::File, path::{Path}};
use crate::metadata::Metadata;

pub struct DotMetaProvider {
}
impl DotMetaProvider {
	pub fn get_meta(path : &Path) -> Option<Metadata> {
		let Ok(f) = File::open(path.join(".meta")) else {
			return None	
		};
		let Ok(metadata) = serde_yaml::from_reader::<File, Metadata>(f) else {
			return None
		} ;
		return Some(metadata);
	}
}