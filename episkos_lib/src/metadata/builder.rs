use std::{
    io,
    path::{Path, PathBuf},
};

use chrono::{DateTime, Utc};
use thiserror::Error;
use uuid::Uuid;

use super::{BuildSystem, Category, Ide, Language, Metadata};

pub struct MetadataBuilder {
    id: Option<Uuid>,
    directory: Option<PathBuf>,
    title: Option<String>,
    categories: Vec<Category>,
    languages: Vec<Language>,
    preffered_ide: Option<Ide>,
    build_systems: Vec<BuildSystem>,
    description: Option<String>,
    repository_url: Option<String>,
    created: Option<DateTime<Utc>>,
}

impl MetadataBuilder {
    #[allow(clippy::new_without_default)]
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
            directory: self.directory.ok_or(Error::DirectoryMissing)?,
            title: self.title.ok_or(Error::TitleMissing)?,
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

    pub fn directory(mut self, directory: &str) -> Self {
        match Path::new(directory).canonicalize() {
            Ok(absolute_path) => self.directory = Some(absolute_path.join("manifest.toml")),
            Err(_) => self.directory = None,
        }
        self
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    pub fn add_category(mut self, category: &str) -> Self {
        if category.is_empty() {
            return self;
        }
        self.categories.push(Category::new(category));
        self
    }

    pub fn categories(mut self, categories: Vec<Category>) -> Self {
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
    Io(#[from] io::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata_builder() {
        let builder = MetadataBuilder::new()
            .title("Test Project")
            .directory(".")
            .add_category("Category1")
            .add_language(Language::with_version("Rust", "1.84.0"))
            .preffered_ide(Ide::new("VSCode"))
            .add_build_system(BuildSystem::with_version("Cargo", "1.84.0"))
            .description("A test project")
            .repository_url("https://github.com/test/project");

        let metadata = builder.build().unwrap();
        assert_eq!(metadata.title, "Test Project");
        assert_eq!(metadata.categories.len(), 1);
        assert_eq!(metadata.categories[0].name, "Category1");
        assert_eq!(metadata.languages.len(), 1);
        assert_eq!(metadata.languages[0].name, "Rust");
        assert!(metadata.preffered_ide.is_some());
        assert_eq!(metadata.build_systems.len(), 1);
        assert_eq!(metadata.description, Some("A test project".to_string()));
        assert_eq!(
            metadata.repository_url,
            Some("https://github.com/test/project".to_string())
        );
    }

    #[test]
    fn test_metadata_missing_title() {
        let builder = MetadataBuilder::new().directory(".");
        let result = builder.build();
        assert!(result.is_err());
        if let Err(err) = result {
            match err {
                Error::TitleMissing => (),
                _ => panic!("Unexpected error type"),
            }
        }
    }

    #[test]
    fn test_metadata_missing_dir() {
        let builder = MetadataBuilder::new().title("Test");
        let result = builder.build();
        assert!(result.is_err());
        if let Err(err) = result {
            match err {
                Error::DirectoryMissing => (),
                _ => panic!("Unexpected error type"),
            }
        }
    }

    #[test]
    fn test_metadata_invalid_dir() {
        let data = MetadataBuilder::new()
            .title("Test")
            .directory("/a/b/c/d/e/f/g")
            .build();
        assert!(data.is_err());
        if let Err(err) = data {
            match err {
                Error::DirectoryMissing => (),
                _ => panic!("Unexpected error type"),
            }
        }
    }
}
