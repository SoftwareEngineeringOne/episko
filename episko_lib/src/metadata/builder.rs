//! # Building and updating [`Metadata`]
//!
//! The [`MetadataBuilder`] is the core struct for creating and
//! updating [`Metadata`].
use std::{
    io,
    path::{Path, PathBuf},
};

use chrono::{DateTime, Utc};
use thiserror::Error;
use uuid::Uuid;

// Temporary for compatibility reasons.
pub use crate::ApplyIf;

use super::{property::Property, BuildSystem, Category, Ide, Language, Metadata};

/// To allow for flexible building all fields
/// can start of as `None` or an empty `Vec`.
///
/// All fields, that don't say otherwise can be set
/// by the caller, but are optional
#[derive(Debug)]
pub struct MetadataBuilder {
    /// Can't be set by the caller, will be generated when building new
    id: Option<Uuid>,
    /// Must be set by the caller
    directory: Option<PathBuf>,
    /// Must be set by the caller
    title: Option<String>,
    categories: Vec<Category>,
    languages: Vec<Language>,
    preferred_ide: Option<Ide>,
    build_systems: Vec<BuildSystem>,
    description: Option<String>,
    repository_url: Option<String>,
    created: Option<DateTime<Utc>>,
    updated: Option<DateTime<Utc>>,
}

impl MetadataBuilder {
    /// Called when [`Metadata::builder()`] is invoked in order
    /// to create a new, empty instance.
    #[allow(clippy::new_without_default)]
    #[must_use]
    pub(crate) fn new() -> MetadataBuilder {
        Self {
            id: None,
            directory: None,
            title: None,
            categories: vec![],
            languages: vec![],
            preferred_ide: None,
            build_systems: vec![],
            description: None,
            repository_url: None,
            created: None,
            updated: None,
        }
    }

    /// Called when [`Metadata::update()`] is invoked in order
    /// to create a new instace, and moving all values from the calling
    /// metadata into that instance.
    #[must_use]
    pub fn from_metadata(metadata: Metadata) -> MetadataBuilder {
        Self {
            id: Some(metadata.id),
            directory: Some(metadata.directory),
            title: Some(metadata.title),
            categories: metadata.categories,
            languages: metadata.languages,
            preferred_ide: metadata.preferred_ide,
            build_systems: metadata.build_systems,
            description: metadata.description,
            repository_url: metadata.repository_url,
            created: Some(metadata.created),
            updated: None,
        }
    }

    /// Finishes the build process and ensures all required values are set.
    ///
    /// Additionally generates a new Id if necessarry, sets the creation date if not
    /// given and updates the last updated date.
    ///
    /// # Errors
    ///
    /// - [`Error::DirectoryMissing`], when the caller didn't provide a directory
    ///     or provided an invalid directory.
    /// - [`Error::TitleMissing`], when the caller didn't provide a title.
    pub fn build(mut self) -> Result<Metadata, Error> {
        self.categories.sort();
        self.build_systems.sort();
        self.languages.sort();

        Ok(Metadata {
            id: self.id.unwrap_or_else(Uuid::new_v4),
            directory: self.directory.ok_or(Error::DirectoryMissing)?,
            title: self.title.ok_or(Error::TitleMissing)?,
            categories: self.categories,
            languages: self.languages,
            preferred_ide: self.preferred_ide,
            build_systems: self.build_systems,
            description: self.description,
            repository_url: self.repository_url,
            created: self.created.unwrap_or_else(Utc::now),
            updated: self.updated.unwrap_or_else(Utc::now),
        })
    }

    /// Assign a set id instead of generating one when building.
    #[must_use]
    pub fn id(mut self, id: Uuid) -> Self {
        self.id = Some(id);
        self
    }

    /// Set the directory based on a `&str`.
    /// Invokes [`MetadataBuilder::directory_path`] after creating a
    /// [`Path`] instance.
    #[must_use]
    pub fn directory(self, directory: &str) -> Self {
        self.directory_path(Path::new(directory))
    }

    /// Set the metadatas directory based on a [`Path`].
    ///
    /// > When an invalid path is given, the builders path field won't
    /// > be set and an error will occur when trying to build.
    /// > This is not the cleanet solution and should be looked at, however
    /// > it allows for normalization of all builder methods.
    #[must_use]
    pub fn directory_path(mut self, path: &Path) -> Self {
        match path.canonicalize() {
            Ok(absolute_path) => {
                if absolute_path.is_dir() {
                    self.directory = Some(absolute_path.join("manifest.toml"));
                } else {
                    self.directory = Some(absolute_path);
                }
            }
            Err(_) => self.directory = Some(path.to_path_buf()),
        }

        self
    }

