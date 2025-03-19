use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::{
    ApplyIf as _,
    metadata::{
        BuildSystem, Category, Ide, Language, Metadata, MetadataBuilder, property::Property,
    },
};

/// Data access object
/// !TODO!
#[derive(Debug, FromRow)]
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

#[derive(Debug, thiserror::Error)]
pub enum ConversionError {
    #[error(transparent)]
    Json(#[from] serde_json::Error),

    #[error(transparent)]
    Chrono(#[from] chrono::ParseError),

    #[error(transparent)]
    Builder(#[from] crate::metadata::builder::Error),
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
            .apply_if(preferred_ide, MetadataBuilder::preffered_ide)
            .apply_if(self.description.as_deref(), MetadataBuilder::description)
            .apply_if(
                self.repository_url.as_deref(),
                MetadataBuilder::repository_url,
            )
            .build()?)
    }
}
