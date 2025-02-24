//! # Validating and caching of a manifest
//!
//! This module contains the code for the validation and caching of a manifest and is directly used by the the validate and cache command.

use camino::Utf8PathBuf;
use color_eyre::Result;
use episko_lib::{
    config::{config_handler::ConfigHandler, Config},
    files::File,
    metadata::Metadata,
};

use crate::connect_to_db;

/// Validates a manifest file and further more validates the cache if
/// possible.
///
/// # Errors
/// - [`color_eyre::Report`] when [`Metadata::validate_file`] fails
pub async fn validate_manifest(file: &Utf8PathBuf, config_handler: &ConfigHandler) -> Result<()> {
    // TODO
    // let db = connect_to_db().await?;
    //Metadata::validate_db(&Metadata::from_file(file.as_std_path())?, &db).await?;
    Metadata::validate_file(file.as_std_path())?;
    Ok(())
}

/// Caches a manifest file to the Database/Cache
///
/// **TODO**
/// Currently calls [`Metadata::update_in_db`], but this fails when the
/// metadata isn't cached in the first place.
///
/// # Errors
/// - Error report when [`Metadata::update_in_db`] fails.
pub async fn cache_manifest(file: &Utf8PathBuf, config_handler: &ConfigHandler) -> Result<()> {
    let mut config = config_handler.load_config()?;
    let db = connect_to_db(&config).await?;

    let metadata = Metadata::from_file(file.as_std_path())?;
    Metadata::update_in_db(&metadata, &db).await?;

    // Reloading the config_handler / config isn't very pretty,
    // however I think it still that it's the simplest way to do this.
    config.add_saved_file(metadata.directory());
    config_handler.save_config(&config)?;

    Ok(())
}
