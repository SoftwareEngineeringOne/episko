use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

use super::Error;

pub struct DatabaseHandler {
    conn: SqlitePool,
}

impl DatabaseHandler {
    pub async fn new(url: &str) -> Result<Self, Error> {
        let conn = SqlitePoolOptions::new()
            .max_connections(1)
            .connect(url)
            .await?;
        Ok(Self { conn })
    }

    pub async fn default() -> Result<Self, Error> {
        let connection_str = dotenvy::var("DATABASE_URL")?;
        Self::new(&connection_str).await
    }
}
