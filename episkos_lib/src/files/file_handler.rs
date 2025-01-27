//! # File Handler
//!
//! Provides functions for file operations such as reading, writing, and overwriting files
//! with serialization and deserialization support.
use std::{fs, path::Path};

use serde::{de::DeserializeOwned, Serialize};

use super::Error;

/// Utility struct for performing file operations with serialization/deserialization support.
pub struct FileHandler;

impl FileHandler {
    pub fn write_file(data: impl Serialize, path: &Path) -> Result<(), Error> {
        let toml = toml::to_string(&data)?;
        fs::write(path, toml)?;

        Ok(())
    }
    pub fn write_new_file(data: impl Serialize, path: &Path) -> Result<(), Error> {
        if path.exists() {
            return Err(Error::PathExists(
                path.to_str().unwrap_or_default().to_string(),
            ));
        }

        let toml = toml::to_string(&data)?;
        fs::write(path, toml)?;

        Ok(())
    }

    pub fn overwrite_file(data: impl Serialize, path: &Path) -> Result<(), Error> {
        if !path.exists() {
            return Err(Error::PathDoesNotExist(
                path.to_str().unwrap_or_default().to_string(),
            ));
        }

        let toml = toml::to_string(&data)?;
        fs::write(path, toml)?;

        Ok(())
    }
    pub fn read_file<T: DeserializeOwned>(path: &Path) -> Result<T, Error> {
        Ok(toml::from_str(&fs::read_to_string(path)?)?)
    }
}
