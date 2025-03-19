use sqlx::query;
use sqlx::Row;

use crate::metadata::Metadata;

use super::{DatabaseHandler, Error, Result};

impl Metadata {
    /// Validate the hash of a given metadata and update
    /// the cache if not up to date.
    ///
    /// !TODO!
    ///
    /// # Errors
    /// !TODO!
    pub async fn validate_db(&self, db: &DatabaseHandler) -> Result<bool> {
        let checksum = self
            .get_hash()
            .map_err(|err| Error::Checksum(err.to_string()))?
            .to_vec();

        let row = query("SELECT checksum FROM metadata WHERE id = ?")
            .bind(self.id)
            .fetch_one(db.conn())
            .await?;

        let checksum_db: Vec<u8> = row.try_get("checksum")?;

        Ok(checksum == checksum_db)
    }

    /// !TODO!
    ///
    /// # Errors
    /// !TODO!
    pub async fn is_cached(&self, db: &DatabaseHandler) -> Result<bool> {
        let row = query("SELECT count(id) as count FROM metadata WHERE id = ?")
            .bind(self.id)
            .fetch_one(db.conn())
            .await?;

        let count: u8 = row.try_get("count")?;

        Ok(count == 1)
    }
}
