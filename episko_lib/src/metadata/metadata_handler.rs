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
    /// Create a new [`MetadataHandler`] instance.
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Load all metadata files from the given [`Config`].
    ///
    /// # Errors
    /// - if the file cannot be read
    pub fn load_from_config(&self, config: &Config) -> Result<Vec<Metadata>> {
        Ok(config
            .files_to_load
            .iter()
            .filter_map(|el| Metadata::from_file(el).ok())
            .collect())
    }

    /// Save metadata to the database and file system.
    ///
    /// # Errors
    /// - if the metadata cannot be saved to the database
    /// - if the metadata cannot be saved to the file system
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

    /// Get paths to locations of manifests in the given directory.
    ///
    /// # Errors
    /// - if the directory cannot be read
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
}
