use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::{
    ApplyIf as _,
    metadata::{
        BuildSystem, Category, Ide, Language, Metadata, MetadataBuilder, MetadataPreview,
        property::Property,
    },
};

/// DAO: Data Access Object
#[derive(Debug, FromRow, Clone)]
pub struct MetadataDao {
    pub id: Uuid,
    pub directory: String,
    pub title: String,
    pub description: Option<String>,
    pub repository_url: Option<String>,
    pub created: String,
    pub updated: String,

    pub preferred_ide_name: Option<String>,
    pub categories: Value,
    pub languages: Value,
    pub build_systems: Value,
}

impl TryInto<Metadata> for MetadataDao {
    type Error = ConversionError;

    fn try_into(self) -> std::result::Result<Metadata, Self::Error> {
        let mut categories: Vec<Category> = serde_json::from_value(self.categories)?;
        categories.iter_mut().for_each(Property::update_id);

        let mut languages: Vec<Language> = serde_json::from_value(self.languages)?;
        languages.iter_mut().for_each(Property::update_id);

        let mut build_systems: Vec<BuildSystem> = serde_json::from_value(self.build_systems)?;
        build_systems.iter_mut().for_each(Property::update_id);

        let created = self.created.parse::<DateTime<Utc>>()?;
        let updated = self.updated.parse::<DateTime<Utc>>()?;

        let mut preferred_ide = self.preferred_ide_name.as_deref().map(Ide::new);
        preferred_ide.iter_mut().for_each(Property::update_id);

        Ok(Metadata::builder()
            .id(self.id)
            .title(&self.title)
            .directory(&self.directory)
            .categories(categories)
            .languages(languages)
            .build_systems(build_systems)
            .created(created)
            .updated(updated)
            .apply_if(preferred_ide, MetadataBuilder::preferred_ide)
            .apply_if(self.description.as_deref(), MetadataBuilder::description)
            .apply_if(
                self.repository_url.as_deref(),
                MetadataBuilder::repository_url,
            )
            .build()?)
    }
}

#[derive(Debug, FromRow, Clone)]
pub struct MetadataPreviewDao {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub created: String,
    pub updated: String,

    pub categories: Value,
    pub languages: Value,
}

impl TryInto<MetadataPreview> for MetadataPreviewDao {
    type Error = ConversionError;

    fn try_into(self) -> Result<MetadataPreview, Self::Error> {
        let mut categories: Vec<Category> = serde_json::from_value(self.categories)?;
        categories.iter_mut().for_each(Property::update_id);

        let mut languages: Vec<Language> = serde_json::from_value(self.languages)?;
        languages.iter_mut().for_each(Property::update_id);

        let created = self.created.parse::<DateTime<Utc>>()?;
        let updated = self.updated.parse::<DateTime<Utc>>()?;

        Ok(MetadataPreview {
            id: self.id,
            title: self.title,
            description: self.description,
            categories,
            languages,
            created,
            updated,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ConversionError {
    #[error(transparent)]
    Json(#[from] serde_json::Error),

    #[error(transparent)]
    Chrono(#[from] chrono::ParseError),

    #[error(transparent)]
    Builder(#[from] crate::metadata::builder::Error),
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_metadata_dao_try_into_success() {
        let dao = MetadataDao {
            id: Uuid::new_v4(),
            directory: ".".to_string(),
            title: "Test Title".to_string(),
            description: Some("Test Description".to_string()),
            repository_url: Some("https://example.com".to_string()),
            created: "2023-01-01T00:00:00Z".to_string(),
            updated: "2023-01-02T00:00:00Z".to_string(),
            preferred_ide_name: Some("Test IDE".to_string()),
            categories: json!([{ "name": "Category 1" }]),
            languages: json!([{  "name": "Language 1" }]),
            build_systems: json!([{ "name": "Build System 1" }]),
        };

        let result: Result<Metadata, ConversionError> = dao.clone().try_into();

        assert_eq!(result.is_ok(), true);
        let metadata = result.unwrap();
        assert_eq!(metadata.id, dao.id);
        assert_eq!(metadata.title, dao.title);
        assert_eq!(metadata.description.as_deref(), dao.description.as_deref());
        assert_eq!(
            metadata.repository_url.as_deref(),
            dao.repository_url.as_deref()
        );
        assert_eq!(
            metadata.preferred_ide.as_ref().map(|ide| ide.name()),
            dao.preferred_ide_name.as_deref()
        );
        assert_eq!(metadata.categories.len(), 1);
        assert_eq!(metadata.languages.len(), 1);
        assert_eq!(metadata.build_systems.len(), 1);
    }

    #[test]
    fn test_metadata_dao_try_into_invalid_date() {
        let dao = MetadataDao {
            id: Uuid::new_v4(),
            directory: ".".to_string(),
            title: "Test Title".to_string(),
            description: None,
            repository_url: None,
            created: "invalid_date".to_string(),
            updated: "2023-01-02T00:00:00Z".to_string(),
            preferred_ide_name: None,
            categories: json!([]),
            languages: json!([]),
            build_systems: json!([]),
        };

        let result: Result<Metadata, ConversionError> = dao.try_into();

        assert!(result.is_err());
        match result.unwrap_err() {
            ConversionError::Chrono(_) => {} // Expected error
            _ => panic!("Expected Chrono error"),
        }
    }

    #[test]
    fn test_metadata_dao_try_into_invalid_json() {
        let dao = MetadataDao {
            id: Uuid::new_v4(),
            directory: ".".to_string(),
            title: "Test Title".to_string(),
            description: None,
            repository_url: None,
            created: "2023-01-01T00:00:00Z".to_string(),
            updated: "2023-01-02T00:00:00Z".to_string(),
            preferred_ide_name: None,
            categories: json!("invalid_json"),
            languages: json!([]),
            build_systems: json!([]),
        };

        let result: Result<Metadata, ConversionError> = dao.try_into();

        assert!(result.is_err());
        match result.unwrap_err() {
            ConversionError::Json(_) => {} // Expected error
            _ => panic!("Expected Json error"),
        }
    }
}
