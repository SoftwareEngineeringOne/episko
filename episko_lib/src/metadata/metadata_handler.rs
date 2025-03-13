use glob::glob;
use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
};

use crate::{
    config::{Config, ConfigHandler},
    database::DatabaseHandler,
    files::File,
};

use super::{Error, Metadata, Result};
use uuid::Uuid;

#[derive(Default, Debug)]
pub struct MetadataHandler;

impl MetadataHandler {
    pub fn new() -> Self {
        Self
    }

    /// Loading should be done using tauri and events and stuff
    /// This function will probably be removed?
    pub fn load_from_config(&self, config: &Config) -> Result<Vec<Metadata>> {
        Ok(config
            .files_to_load
            .iter()
            .filter_map(|el| Metadata::from_file(&el).ok())
            .collect())
    }

    pub async fn save_metadata(
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

    /// Get paths to locations of manifests
    ///
    /// Loading should be done using tauri and events and stuff
    pub fn search_directory(dir: &Path) -> Result<Vec<PathBuf>> {
        Ok(glob(
            dir.join("**/manifest.toml")
                .to_str()
                .ok_or(Error::Directory("unable to locate dir".to_string()))?,
        )
        .map_err(|err| Error::Directory(err.to_string()))?
        .map(|manifest| manifest.map_err(|err| Error::File(err.to_string())))
        .collect::<Result<Vec<_>>>()?)
    }

    pub fn search_metadata(query: &str) -> Result<Vec<Metadata>> {
        todo!()
    }
}
