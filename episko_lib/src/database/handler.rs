use sqlx::{query, sqlite::SqlitePoolOptions, SqlitePool};

use crate::metadata::{build_system, property::Property, BuildSystem, Metadata};

use super::{DatabaseObject, Result};

pub struct DatabaseHandler {
    conn: SqlitePool,
}

impl DatabaseHandler {
    pub async fn new(url: &str) -> Result<Self> {
        let conn = SqlitePoolOptions::new()
            .max_connections(1)
            .connect(url)
            .await?;
        Ok(Self { conn })
    }

    pub async fn default() -> Result<Self> {
        let connection_str = dotenvy::var("DATABASE_URL")?;
        Self::new(&connection_str).await
    }

    pub async fn insert_metadata(&self, metadata: Metadata) -> Result<()> {
        // 1 - m Relations
        if let Some(ref ide) = metadata.preffered_ide {
            if !ide.exists(&self.conn).await? {
                ide.write_to_db(&self.conn).await?;
            }
        }

        self.change_my_name_bitch(&metadata).await?;

        // n - m Relations
        for category in metadata.categories.iter() {
            if !category.exists(&self.conn).await? {
                category.write_to_db(&self.conn).await?;
            }
            query("INSERT INTO rel_metadata_category(metadata_id, category_id) VALUES(?, ?)")
                .bind(metadata.id)
                .bind(category.generate_id().to_vec())
                .execute(&self.conn)
                .await?;
        }

        for language in metadata.languages.iter() {
            if !language.exists(&self.conn).await? {
                language.write_to_db(&self.conn).await?;
            }
            query("INSERT INTO rel_metadata_language(metadata_id, language_id) VALUES(?, ?)")
                .bind(metadata.id)
                .bind(language.generate_id().to_vec())
                .execute(&self.conn)
                .await?;
        }

        for build_system in metadata.build_systems.iter() {
            if !build_system.exists(&self.conn).await? {
                build_system.write_to_db(&self.conn).await?;
            }
            query(
                "INSERT INTO rel_metadata_build_system(metadata_id, build_system_id) VALUES(?, ?)",
            )
            .bind(metadata.id)
            .bind(build_system.generate_id().to_vec())
            .execute(&self.conn)
            .await?;
        }
        Ok(())
    }

    async fn change_my_name_bitch(&self, metadata: &Metadata) -> Result<()> {
        let ide_id = match &metadata.preffered_ide {
            Some(ide) => Some(ide.generate_id().to_vec()),
            None => None,
        };
        query("
            INSERT INTO 
                metadata(id, directory, title, description, preferred_ide, repository_url, created, updated)
            VALUES(?, ?, ?, ?, ?, ?, ?, ?)
            ")
            .bind(metadata.id)
            .bind(metadata.directory.to_str())
            .bind(metadata.title.clone())
            .bind(metadata.description.clone())
            .bind(ide_id)
            .bind(metadata.repository_url.clone())
            .bind(metadata.created)
            .bind(metadata.updated)
            .execute(&self.conn).await?;

        Ok(())
    }
}
