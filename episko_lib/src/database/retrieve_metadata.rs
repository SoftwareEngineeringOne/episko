use super::{DatabaseHandler, DatabaseObject, Result};
use crate::metadata::{BuildSystem, Category, Ide, Language, Metadata};
use chrono::DateTime;
use sqlx::Row;
use std::str::FromStr;
use uuid::Uuid;

impl Metadata {
    /// Retrieve a metadata object based on it's id.
    pub async fn from_db(db: &DatabaseHandler, id: Uuid) -> Result<Self> {
        let row = sqlx::query(
            "SELECT id, directory, title, description, preferred_ide, repository_url, created, updated 
             FROM metadata 
             WHERE id = ?",
        )
        .bind(id)
        .fetch_one(db.conn())
        .await?;

        let id: Vec<u8> = row.try_get("id")?;
        let directory: String = row.try_get("directory")?;
        let title: String = row.try_get("title")?;
        let description: Option<String> = row.try_get("description")?;
        let preferred_ide_id: Option<Vec<u8>> = row.try_get("preferred_ide")?;
        let repository_url: Option<String> = row.try_get("repository_url")?;
        let created: String = row.try_get("created")?;
        let updated: String = row.try_get("updated")?;

        // Load many-to-many relationships.
        let categories = {
            let rows =
                sqlx::query("SELECT category_id FROM rel_metadata_category WHERE metadata_id = ?")
                    .bind(&id)
                    .fetch_all(db.conn())
                    .await?;
            let mut cats = Vec::new();
            for row in rows {
                let cat_id: Vec<u8> = row.try_get("category_id")?;
                let cat = Category::from_db(cat_id, db.conn()).await?;
                cats.push(cat);
            }
            cats
        };

        let languages = {
            let rows =
                sqlx::query("SELECT language_id FROM rel_metadata_language WHERE metadata_id = ?")
                    .bind(&id)
                    .fetch_all(db.conn())
                    .await?;
            let mut langs = Vec::new();
            for row in rows {
                let lang_id: Vec<u8> = row.try_get("language_id")?;
                let lang = Language::from_db(lang_id, db.conn()).await?;
                langs.push(lang);
            }
            langs
        };

        let build_systems = {
            let rows = sqlx::query(
                "SELECT build_system_id FROM rel_metadata_build_system WHERE metadata_id = ?",
            )
            .bind(&id)
            .fetch_all(db.conn())
            .await?;
            let mut bss = Vec::new();
            for row in rows {
                let bs_id: Vec<u8> = row.try_get("build_system_id")?;
                let bs = BuildSystem::from_db(bs_id, db.conn()).await?;
                bss.push(bs);
            }
            bss
        };

        // Retrieve the preferred IDE (if any).
        let preffered_ide = if let Some(ide_id) = preferred_ide_id {
            Some(Ide::from_db(ide_id, db.conn()).await?)
        } else {
            None
        };

        let mut builder = Metadata::builder()
            .id(Uuid::from_slice(&id)?)
            .directory(&directory)
            .title(&title)
            .created(DateTime::from_str(&created)?)
            .updated(DateTime::from_str(&updated)?)
            .categories(categories)
            .languages(languages)
            .build_systems(build_systems);

        if let Some(description) = description {
            builder = builder.description(&description);
        };

        if let Some(url) = repository_url {
            builder = builder.repository_url(&url);
        };

        if let Some(ide) = preffered_ide {
            builder = builder.preffered_ide(ide);
        }

        Ok(builder.build()?)
    }
}
