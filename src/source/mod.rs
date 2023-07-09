use std::path::{ PathBuf};
use crate::metadata::Metadata;
mod dotmeta_provider;
use dotmeta_provider::DotMetaProvider;
trait Provider  {
	fn get_meta(&self, path : &PathBuf) -> Option<Metadata>;
}

pub struct MetadataProvider {
	providers : Vec<Box<dyn Provider>>
}


impl MetadataProvider {
	pub fn new() -> Self {
		return Self {
			providers: vec!{
				Box::new(DotMetaProvider{})
			}
		}
	}
	pub fn get_meta(&self, path : &PathBuf) -> Option<Metadata> {
        self.providers.iter().find_map(|provider| provider.get_meta(&path))
    }
}
