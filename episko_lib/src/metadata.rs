//! # Metadata used within the project
//!
//! This module contains the metadata upon which the project is based.
//!
//! ## Important interfaces
//! The [`Metadata`] struct is at the core of the whole project and
//! as such offers a few important interfaces which should be used.
//! ### Builder
//! A metadata object can be created or updated using the [`MetadataBuilder`]
//!
//! #### Example
//! ```
//! use episko_lib::metadata::Metadata;
//! use std::path::Path;
//!
//! // Creating a minimal metadata object
//! let metadata = Metadata::builder()
//!     .title("Example Project")
//!     .directory(".")
//!     .build()
//!     // Building can fail e.g. if the title is missing or the directory is invalid
//!     .unwrap();
//!
//! // The builder can also be used to update metadata
//! metadata.update()
//!     .add_category("example")
//!     .build()
//!     .unwrap();
//!
//! ```
//! ### Validation
//! In order to check wether or not a Manifest/Metadata has been changed,
//! the [`Metadata::get_hash()`] function can be used. As this also serializes the
//! struct in the process, receiving a hash, also implies a valid struct.
//!
//! ### Properties
//! The metadata struct holds different kinds of attributes, as were defined
//! during the earlier phases of this project. Properties can be split into
//! 2 categories.
//!
//! #### Simple
//! These properties can be described by a type from the std library or another
//! crate that is used. They need no special attention.
//! #### Advanced
//! Advanced Properties are properties, which would be described as
//! seperate entities in an ERM model. They can be shared between
//! metadata and can be used to describe relations.
//!
//! These properties are accompanied by additional traits and structures, which can
//! be found as submodules.
use std::{
    io,
    path::{Path, PathBuf},
};

use chrono::{DateTime, Utc};
use property::Property;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use thiserror::Error;

use uuid::Uuid;

pub mod build_system;
pub mod builder;
pub mod category;
pub mod ide;
pub mod language;
pub mod metadata_handler;
pub mod property;

pub use build_system::BuildSystem;
pub use builder::MetadataBuilder;
pub use category::Category;
pub use ide::Ide;
pub use language::Language;

pub type Result<T> = std::result::Result<T, Error>;

/// Metadata structure containing information about a project.
#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub id: Uuid,
    #[serde(skip)]
    pub directory: PathBuf,
    pub title: String,
    pub description: Option<String>,
    #[serde(rename = "category")]
    pub categories: Vec<Category>,
    #[serde(rename = "language")]
    pub languages: Vec<Language>,
    #[serde(rename = "build_system")]
    pub build_systems: Vec<BuildSystem>,
    pub preferred_ide: Option<Ide>,
    pub repository_url: Option<String>,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

impl Metadata {
    /// Retrieve a builder to create a new Metadata object.
    #[must_use]
    pub fn builder() -> MetadataBuilder {
        MetadataBuilder::new()
    }

    /// Consumes the instance and returns a builder with
    /// the corresponding values.
    #[must_use]
    pub fn update(self) -> MetadataBuilder {
        MetadataBuilder::from_metadata(self)
    }

    /// Retrieve the directory the manifest file of this
    /// metadata object is/should be safed in.
    ///
    /// As the directory can differ from host to host, this
    /// property needs to be treated special and as such is also
    /// not serialied/deserialized.
    #[must_use]
    pub fn directory(&self) -> &Path {
        &self.directory
    }

    /// !TODO!
    pub fn update_directory(&mut self, path: PathBuf) {
        self.directory = path;
    }

    /// !TODO!
    #[must_use]
    pub fn id(&self) -> Uuid {
        self.id
    }

    /// !TODO!
    pub fn update_ids(&mut self) {
        self.categories.iter_mut().for_each(Property::update_id);
        self.languages.iter_mut().for_each(Property::update_id);
        self.build_systems.iter_mut().for_each(Property::update_id);
        self.preferred_ide.iter_mut().for_each(Property::update_id);
    }

    /// Generate a Sha256 hash based on the instance for
    /// validation purposes or to check for changes.
    ///
    /// # Errors
    ///
    /// - [`Error::Serialization`], when unable to serialize into toml string
    pub fn get_hash(&self) -> Result<[u8; 32]> {
        let string = toml::to_string(self)?;

        let mut hasher = Sha256::new();
        hasher.update(string);
        Ok(hasher.finalize().into())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetadataPreview {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub categories: Vec<Category>,
    pub languages: Vec<Language>,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to build Metadata")]
    FailedToBuild(#[from] builder::Error),

    #[error("io error")]
    Io(#[from] io::Error),

    #[error("serialization error")]
    Serialization(#[from] toml::ser::Error),

    #[error("name cant be empty")]
    EmptyName,

    #[error("failed to load from file: {0}")]
    File(String),

    #[error("unable to save metadata: {0}")]
    Save(String),

    #[error("unable to find manifests in directory: {0}")]
    Directory(String),
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_metadata_checksum_is_consistent() {
        let metadata = get_simple_metadata();
        let checksum1 = metadata.get_hash().unwrap();
        for _ in 0..100 {
            let checksum2 = metadata.get_hash().unwrap();
            assert_eq!(checksum1, checksum2)
        }
    }

    #[test]
    fn test_metadata_checksum_is_changing() {
        let metadata = get_simple_metadata();
        let checksum1 = metadata.get_hash().unwrap();

        let metadata = metadata.update().build().unwrap();
        let checksum2 = metadata.get_hash().unwrap();
        assert_ne!(checksum1, checksum2);

        let metadata = metadata.update().title("Fun").build().unwrap();
        let checksum3 = metadata.get_hash().unwrap();
        assert_ne!(checksum1, checksum3);
        assert_ne!(checksum2, checksum3);
    }

    fn get_simple_metadata() -> Metadata {
        Metadata::builder()
            .title("Hello")
            .directory("/")
            .build()
            .unwrap()
    }
}
