//! # File serialization and deserialization
//!
//! This module is placed behind the "files" feature flag, wich is
//! enabled by default.
//!
//! ## Using the [`File`] trait
//!
//! The public API for structs that can be written to files is based on the
//! `File` trait, which should be implemented for these structs.
//!
//! ## Using the [`file_handler::FileHandler`] struct
//!
//! The `FileHandler` struct can also be used directly for structs that can't
//! implement the `File` trait because of the [orphan rule](https://doc.rust-lang.org/book/ch10-02-traits.html)
//! but implementing `File` should be preferred.
use std::path::Path;

use serde::{de::DeserializeOwned, Serialize};
use thiserror::Error;

pub mod config;
pub mod file_handler;
pub mod metadata;

pub type Result<T> = std::result::Result<T, Error>;

/// The `File` trait should be implemented on structs that should
/// posess the capabilities to be written and read from toml files.
///
/// For serialization and deserialization the [toml](https://docs.rs/toml/latest/toml/)
/// crate is used, and as such structs wanting to implement this trait also need
/// to implement `Serialize` and `DeserializeOwned`.
pub trait File: Serialize + DeserializeOwned {
    /// Write `Self` into a toml file.
    ///
    /// # Errors
    ///
    /// Returned errors may differ based on the implementation but should
    /// largely be based on the errors as returned by [`file_handler::FileHandler`]
    fn write_file(&self, path: &Path) -> Result<()>;

    /// Read `Self` from a toml file.
    ///
    /// # Errors
    ///
    /// Returned errors may differ based on the implementation but should
    /// largely be based on the errors as returned by [`file_handler::FileHandler`]
    fn from_file(path: &Path) -> Result<Self>;

    /// Remove a file. This is a very generic function, as it doesn't
    /// require the implementor to be aware of it's file location and leaves
    /// this to the discretion of the caller.
    ///
    /// By adding a function which returns the proper location of "Self's" file
    /// thus requiring it to be self aware, this function could be made more concrete.
    ///
    /// # Errors
    /// _For the default implementation_
    /// - [`Error::Io`] when the give file does not exist.
    ///
    fn remove_file(path: &Path) -> Result<()> {
        match Self::validate_file(path) {
            Ok(()) => Ok(std::fs::remove_file(path)?),
            Err(e) => Err(e),
        }
    }

    /// Validates the given file by trying to create `Self` from the
    /// given file.
    ///
    /// # Errors
    /// Returns an [`enum@Error`] when the serialization failed and as such
    /// returns the same errors as [`File::from_file`].
    fn validate_file(path: &Path) -> Result<()> {
        Self::from_file(path)?;
        Ok(())
    }
}

/// Errors used within this module.
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
