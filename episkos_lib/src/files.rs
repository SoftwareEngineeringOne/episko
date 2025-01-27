//! # File serialization and deserialization
use std::{io, path::Path};

use serde::{de::DeserializeOwned, Serialize};
use thiserror::Error;

pub mod file_handler;
pub mod metadata;

pub trait File: Serialize + DeserializeOwned {
    type Error: From<io::Error>;

    fn write_file(&self, path: &Path) -> Result<(), Self::Error>;

    fn from_file(path: &Path) -> Result<Self, Self::Error>;

    fn remove_file(path: &Path) -> Result<(), Self::Error> {
        match Self::validate_file(path) {
            Ok(()) => Ok(std::fs::remove_file(path)?),
            Err(e) => Err(e),
        }
    }

    fn validate_file(path: &Path) -> Result<(), Self::Error> {
        Self::from_file(path)?;
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("io error")]
    Io(#[from] std::io::Error),

    #[error("toml serialization error")]
    TomlSerialization(#[from] toml::ser::Error),

    #[error("toml deserialization error")]
    TomlDeserialization(#[from] toml::de::Error),

    #[error("path {0} already exists")]
    PathExists(String),

    #[error("path {0} does not exist")]
    PathDoesNotExist(String),

    #[error("validation error")]
    Invalid(#[from] crate::metadata::Error),
}
