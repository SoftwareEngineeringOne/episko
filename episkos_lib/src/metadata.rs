//! # Metadata
//!
//! Provides tools for handling project metadata, including the ability to build, update,
//! and serialize metadata structures.
use std::{
    io,
    path::{Path, PathBuf},
};

use builder::MetadataBuilder;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use uuid::Uuid;

pub mod build_system;
mod builder;
pub mod category;
pub mod ide;
pub mod language;
pub mod property;

pub use build_system::BuildSystem;
pub use category::Category;
pub use ide::Ide;
pub use language::Language;

/// Core metadata structure containing information about a project.
#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    id: Uuid,
    #[serde(skip)]
    directory: PathBuf,
    title: String,
    categories: Vec<Category>,
    languages: Vec<Language>,
    preffered_ide: Option<Ide>,
    build_systems: Vec<BuildSystem>,
    description: Option<String>,
    repository_url: Option<String>,
    created: DateTime<Utc>,
    updated: DateTime<Utc>,
}

impl Metadata {
    /// Creates a new builder for the metadata struct.
    pub fn builder() -> MetadataBuilder {
        MetadataBuilder::new()
    }

    /// Create a builder to update existing metadata.
    pub fn update(self) -> MetadataBuilder {
        MetadataBuilder::from_metadata(self)
    }

    /// Returns the directory associated with the metadata.
    pub fn directory(&self) -> &Path {
        &self.directory
    }

    /// Update the directory of the metadata.
    ///
    /// The "directory" field has to be treated differently, as
    /// it can differ between hosts, as such it is not written
    /// to the manifest file.
    pub fn update_directory(&mut self, path: PathBuf) {
        self.directory = path
    }
}

/// Errors related to metadata operations.
#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to build Metadata")]
    FailedToBuild(#[from] builder::Error),

    #[error("io error")]
    Io(#[from] io::Error),
}
