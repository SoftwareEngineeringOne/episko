use std::{fs, path::Path};

use serde::{de::DeserializeOwned, Serialize};
use thiserror::Error;

pub struct FileSystemHandler;

impl FileSystemHandler {
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
}
