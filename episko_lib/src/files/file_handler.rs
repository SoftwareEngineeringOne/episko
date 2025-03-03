//! # File Handler
//!
//! Provides functions for file operations such as reading, writing, and overwriting files
//! with serialization and deserialization support.
use std::{fs, path::Path};

use serde::{de::DeserializeOwned, Serialize};

use super::{Error, Result};

/// Utility struct for performing file operations with serialization/deserialization support.
pub struct FileHandler;

impl FileHandler {
    /// Write a file with toml data, overwriting it if it exists or creating a new one if not.
    ///
    /// # Errors
    ///
    /// - [`Error::Io`] when a [`std::io::Error`] occurred
    /// - [`Error::TomlSerialization`] when serialization of the given data failed
    pub fn write_file(data: impl Serialize, path: &Path) -> Result<()> {
        let toml = toml::to_string(&data)?;
        fs::write(path, toml)?;

        Ok(())
    }

    /// Write a file with toml data, only if it doesn't exist yet.
    /// This function should be used when you ensure, that no exising file is
    /// overwritten.
    ///
    /// # Errors
    ///
    /// - [`Error::Io`] when a [`std::io::Error`] occurred
    /// - [`Error::TomlSerialization`] when serialization of the given data failed
    /// - [`Error::PathExists`] when the given file aready exists
    pub fn write_new_file(data: impl Serialize, path: &Path) -> Result<()> {
        if path.exists() {
            return Err(Error::PathExists(
                path.to_str().unwrap_or_default().to_string(),
            ));
        }

        let toml = toml::to_string(&data)?;
        fs::write(path, toml)?;

        Ok(())
    }

    /// Write toml data to an existing file.
    /// This function is used when you want to ensure, that an existing file is updated.
    ///
    /// # Errors
    ///
    /// - [`Error::Io`] when a [`std::io::Error`] occurred
    /// - [`Error::TomlSerialization`] when serialization of the given data failed
    /// - [`Error::PathDoesNotExist`] when the given file doesn't exists
    pub fn overwrite_file(data: impl Serialize, path: &Path) -> Result<()> {
        if !path.exists() {
            return Err(Error::PathDoesNotExist(
                path.to_str().unwrap_or_default().to_string(),
            ));
        }

        let toml = toml::to_string(&data)?;
        fs::write(path, toml)?;

        Ok(())
    }

    /// Read toml data from a given file.
    ///
    /// # Errors
    ///
    /// - [`Error::Io`] when a `std::io::Error` occurred
    /// - [`Error::TomlDeserialization`] when deserialization of the given data failed
    pub fn read_file<T: DeserializeOwned>(path: &Path) -> Result<T> {
        Ok(toml::from_str(&fs::read_to_string(path)?)?)
    }
}
