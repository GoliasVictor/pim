use std::path::{ Path};
use crate::metadata::Metadata;
mod dotmeta_provider;
use dotmeta_provider::DotMetaProvider;



pub fn get_meta(path : &Path) -> Option<Metadata> {
	if let Some(meta) = DotMetaProvider::get_meta(path) {
		return Some(meta)
	}
	return None
}

