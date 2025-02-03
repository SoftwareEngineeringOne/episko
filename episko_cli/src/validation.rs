//! # Validating and caching of a manifest
//!
//! This module contains the code for the validation and caching of a manifest and is directly used by the the validate and cache command.

use camino::Utf8PathBuf;
use color_eyre::Result;
use episko_lib::{database::DatabaseHandler, files::File, metadata::Metadata};

pub async fn validate_manifest(file: &Utf8PathBuf) -> Result<()> {
    // TODO
    //let db = DatabaseHandler::default().await?;
    //Metadata::validate_db(&Metadata::from_file(file.as_std_path())?, &db).await?;
    Metadata::validate_file(file.as_std_path())?;
    Ok(())
}

pub async fn cache_manifest(file: &Utf8PathBuf) -> Result<()> {
    let db = DatabaseHandler::default().await?;
    Metadata::update_in_db(&Metadata::from_file(file.as_std_path())?, &db).await?;
    Ok(())
}
