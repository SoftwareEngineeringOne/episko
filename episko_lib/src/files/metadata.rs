//! This module contains implementations regarding file based operations
//! for the [`Metadata`] struct.
use crate::metadata::Metadata;

use super::{file_handler::FileHandler, File, Result};

/// Implementing the [`File`] trait allows calling serialization and
/// deserialization functionality directly from an instance of [`Metadata`]
/// when this module is in scope.
impl File for Metadata {
    /// Write an instance to a file.
    ///
    /// # Errors
    ///
    /// The same errors as [`FileHandler::write_file`] are returned.
    fn write_file(&self, path: &std::path::Path) -> Result<()> {
        FileHandler::write_file(self, path)
    }

    fn from_file(path: &std::path::Path) -> Result<Self> {
        let path = if path.is_file() {
            path.to_path_buf()
        } else {
            path.join("manifest.toml")
        }
        .canonicalize()?;

        let mut metadata: Metadata = FileHandler::read_file(&path)?;
        metadata.update_directory(path);
        metadata.update_ids();
        Ok(metadata)
    }
}
