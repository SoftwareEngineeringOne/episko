use glob::glob;
use std::path::{Path, PathBuf};

use crate::{
    config::{Config, ConfigHandler},
    database::DatabaseHandler,
    files::File,
};

use super::{Error, Metadata, Result};

#[derive(Default, Debug)]
pub struct MetadataHandler;

impl MetadataHandler {
    /// !TODO!
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Loading should be done using tauri and events and stuff
    /// This function will probably be removed?
    ///
    /// # Errors
    /// !TODO!
    pub fn load_from_config(&self, config: &Config) -> Result<Vec<Metadata>> {
        Ok(config
            .files_to_load
            .iter()
            .filter_map(|el| Metadata::from_file(el).ok())
            .collect())
    }

    /// !TODO!
    ///
    /// # Errors
    /// !TODO!
    pub async fn save_metadata(
        metadata: &Metadata,
        db: &DatabaseHandler,
        config_handler: &mut ConfigHandler,
    ) -> Result<()> {
        metadata
            .write_to_db(db)
            .await
            .map_err(|err| Error::Save(err.to_string()))?;
        metadata
            .write_file(metadata.directory())
            .map_err(|err| Error::Save(err.to_string()))?;

        config_handler.add_saved_file(metadata.directory());

        config_handler
            .save_config()
            .map_err(|err| Error::Save(err.to_string()))?;

        Ok(())
    }

    pub async fn delete_metadata(
        metadata: &Metadata,
        db: &DatabaseHandler,
        config_handler: &mut ConfigHandler,
    ) -> Result<()> {
        // if part of dir, returns false but can be ignored
        let _ = config_handler.remove_saved_file(&metadata.directory);

        metadata
            .remove_from_db(&db)
            .await
            .map_err(|err| Error::Delete(err.to_string()))?;
        Metadata::remove_file(&metadata.directory).map_err(|err| Error::Delete(err.to_string()))?;

        Ok(())
    }

    /// Get paths to locations of manifests
    ///
    /// Loading should be done using tauri and events and stuff
    ///
    /// # Errors
    /// !TODO!
    pub fn search_directory(dir: &Path) -> Result<Vec<PathBuf>> {
        glob(
            dir.join("**/manifest.toml")
                .to_str()
                .ok_or(Error::Directory("unable to locate dir".to_string()))?,
        )
        .map_err(|err| Error::Directory(err.to_string()))?
        .map(|manifest| manifest.map_err(|err| Error::File(err.to_string())))
        .collect::<Result<Vec<_>>>()
    }

    /// !TODO!
    ///
    /// # Errors
    /// !TODO!
    pub fn search_metadata(_query: &str) -> Result<Vec<Metadata>> {
        todo!()
    }
}
