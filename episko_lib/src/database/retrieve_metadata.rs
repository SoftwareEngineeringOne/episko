use super::{DatabaseHandler, DatabaseObject, Error, Result};
use crate::metadata::{BuildSystem, Category, Ide, Language, Metadata};

use chrono::DateTime;
use sqlx::Row;
use std::str::FromStr;
use uuid::Uuid;

impl Metadata {
    pub async fn from_db(db: &DatabaseHandler, id: Uuid) -> Result<Self> {
        let row = sqlx::query(
            "SELECT id, directory, title, description, preferred_ide, repository_url, created, updated 
             FROM metadata 
             WHERE id = ?",
        )
        .bind(id)
        .fetch_one(db.conn())
        .await?;
        build_metadata_from_row(&row, db).await
    }

    /// This is very bad and slow :(
    pub async fn all_from_db(db: &DatabaseHandler) -> Result<Vec<Self>> {
        let rows = sqlx::query(
            "SELECT id, directory, title, description, preferred_ide, repository_url, created, updated 
             FROM metadata",
        )
        .fetch_all(db.conn())
        .await?;

        // Iterate through all rows and build Metadata objects.
        let mut metadatas = Vec::with_capacity(rows.len());
        for row in rows {
            let metadata = build_metadata_from_row(&row, db).await;
            match metadata {
                Ok(metadata) => metadatas.push(metadata),
                Err(err) => match err {
                    Error::NotFound(id) => {
                        Metadata::remove_non_existent_from_db(id, db).await?;
                    }
                    _ => return Err(err),
                },
            }
        }
        Ok(metadatas)
    }
}

async fn build_metadata_from_row(
    row: &sqlx::sqlite::SqliteRow,
    db: &DatabaseHandler,
) -> Result<Metadata> {
    let id: Vec<u8> = row.try_get("id")?;
    let directory: String = row.try_get("directory")?;
    let title: String = row.try_get("title")?;
    let description: Option<String> = row.try_get("description")?;
    let preferred_ide_id: Option<Vec<u8>> = row.try_get("preferred_ide")?;
    let repository_url: Option<String> = row.try_get("repository_url")?;
    let created: String = row.try_get("created")?;
    let updated: String = row.try_get("updated")?;

    let categories = load_categories(db, &id).await?;
    let languages = load_languages(db, &id).await?;
    let build_systems = load_build_systems(db, &id).await?;

    let preffered_ide = if let Some(ide_id) = preferred_ide_id {
        Some(Ide::from_db(ide_id, db.conn()).await?)
    } else {
        None
    };

    let id = Uuid::from_slice(&id)?;

    let mut builder = Metadata::builder()
        .id(id)
        .directory(&directory)
        .title(&title)
        .created(DateTime::from_str(&created)?)
        .updated(DateTime::from_str(&updated)?)
        .categories(categories)
        .languages(languages)
        .build_systems(build_systems);

    if let Some(desc) = description {
        builder = builder.description(&desc);
    }
    if let Some(url) = repository_url {
        builder = builder.repository_url(&url);
    }
    if let Some(ide) = preffered_ide {
        builder = builder.preffered_ide(ide);
    }

    Ok(builder.build().map_err(|_| Error::NotFound(id))?)
}

async fn load_categories(db: &DatabaseHandler, metadata_id: &[u8]) -> Result<Vec<Category>> {
    let rows = sqlx::query("SELECT category_id FROM rel_metadata_category WHERE metadata_id = ?")
        .bind(metadata_id)
        .fetch_all(db.conn())
        .await?;
    let mut cats = Vec::new();
    for row in rows {
        let cat_id: Vec<u8> = row.try_get("category_id")?;
        let cat = Category::from_db(cat_id, db.conn()).await?;
        cats.push(cat);
    }
    Ok(cats)
}

async fn load_languages(db: &DatabaseHandler, metadata_id: &[u8]) -> Result<Vec<Language>> {
    let rows = sqlx::query("SELECT language_id FROM rel_metadata_language WHERE metadata_id = ?")
        .bind(metadata_id)
        .fetch_all(db.conn())
        .await?;
    let mut langs = Vec::new();
    for row in rows {
        let lang_id: Vec<u8> = row.try_get("language_id")?;
        let lang = Language::from_db(lang_id, db.conn()).await?;
        langs.push(lang);
    }
    Ok(langs)
}

async fn load_build_systems(db: &DatabaseHandler, metadata_id: &[u8]) -> Result<Vec<BuildSystem>> {
    let rows =
        sqlx::query("SELECT build_system_id FROM rel_metadata_build_system WHERE metadata_id = ?")
            .bind(metadata_id)
            .fetch_all(db.conn())
            .await?;
    let mut bss = Vec::new();
    for row in rows {
        let bs_id: Vec<u8> = row.try_get("build_system_id")?;
        let bs = BuildSystem::from_db(bs_id, db.conn()).await?;
        bss.push(bs);
    }
    Ok(bss)
}
