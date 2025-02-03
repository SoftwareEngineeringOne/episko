//! # Removal of a manifest
//!
//! This module contains the code for the removal of a manifest and is directly used by the the remove command.

use camino::Utf8PathBuf;
use color_eyre::Result;
use episko_lib::{database::DatabaseHandler, files::File, metadata::Metadata};

pub async fn remove_manifest(file: &Utf8PathBuf) -> Result<()> {
    let db = DatabaseHandler::default().await?;
    Metadata::remove_from_db(&Metadata::from_file(file.as_std_path())?, &db).await?;
    Metadata::remove_file(file.as_std_path())?;
    Ok(())
}
