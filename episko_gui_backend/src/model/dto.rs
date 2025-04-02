use std::path::PathBuf;

use chrono::{DateTime, Utc};
use episko_lib::metadata::{BuildSystem, Category, Ide, Language, Metadata};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// DTO: Data Transfer Object
#[derive(Serialize, Deserialize, Debug)]
pub struct MetadataDto {
    id: Uuid,
    directory: PathBuf,
    title: String,
    description: Option<String>,
    categories: Vec<Category>,
    languages: Vec<Language>,
    build_systems: Vec<BuildSystem>,
    preferred_ide: Option<Ide>,
    repository_url: Option<String>,
    created: DateTime<Utc>,
    updated: DateTime<Utc>,
}

impl From<Metadata> for MetadataDto {
    fn from(metadata: Metadata) -> MetadataDto {
        MetadataDto {
            id: metadata.id,
            directory: metadata.directory,
            title: metadata.title,
            description: metadata.description,
            categories: metadata.categories,
            languages: metadata.languages,
            build_systems: metadata.build_systems,
            preferred_ide: metadata.preferred_ide,
            repository_url: metadata.repository_url,
            created: metadata.created,
            updated: metadata.updated,
        }
    }
}

impl From<MetadataDto> for Metadata {
    fn from(val: MetadataDto) -> Self {
        Metadata {
            id: val.id,
            directory: val.directory,
            title: val.title,
            description: val.description,
            categories: val.categories,
            languages: val.languages,
            build_systems: val.build_systems,
            preferred_ide: val.preferred_ide,
            repository_url: val.repository_url,
            created: val.created,
            updated: val.updated,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use episko_lib::metadata::{
        property::Property as _, BuildSystem, Category, Ide, Language, Metadata,
    };
    use uuid::Uuid;

    #[test]
    fn test_metadata_to_metadata_dto_conversion() {
        let category = Category::new("Application");
        let language = Language::new("Rust");
        let build_system = BuildSystem::new("Cargo");
        let ide = Ide::new("Neovim");
        let id = Uuid::new_v4();
        let created = Utc::now();
        let updated = Utc::now();

        let metadata = Metadata {
            id,
            directory: PathBuf::from("."),
            title: String::from("Test Project"),
            description: Some(String::from("A test project description")),
            categories: vec![category],
            languages: vec![language],
            build_systems: vec![build_system],
            preferred_ide: Some(ide),
            repository_url: Some(String::from("https://github.com/test/project")),
            created,
            updated,
        };

        // redefine everything to avoid implementing clone
        let category = Category::new("Application");
        let language = Language::new("Rust");
        let build_system = BuildSystem::new("Cargo");
        let ide = Ide::new("Neovim");

        let metadata_dto: MetadataDto = Metadata {
            id,
            directory: PathBuf::from("."),
            title: String::from("Test Project"),
            description: Some(String::from("A test project description")),
            categories: vec![category],
            languages: vec![language],
            build_systems: vec![build_system],
            preferred_ide: Some(ide),
            repository_url: Some(String::from("https://github.com/test/project")),
            created,
            updated,
        }
        .into();

        assert_eq!(metadata.id, metadata_dto.id);
        assert_eq!(metadata.title, metadata_dto.title);
        assert_eq!(metadata.description, metadata_dto.description);
        assert_eq!(metadata.categories, metadata_dto.categories);
        assert_eq!(metadata.languages, metadata_dto.languages);
        assert_eq!(metadata.build_systems, metadata_dto.build_systems);
        assert_eq!(metadata.preferred_ide, metadata_dto.preferred_ide);
        assert_eq!(metadata.repository_url, metadata_dto.repository_url);
        assert_eq!(metadata.created, metadata_dto.created);
        assert_eq!(metadata.updated, metadata_dto.updated);
    }
}
