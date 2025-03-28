//! # Removal of a manifest
//!
//! This module contains the code for the removal of a manifest and is directly used by the the remove command.

use camino::Utf8PathBuf;
use color_eyre::Result;
use episko_lib::{
    config::{Config, ConfigHandler},
    files::File,
    metadata::Metadata,
};

use crate::connect_to_db;

/// Remove a manifest file and try to remove it from the Database/Cache
///
/// # Errors
/// - [`color_eyre::Report`] when [`Metadata::remove_file`] fails
pub async fn remove_manifest(file: &Utf8PathBuf, config_handler: &mut ConfigHandler) -> Result<()> {
    if try_remove_from_db(file, config_handler.config())
        .await
        .is_err()
    {
        eprintln!("WARNING: Unable to remove metadata from cache!");
        eprintln!("The file will be deleted anyway...");
    }

    if config_handler.remove_saved_file(file.as_std_path()) {
        config_handler.save_config()?;
    }

    Metadata::remove_file(file.as_std_path())?;

    Ok(())
}

async fn try_remove_from_db(file: &Utf8PathBuf, config: &Config) -> Result<()> {
    let db = connect_to_db(config).await?;

    Ok(Metadata::from_file(file.as_std_path())?
        .remove_from_db(&db)
        .await?)
}
