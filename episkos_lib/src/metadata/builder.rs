use std::{
    io,
    path::{Path, PathBuf},
};

use chrono::{DateTime, Utc};
use thiserror::Error;
use uuid::Uuid;

use super::{BuildSystem, Ide, Language, Metadata};

pub struct MetadataBuilder {
    id: Option<Uuid>,
    directory: Option<PathBuf>,
    title: Option<String>,
    categories: Vec<String>,
    languages: Vec<Language>,
    preffered_ide: Option<Ide>,
    build_systems: Vec<BuildSystem>,
    description: Option<String>,
    repository_url: Option<String>,
    created: Option<DateTime<Utc>>,
}

impl MetadataBuilder {
    pub fn new() -> MetadataBuilder {
        Self {
            id: None,
            directory: None,
            title: None,
            categories: vec![],
            languages: vec![],
            preffered_ide: None,
            build_systems: vec![],
            description: None,
            repository_url: None,
            created: None,
        }
    }

    pub fn from_metadata(metadata: Metadata) -> MetadataBuilder {
        Self {
            id: Some(metadata.id),
            directory: Some(metadata.directory),
            title: Some(metadata.title),
            categories: metadata.categories,
            languages: metadata.languages,
            preffered_ide: metadata.preffered_ide,
            build_systems: metadata.build_systems,
            description: metadata.description,
            repository_url: metadata.repository_url,
            created: Some(metadata.created),
        }
    }

    pub fn build(self) -> Result<Metadata, Error> {
        Ok(Metadata {
            id: self.id.unwrap_or_else(Uuid::new_v4),
            directory: self.directory.ok_or_else(|| Error::DirectoryMissing)?,
            title: self.title.ok_or_else(|| Error::TitleMissing)?,
            categories: self.categories,
            languages: self.languages,
            preffered_ide: self.preffered_ide,
            build_systems: self.build_systems,
            description: self.description,
            repository_url: self.repository_url,
            created: self.created.unwrap_or_else(Utc::now),
            updated: Utc::now(),
        })
    }

    pub fn id(mut self, id: Uuid) -> Self {
        self.id = Some(id);
        self
    }

    pub fn directory(mut self, directory: &Path) -> Result<Self, Error> {
        let absolute_path = directory.canonicalize()?.join("manifest.toml");
        self.directory = Some(absolute_path);
        Ok(self)
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    pub fn add_category(mut self, category: &str) -> Self {
        if category.len() == 0 {
            return self;
        }
        self.categories.push(category.to_string());
        self
    }

    pub fn categories(mut self, categories: Vec<String>) -> Self {
        self.categories = categories;
        self
    }

    pub fn add_language(mut self, language: Language) -> Self {
        self.languages.push(language);
        self
    }

    pub fn languages(mut self, languages: Vec<Language>) -> Self {
        self.languages = languages;
        self
    }

    pub fn preffered_ide(mut self, ide: Ide) -> Self {
        self.preffered_ide = Some(ide);
        self
    }

    pub fn add_build_system(mut self, build_system: BuildSystem) -> Self {
        self.build_systems.push(build_system);
        self
    }

    pub fn build_systems(mut self, build_systems: Vec<BuildSystem>) -> Self {
        self.build_systems = build_systems;
        self
    }

    pub fn description(mut self, description: &str) -> Self {
        self.description = match description.len() {
            0 => None,
            _ => Some(description.to_string()),
        };
        self
    }

    pub fn repository_url(mut self, url: &str) -> Self {
        self.repository_url = match url.len() {
            0 => None,
            _ => Some(url.to_string()),
        };
        self
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("title missing")]
    TitleMissing,

    #[error("directory missing")]
    DirectoryMissing,

    #[error("io error")]
    IoError(#[from] io::Error),
}
