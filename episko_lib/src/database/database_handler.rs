//! Submodule of [`crate::database`] for the [`DatabaseHandler`]
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

use super::Result;

/// This struct is used to initialize and manage
/// the connection to the database using a [`SqlitePool`] instance.
pub struct DatabaseHandler {
    conn: SqlitePool,
}

impl DatabaseHandler {
    /// Creates a new instance using the provided url.
    pub async fn new(url: &str) -> Result<Self> {
        let conn = SqlitePoolOptions::new()
            .max_connections(1)
            .connect(url)
            .await?;
        Ok(Self { conn })
    }

    /// Create a new instance based on the default behaviour.
    /// This currently means using the environmental variable `DATABASE_URL`.
    ///
    /// Later on the default behaviour should be managed by the a config.
    pub async fn default() -> Result<Self> {
        let connection_str = dotenvy::var("DATABASE_URL")?;
        Self::new(&connection_str).await
    }

    /// Provides a reference to the [`SqlitePool`] which can be used
    /// as a [`sqlx::sqlite::SqliteExecutor`].
    pub fn conn(&self) -> &SqlitePool {
        &self.conn
    }
}
