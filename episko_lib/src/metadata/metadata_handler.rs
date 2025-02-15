use crate::config::Config;

use super::{Metadata, Result};

#[derive(Default, Debug)]
pub struct MetadataHandler {
    loaded_metadata: Vec<Metadata>,
}

impl MetadataHandler {
    pub fn new() -> Self {
        Self {
            loaded_metadata: vec![],
        }
    }

    pub fn load_from_config(self, config: &Config) -> Result<()> {
        todo!()
    }

    pub fn search_directory() -> Result<Vec<Metadata>> {
        todo!()
    }

    pub fn search_metadata(query: &str) -> Result<Vec<Metadata>> {
        todo!()
    }
}
