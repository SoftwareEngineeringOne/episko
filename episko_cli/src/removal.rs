//! # Removal of a manifest
//!
//! This module contains the code for the removal of a manifest and is directly used by the the remove command.

use camino::Utf8PathBuf;
use color_eyre::Result;
use episko_lib::{files::File, metadata::Metadata};

pub fn remove_manifest(file: &Utf8PathBuf) -> Result<()> {
    Ok(Metadata::remove_file(file.as_std_path())?)
}
