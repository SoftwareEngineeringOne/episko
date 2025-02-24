use std::collections::{HashMap, HashSet};

use crate::{
    config::{Config, ConfigHandler},
    database::DatabaseHandler,
    files::File,
};

use super::{Error, Metadata, Result};
use uuid::Uuid;

#[derive(Default, Debug)]
pub struct MetadataHandler {
    pub loaded_metadata: HashMap<Uuid, Metadata>,
}

impl MetadataHandler {
    pub fn new() -> Self {
        Self {
            loaded_metadata: HashMap::new(),
        }
    }

    pub fn load_from_config(&mut self, config: &Config) -> Result<()> {
        config
            .files_to_load
            .iter()
            .filter_map(|el| Metadata::from_file(&el).ok())
            .for_each(|metadata| {
                self.loaded_metadata.insert(metadata.id(), metadata);
            });
        Ok(())
    }

    pub async fn save_metadata(
        &mut self,
        metadata: Metadata,
        db: &DatabaseHandler,
        config_handler: &ConfigHandler,
    ) -> Result<()> {
        Self::save_metadata_static(&metadata, db, config_handler).await?;
        self.loaded_metadata.insert(metadata.id(), metadata);
        Ok(())
    }

    pub async fn save_metadata_static(
        metadata: &Metadata,
        db: &DatabaseHandler,
        config_handler: &ConfigHandler,
    ) -> Result<()> {
        metadata
            .write_to_db(db)
            .await
            .map_err(|err| Error::Save(err.to_string()))?;
        metadata
            .write_file(metadata.directory())
            .map_err(|err| Error::Save(err.to_string()))?;

        let mut config = config_handler
            .load_config()
            .map_err(|err| Error::Save(err.to_string()))?;
        config.add_saved_file(metadata.directory());
        config_handler
            .save_config(&config)
            .map_err(|err| Error::Save(err.to_string()))?;

        Ok(())
    }

    pub fn search_directory() -> Result<Vec<Metadata>> {
        todo!()
    }

    pub fn search_metadata(query: &str) -> Result<Vec<Metadata>> {
        todo!()
    }
}
