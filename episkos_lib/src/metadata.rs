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
pub mod ide;
pub mod language;

pub use build_system::BuildSystem;
pub use ide::Ide;
pub use language::Language;

use crate::file_system_handler::{self, FileSystemHandler};

#[derive(Debug, Serialize, Deserialize)]
pub struct Manifest {
    #[serde(rename(serialize = "Metadata", deserialize = "Metadata"))]
    metadata: Metadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    id: Uuid,
    #[serde(skip)]
    pub directory: PathBuf,
    title: String,
    categories: Vec<String>,
    languages: Vec<Language>,
    preffered_ide: Option<Ide>,
    build_systems: Vec<BuildSystem>,
    description: Option<String>,
    repository_url: Option<String>,
    created: DateTime<Utc>,
    updated: DateTime<Utc>,
}

impl Metadata {
    pub fn from_manifest(path: &Path) -> Result<Metadata, Error> {
        let path = match path.is_file() {
            true => path.to_path_buf(),
            false => path.join("manifest.toml"),
        }
        .canonicalize()?;
        let mut metadata: Metadata = FileSystemHandler::read_file::<Manifest>(&path)?.metadata;
        metadata.directory = path.canonicalize()?;
        Ok(metadata)
    }

    pub fn builder() -> MetadataBuilder {
        MetadataBuilder::new()
    }

    pub fn update(self) -> MetadataBuilder {
        MetadataBuilder::from_metadata(self)
    }

    pub fn write_manifest(self) -> Result<Self, Error> {
        let manifest = Manifest { metadata: self };

        FileSystemHandler::write_file(&manifest, &manifest.metadata.directory)?;

        Ok(manifest.metadata)
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to build Metadata")]
    FailedToBuild(#[from] builder::Error),

    #[error("file system error")]
    FileSystem(#[from] file_system_handler::Error),

    #[error("io error")]
    Io(#[from] io::Error),
}
