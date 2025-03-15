//! Submodule of [`crate::database`] for the [`DatabaseHandler`]
use std::time::Duration;

use sqlx::{
    SqlitePool,
    migrate::{MigrateDatabase, Migrator},
    sqlite::SqlitePoolOptions,
};

use crate::config::Config;

use super::Result;

static MIGRATOR: Migrator = sqlx::migrate!();

/// This struct is used to initialize and manage
/// the connection to the database using a [`SqlitePool`] instance.
#[derive(Debug)]
pub struct DatabaseHandler {
    conn: SqlitePool,
}

impl DatabaseHandler {
    pub async fn with_config(config: &Config) -> Result<Self> {
        let url = format!(
            "sqlite:///{}",
            config.database_path.to_str().unwrap_or_default()
        );

        Self::new(&url).await
    }
    /// Creates a new instance using the provided url.
    pub async fn new(url: &str) -> Result<Self> {
        if !sqlx::Sqlite::database_exists(url).await? {
            sqlx::Sqlite::create_database(url).await?;
        }

        let conn = SqlitePoolOptions::new()
            .max_connections(12)
            .acquire_timeout(Duration::from_secs(5))
            .connect(url)
            .await?;

        MIGRATOR.run(&conn).await?;

        Ok(Self { conn })
    }

    /// Provides a reference to the [`SqlitePool`] which can be used
    /// as a [`sqlx::sqlite::SqliteExecutor`].
    pub fn conn(&self) -> &SqlitePool {
        &self.conn
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn create_mem_db() {
        let _ = DatabaseHandler::new("sqlite://").await.unwrap();
    }
}
