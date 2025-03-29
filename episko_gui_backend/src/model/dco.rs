use episko_lib::{
    metadata::{
        property::Property, BuildSystem, Category, Ide, Language, Metadata, MetadataBuilder,
    },
    ApplyIf as _,
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::Error;

/// Dco data creation object
/// !TODO!
#[derive(Serialize, Deserialize, Debug)]
pub struct MetadataDco {
    directory: PathBuf,
    title: String,
    description: Option<String>,
    categories: Vec<Category>,
    languages: Vec<Language>,
    build_systems: Vec<BuildSystem>,
    preferred_ide: Option<Ide>,
    repository_url: Option<String>,
}

impl MetadataDco {
    /// !TODO!
    ///
    /// # Errors
    /// !TODO!
    pub fn create(mut self) -> Result<Metadata, Error> {
        self.categories.iter_mut().for_each(Property::update_id);
        self.build_systems.iter_mut().for_each(Property::update_id);
        self.languages.iter_mut().for_each(Property::update_id);
        self.preferred_ide.iter_mut().for_each(Property::update_id);

        Ok(Metadata::builder()
            .directory_path(&self.directory)
            .title(&self.title)
            .categories(self.categories)
            .languages(self.languages)
            .build_systems(self.build_systems)
            .apply_if(self.preferred_ide, MetadataBuilder::preferred_ide)
            .apply_if(self.description.as_deref(), MetadataBuilder::description)
            .apply_if(
                self.repository_url.as_deref(),
                MetadataBuilder::repository_url,
            )
            .build()?)
    }

    /// !TODO!
    ///
    /// # Errors
    /// !TODO!
    pub fn update(mut self, metadata: Metadata) -> Result<Metadata, Error> {
        self.categories.iter_mut().for_each(Property::update_id);
        self.build_systems.iter_mut().for_each(Property::update_id);
        self.languages.iter_mut().for_each(Property::update_id);
        self.preferred_ide.iter_mut().for_each(Property::update_id);

        Ok(metadata
            .update()
            .directory_path(&self.directory)
            .title(&self.title)
            .categories(self.categories)
            .languages(self.languages)
            .build_systems(self.build_systems)
            .update_ide(self.preferred_ide)
            .update_description(self.description)
            .update_repository_url(self.repository_url)
            .build()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use episko_lib::metadata::{
        property::Property as _, BuildSystem, Category, Ide, Language, Metadata,
    };
    use std::path::PathBuf;

    #[test]
    fn test_create_metadata() {
        // Arrange
        let category = Category::new("Application");
        let language = Language::new("Rust");
        let build_system = BuildSystem::new("Cargo");
        let ide = Ide::new("Neovim");

        let dco = MetadataDco {
            directory: PathBuf::from("."),
            title: String::from("Test Project"),
            description: Some(String::from("A test project description")),
            categories: vec![category],
            languages: vec![language],
            build_systems: vec![build_system],
            preferred_ide: Some(ide),
            repository_url: Some(String::from("https://github.com/test/project")),
        };

        // Act
        let result = dco.create();

        // Assert
        assert!(result.is_ok());
        let metadata = result.unwrap();
        assert_eq!(metadata.title, "Test Project");
        assert_eq!(
            metadata.description,
            Some("A test project description".to_string())
        );
        assert_eq!(metadata.categories[0].name, "Application");
        assert_eq!(metadata.languages[0].name, "Rust");
        assert_eq!(metadata.build_systems[0].name, "Cargo");
        assert_eq!(metadata.preferred_ide.unwrap().name, "Neovim");
        assert_eq!(
            metadata.repository_url,
            Some("https://github.com/test/project".to_string())
        );
    }

    #[test]
    fn test_update_metadata() {
        let existing_metadata = Metadata::builder()
            .directory(".")
            .title("Old Project")
            .categories(vec![Category::new("Library")])
            .languages(vec![Language::new("Python")])
            .build_systems(vec![BuildSystem::new("Make")])
            .build()
            .unwrap();

        let category = Category::new("Application");
        let language = Language::new("Rust");
        let build_system = BuildSystem::new("Cargo");
        let ide = Ide::new("Neovim");

        let dco = MetadataDco {
            directory: PathBuf::from("."),
            title: String::from("Updated Project"),
            description: Some(String::from("An updated project description")),
            categories: vec![category],
            languages: vec![language],
            build_systems: vec![build_system],
            preferred_ide: Some(ide),
            repository_url: Some(String::from("https://github.com/updated/project")),
        };

        let result = dco.update(existing_metadata);

        assert!(result.is_ok());
        let updated_metadata = result.unwrap();
        assert_eq!(updated_metadata.title, "Updated Project");
        assert_eq!(
            updated_metadata.description,
            Some("An updated project description".to_string())
        );
        assert_eq!(updated_metadata.categories[0].name, "Application");
        assert_eq!(updated_metadata.languages[0].name, "Rust");
        assert_eq!(updated_metadata.languages[0].version, None);
        assert_eq!(updated_metadata.build_systems[0].name, "Cargo");
        assert_eq!(updated_metadata.preferred_ide.unwrap().name, "Neovim");
        assert_eq!(
            updated_metadata.repository_url,
            Some("https://github.com/updated/project".to_string())
        );
    }
}
