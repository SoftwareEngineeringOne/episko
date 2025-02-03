use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

use super::Result;

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
