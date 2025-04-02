//! # Metrics of all projects

use crate::statistics::Statistic;
use sqlx::Row;

use super::DatabaseHandler;
use crate::database::Error;
use std::collections::HashMap;

impl Statistic {
    /// Retrieve the project count sorted by language.
    ///
    /// # Errors
    /// - if the database query fails
    pub async fn projects_by_language(db: &DatabaseHandler) -> Result<HashMap<String, u32>, Error> {
        Self::count_projects(
            db,
            "SELECT language.name AS name, count(language.name) AS count
             FROM metadata
             JOIN rel_metadata_language
             ON metadata.id=rel_metadata_language.metadata_id
             JOIN language
             ON rel_metadata_language.language_id=language.id
             GROUP BY language.name;"
                .to_string(),
        )
        .await
    }

    /// Retrieve the project count sorted by IDE.
    ///
    /// # Errors
    /// - if the database query fails
    pub async fn projects_by_ide(db: &DatabaseHandler) -> Result<HashMap<String, u32>, Error> {
        Self::count_projects(
            db,
            "SELECT ide.name AS name, count(ide.name) AS count
             FROM metadata
             JOIN ide
             ON metadata.preferred_ide=ide.id
             GROUP BY ide.name;"
                .to_string(),
        )
        .await
    }

    /// Retrieve the project count sorted by category.
    ///
    /// # Errors
    /// - if the database query fails
    pub async fn projects_by_category(db: &DatabaseHandler) -> Result<HashMap<String, u32>, Error> {
        Self::count_projects(
            db,
            "SELECT category.name AS name, count(category.name) AS count
             FROM metadata
             JOIN rel_metadata_category
             ON metadata.id=rel_metadata_category.metadata_id
             JOIN category
             ON rel_metadata_category.category_id=category.id
             GROUP BY category.name;"
                .to_string(),
        )
        .await
    }

    /// Retrieve the project count sorted by build system
    ///
    /// # Errors
    /// - if the database query fails
    pub async fn projects_by_build_system(
        db: &DatabaseHandler,
    ) -> Result<HashMap<String, u32>, Error> {
        Self::count_projects(
            db,
            "SELECT build_system.name AS name, count(build_system.name) AS count
             FROM metadata
             JOIN rel_metadata_build_system
             ON metadata.id=rel_metadata_build_system.metadata_id
             JOIN build_system
             ON rel_metadata_build_system.build_system_id=build_system.id
             GROUP BY build_system.name;"
                .to_string(),
        )
        .await
    }

    /// Retrieve the total count of all projects.
    ///
    /// # Errors
    /// - if the database query fails
    pub async fn number_of_projects(db: &DatabaseHandler) -> Result<u32, Error> {
        let row = sqlx::query(
            "SELECT count(id) AS count
             FROM metadata;",
        )
        .fetch_one(db.conn())
        .await?;

        Ok(row.try_get("count")?)
    }

    /// Retrieve the total count of all languages.
    ///
    /// # Errors
    /// - if the database query fails
    pub async fn number_of_languages(db: &DatabaseHandler) -> Result<u32, Error> {
        let row = sqlx::query(
            "SELECT count(DISTINCT name) AS count
             FROM language;",
        )
        .fetch_one(db.conn())
        .await?;

        Ok(row.try_get("count")?)
    }

    /// Execute the given query and return the formatted result.
    ///
    /// # Errors
    /// - if the database query fails
    async fn count_projects(
        db: &DatabaseHandler,
        query: String,
    ) -> Result<HashMap<String, u32>, Error> {
        let row = sqlx::query(&query).fetch_all(db.conn()).await?;

        let mut counted_projects = HashMap::new();

        for el in row {
            counted_projects.insert(el.try_get("name")?, el.try_get("count")?);
        }

        Ok(counted_projects)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        database::db_test::fill_db,
        metadata::{property::Property, BuildSystem, Category, Ide, Language, Metadata},
    };

    use super::*;
    use sqlx::SqlitePool;

    static METADATA_AMOUNT: u32 = 10;
    async fn fill_db_for_statistics(db: &DatabaseHandler) {
        let bs1 = BuildSystem::new("Cargo");
        let bs2 = BuildSystem::new("CMake");

        let cat1 = Category::new("cool");
        let cat2 = Category::new("test");

        let lang1 = Language::new("Rust");
        let lang2 = Language::new("Go");

        let ide1 = Ide::new("neovim");
        let ide2 = Ide::new("emacs");

        for i in 0..METADATA_AMOUNT {
            let mut builder = Metadata::builder()
                .title(&format!("test_{i}"))
                .directory(".")
                .build_systems(vec![bs1.clone(), bs2.clone()])
                .categories(vec![cat1.clone(), cat2.clone()])
                .languages(vec![lang1.clone(), lang2.clone()]);

            builder = if i < (METADATA_AMOUNT / 2) {
                builder.preferred_ide(ide1.clone())
            } else {
                builder.preferred_ide(ide2.clone())
            };

            builder.build().unwrap().write_to_db(&db).await.unwrap();
        }
    }

    async fn test_projects_by<F>(conn: SqlitePool, projects_by: F, expected_per_key: u32)
    where
        F: AsyncFnOnce(&DatabaseHandler) -> Result<HashMap<String, u32>, Error>,
    {
        let db = DatabaseHandler::with_conn(conn);

        fill_db_for_statistics(&db).await;

        let result = projects_by(&db).await.unwrap();

        for (_, value) in result {
            assert_eq!(value, expected_per_key)
        }
    }

    #[sqlx::test]
    async fn test_projects_by_build_system(conn: SqlitePool) {
        test_projects_by(conn, Statistic::projects_by_build_system, METADATA_AMOUNT).await;
    }

    #[sqlx::test]
    async fn test_projects_by_language(conn: SqlitePool) {
        test_projects_by(conn, Statistic::projects_by_language, METADATA_AMOUNT).await;
    }

    #[sqlx::test]
    async fn test_projects_by_category(conn: SqlitePool) {
        test_projects_by(conn, Statistic::projects_by_category, METADATA_AMOUNT).await;
    }

    #[sqlx::test]
    async fn test_projects_by_ide(conn: SqlitePool) {
        test_projects_by(conn, Statistic::projects_by_ide, METADATA_AMOUNT / 2).await;
    }

    #[sqlx::test]
    async fn test_number_of_projects(conn: SqlitePool) {
        let db = DatabaseHandler::with_conn(conn);

        for i in 0..METADATA_AMOUNT {
            let result = Statistic::number_of_projects(&db).await.unwrap();

            assert_eq!(result, i);

            fill_db(1, &db).await;
        }

        let result = Statistic::number_of_projects(&db).await.unwrap();
        assert_eq!(result, METADATA_AMOUNT);
    }

    #[sqlx::test]
    async fn test_number_of_languages(conn: SqlitePool) {
        let db = DatabaseHandler::with_conn(conn);

        for i in 0..METADATA_AMOUNT {
            let result = Statistic::number_of_languages(&db).await.unwrap();
            assert_eq!(result, i);

            Metadata::builder()
                .title(&format!("test_{i}"))
                .directory(".")
                .add_language(Language::new(&format!("lang_{i}")))
                .build()
                .unwrap()
                .write_to_db(&db)
                .await
                .unwrap();
        }

        let result = Statistic::number_of_languages(&db).await.unwrap();
        assert_eq!(result, METADATA_AMOUNT);
    }
}
