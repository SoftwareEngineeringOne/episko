//! # File Operations Module
//!
//! Provides utilities and abstractions for file-related operations, such as reading and
//! writing files, and serializing/deserializing data.
//!
//! ## Submodules
//! - `file_handler`: Core file handling utilities.
//! - `metadata`: File-specific implementations for metadata handling.

use std::path::Path;

use serde::{de::DeserializeOwned, Serialize};

pub mod file_handler;
pub mod metadata;

/// A trait for file operations, providing methods for serialization and deserialization.
///
/// Types implementing this trait can be written to or loaded from files. It can
/// e.g. be used for Metadata or the Config
pub trait File: Serialize + DeserializeOwned {
    /// The error type returned by file operations.
    type Error;

    /// Writes the current object to the specified file path.
    ///
    /// # Parameters
    /// - `path`: The path to the file where the data should be written.
    /// _To keep the trait flexible, it won't be asumed, that the implementing
    /// type knows where the file should be placed, as it would be the case with
    /// Metadata._
    ///
    /// # Errors
    /// Returns `Self::Error` if the operation fails.
    fn write_file(&self, path: &Path) -> Result<(), Self::Error>;

    /// Creates an object from the specified file.
    ///
    /// # Parameters
    /// - `path`: The path to the file to be read.
    ///
    /// # Errors
    /// Returns `Self::Error` if the operation fails.
    fn from_file(path: &Path) -> Result<Self, Self::Error>;
}
