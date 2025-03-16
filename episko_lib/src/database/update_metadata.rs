use crate::metadata::property::Property;
use crate::metadata::Metadata;

use super::DatabaseHandler;
use super::DatabaseObject;
use super::Error;
use super::Result;

impl Metadata {
    /// Update a given metadata.
    ///
    /// For simplicity reasons the object is basically replaced completly even if
    /// not all properties have changed.
    ///
    /// # Errors
    /// Returns [Ok] when the item was updated.
    pub async fn update_in_db(&self, db: &DatabaseHandler) -> Result<()> {
        let mut transaction = db.conn().begin().await?;

        let ide_id = self
            .preffered_ide
            .as_ref()
            .map(|ide| ide.generate_id().to_vec());

        sqlx::query(
            "UPDATE metadata
             SET directory = ?,
                 title = ?,
                 description = ?,
                 preferred_ide = ?,
                 repository_url = ?,
                 created = ?,
                 updated = ?,
                 checksum = ?
             WHERE id = ?",
        )
        .bind(self.directory.to_str())
        .bind(&self.title)
        .bind(&self.description)
        .bind(&ide_id)
        .bind(&self.repository_url)
        .bind(self.created)
        .bind(self.updated)
        .bind(
            self.get_hash()
                .map_err(|err| Error::Checksum(err.to_string()))?
                .to_vec(),
        )
        .bind(self.id)
        .execute(&mut *transaction)
        .await?;

        sqlx::query("DELETE FROM rel_metadata_category WHERE metadata_id = ?")
            .bind(self.id)
            .execute(&mut *transaction)
            .await?;
        for category in &self.categories {
            // Make sure the related category exists.
            if !category.exists(&mut *transaction).await? {
                category.write_to_db(&mut *transaction).await?;
            }
            sqlx::query("INSERT INTO rel_metadata_category(metadata_id, category_id) VALUES(?, ?)")
                .bind(self.id)
                .bind(category.generate_id().as_slice())
                .execute(&mut *transaction)
                .await?;
        }

        sqlx::query("DELETE FROM rel_metadata_language WHERE metadata_id = ?")
            .bind(self.id)
            .execute(&mut *transaction)
            .await?;
        for language in &self.languages {
            if !language.exists(&mut *transaction).await? {
                language.write_to_db(&mut *transaction).await?;
            }
            sqlx::query("INSERT INTO rel_metadata_language(metadata_id, language_id) VALUES(?, ?)")
                .bind(self.id)
                .bind(language.generate_id().as_slice())
                .execute(&mut *transaction)
                .await?;
        }

        sqlx::query("DELETE FROM rel_metadata_build_system WHERE metadata_id = ?")
            .bind(self.id)
            .execute(&mut *transaction)
            .await?;
        for build_system in &self.build_systems {
            if !build_system.exists(&mut *transaction).await? {
                build_system.write_to_db(&mut *transaction).await?;
            }
            sqlx::query(
                "INSERT INTO rel_metadata_build_system(metadata_id, build_system_id) VALUES(?, ?)",
            )
            .bind(self.id)
            .bind(build_system.generate_id().as_slice())
            .execute(&mut *transaction)
            .await?;
        }

        transaction.commit().await?;
        Ok(())
    }
}
