use sqlx::{
    query::{self, Query},
    sqlite::{SqliteArguments, SqlitePoolOptions},
    SqliteConnection, SqliteExecutor, SqlitePool,
};

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

    pub fn conn(&self) -> &SqlitePool {
        &self.conn
    }
}

impl Metadata {
    const REL_INSERT_QUERY: &str = "INSERT INTO rel_metadata_{}(metadata_id, {}_id) VALUES(?, ?)";
    const METADATA_INSERT_QUERY: &str = "
        INSERT INTO metadata(
            id, directory, title, description, 
            preferred_ide, repository_url, created, updated
        ) VALUES(?, ?, ?, ?, ?, ?, ?, ?)";

    pub async fn write_to_db(&self, db: &DatabaseHandler) -> Result<()> {
        let mut transaction = db.conn.begin().await?;

        // Handle preferred IDE relationship
        self.handle_relation(&mut transaction, &self.preffered_ide)
            .await?;

        // Insert main metadata
        self.insert_metadata(&mut transaction).await?;

        // Handle all many-to-many relationships
        self.handle_relations(&mut transaction, "category", &self.categories)
            .await?;
        self.handle_relations(&mut transaction, "language", &self.languages)
            .await?;
        self.handle_relations(&mut transaction, "build_system", &self.build_systems)
            .await?;

        transaction.commit().await?;
        Ok(())
    }

    async fn handle_relation<T: DatabaseObject>(
        &self,
        executor: &mut SqliteConnection,
        relation: &Option<T>,
    ) -> Result<()> {
        if let Some(item) = relation {
            if !item.exists(&mut *executor).await? {
                item.write_to_db(executor).await?;
            }
        }
        Ok(())
    }

    async fn handle_relations<T: DatabaseObject + Property>(
        &self,
        executor: &mut SqliteConnection,
        relation_type: &str,
        items: &[T],
    ) -> Result<()> {
        let query_str = Self::REL_INSERT_QUERY.replace("{}", relation_type);

        for item in items {
            if !item.exists(&mut *executor).await? {
                item.write_to_db(&mut *executor).await?;
            }

            sqlx::query(&query_str)
                .bind(&self.id)
                .bind(item.generate_id().as_slice())
                .execute(&mut *executor)
                .await?;
        }
        Ok(())
    }

    async fn insert_metadata(&self, executor: &mut SqliteConnection) -> Result<()> {
        let directory_str = self.directory.to_str();
        let ide_id = self
            .preffered_ide
            .as_ref()
            .map(|ide| ide.generate_id().to_vec());

        sqlx::query(Self::METADATA_INSERT_QUERY)
            .bind(&self.id)
            .bind(directory_str)
            .bind(&self.title)
            .bind(&self.description)
            .bind(ide_id)
            .bind(&self.repository_url)
            .bind(self.created)
            .bind(self.updated)
            .execute(executor)
            .await?;

        Ok(())
    }
}
