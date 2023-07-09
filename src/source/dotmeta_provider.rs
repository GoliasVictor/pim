use std::{fs::File, path::{Path, PathBuf}};
use crate::metadata::Metadata;
use super::Provider;

pub struct DotMetaProvider {
}
impl Provider for DotMetaProvider {
    fn get_meta(&self, path : &PathBuf) -> Option<Metadata> {
		
		let Ok(f) = File::open(path) else {
			return None	
		};
		let Ok(metadata) = serde_yaml::from_reader::<File, Metadata>(f) else {
			println!("{path:?}");
			return None
		} ;
		return Some(metadata);
	}
}