    /// Set the metadatas title.
    /// When not called during the build process, [`MetadataBuilder::build()`] will fail.
    #[must_use]
    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    /// Add one [`Category`] to already existing categories.
    #[must_use]
    pub fn add_category(mut self, category: &str) -> Self {
        if category.is_empty() {
            return self;
        }
        self.categories.push(Category::new(category));
        self
    }

    /// Replace the existing [`Vec<Category>`].
    #[must_use]
    pub fn categories(mut self, categories: Vec<Category>) -> Self {
        self.categories = categories;
        self
    }

    /// Add one [`Language`] to already existing categories.
    #[must_use]
    pub fn add_language(mut self, language: Language) -> Self {
        self.languages.push(language);
        self
    }

    /// Replace the existing [`Vec<Language>`].
    #[must_use]
    pub fn languages(mut self, languages: Vec<Language>) -> Self {
        self.languages = languages;
        self
    }

    /// Add one [`BuildSystem`] to already existing categories.
    #[must_use]
    pub fn add_build_system(mut self, build_system: BuildSystem) -> Self {
        self.build_systems.push(build_system);
        self
    }

    /// Replace the existing [`Vec<BuildSystem>`].
    #[must_use]
    pub fn build_systems(mut self, build_systems: Vec<BuildSystem>) -> Self {
        self.build_systems = build_systems;
        self
    }

    /// Set the preferred [`Ide`]
    #[must_use]
    pub fn preferred_ide(mut self, ide: Ide) -> Self {
        self.preferred_ide = Some(ide);
        self
    }

    /// Update the preferred [`Ide`]
    #[must_use]
    pub fn update_ide(mut self, ide: Option<Ide>) -> Self {
        self.preferred_ide = ide;
        self
    }

    /// Set the description
    #[must_use]
    pub fn description(mut self, description: &str) -> Self {
        self.description = match description.len() {
            0 => None,
            _ => Some(description.to_string()),
        };
        self
    }

    /// Update the description
    #[must_use]
    pub fn update_description(mut self, description: Option<String>) -> Self {
        self.description = description;
        self
    }

    /// Set the repository url
    #[must_use]
    pub fn repository_url(mut self, url: &str) -> Self {
        self.repository_url = match url.len() {
            0 => None,
            _ => Some(url.to_string()),
        };
        self
    }

    /// Update the repository url
    #[must_use]
    pub fn update_repository_url(mut self, url: Option<String>) -> Self {
        self.repository_url = url;
        self
    }

    /// Set the creation [`DateTime`]
    #[must_use]
    pub fn created(mut self, created: DateTime<Utc>) -> Self {
        self.created = Some(created);
        self
    }

    /// Set the updated [`DateTime`]
    #[must_use]
    pub fn updated(mut self, updated: DateTime<Utc>) -> Self {
        self.updated = Some(updated);
        self
    }
}

/// Can be used to add a property wrapped in an [`Option`] based
/// on if it's [`Some`].
///
/// # Example
///
/// ```
/// use episko_lib::{ApplyIf, metadata::{Metadata, MetadataBuilder, Category, property::Property}};
///
/// let builder: MetadataBuilder = Metadata::builder().title("Example").directory(".");
///
/// let category_some: Option<&str> = Some("Optional");
/// let category_none: Option<&str> = None;
///
/// // metadata will have the "Optional" category.
/// let metadata = builder
///     .apply_if(category_some, MetadataBuilder::add_category) // Category will be added
///     .apply_if(category_none, MetadataBuilder::add_category) // None will be ignored
///     .build()
///     .unwrap();
///
/// ```
impl ApplyIf for MetadataBuilder {
    fn apply_if<T>(self, value: Option<T>, f: fn(Self, T) -> Self) -> Self {
        match value {
            Some(x) => f(self, x),
            None => self,
        }
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
            .preferred_ide(Ide::new("VSCode"))
            .add_build_system(BuildSystem::with_version("Cargo", "1.84.0"))
            .description("A test project")
            .repository_url("https://github.com/test/project");

        let metadata = builder.build().unwrap();
        assert_eq!(metadata.title, "Test Project");
        assert_eq!(metadata.categories.len(), 1);
        assert_eq!(metadata.categories[0].name, "Category1");
        assert_eq!(metadata.languages.len(), 1);
        assert_eq!(metadata.languages[0].name, "Rust");
        assert!(metadata.preferred_ide.is_some());
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
}
