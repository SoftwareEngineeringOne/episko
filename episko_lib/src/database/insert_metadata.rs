use super::{DatabaseHandler, DatabaseObject, Error, Result};
use crate::metadata::{Metadata, property::Property};
use sqlx::SqliteConnection;

impl Metadata {
    const REL_INSERT_QUERY: &str = "INSERT INTO rel_metadata_{}(metadata_id, {}_id) VALUES(?, ?)";
    const METADATA_INSERT_QUERY: &str = "
        INSERT OR REPLACE INTO metadata(
            id, directory, title, description, 
            preferred_ide, repository_url, created, updated, checksum
        ) VALUES(?, ?, ?, ?, ?, ?, ?, ?, ?)";

    /// Write a [`Metadata`] instance to the database.
    ///
    /// All sub-properties will also be inserted if they don't exist yet.
    /// Furthermore all relation tables will also be updated accordingly.
    ///
    /// # Errors
    /// - [`super::Error`]
    pub async fn write_to_db(&self, db: &DatabaseHandler) -> Result<()> {
        let mut transaction = db.conn().begin().await?;

        // Handle preferred IDE relationship
        self.handle_relation(&mut transaction, self.preffered_ide.as_ref())
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
        relation: Option<&T>,
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
                .bind(self.id)
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
            .bind(self.id)
            .bind(directory_str)
            .bind(&self.title)
            .bind(&self.description)
            .bind(ide_id)
            .bind(&self.repository_url)
            .bind(self.created)
            .bind(self.updated)
            .bind(
                self.get_hash()
                    .map_err(|err| Error::Checksum(err.to_string()))?
                    .to_vec(),
            )
            .execute(executor)
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::database::setup_db;

    #[sqlx::test]
    async fn insert_metadata() {
        // let db = DatabaseHandler::new("sqlite://").await.unwrap();
        let db = setup_db().await;

        let metadata = Metadata::builder()
            .directory(".")
            .title("Test")
            .build()
            .unwrap();

        metadata.write_to_db(&db).await.unwrap();
    }
}
