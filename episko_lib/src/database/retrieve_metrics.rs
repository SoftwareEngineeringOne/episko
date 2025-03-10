//! # Metrics of all projects

use crate::statistics::Statistic;
use sqlx::Row;

use super::DatabaseHandler;
use crate::database::Error;
use std::collections::HashMap;

impl Statistic {
    /// Retrieve the project count sorted by language.
    pub async fn projects_by_language(db: &DatabaseHandler) -> Result<HashMap<String, u32>, Error> {
        return Ok(Self::count_projects(
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
        .await?);
    }

    /// Retrieve the project count sorted by IDE.
    pub async fn projects_by_ide(db: &DatabaseHandler) -> Result<HashMap<String, u32>, Error> {
        return Ok(Self::count_projects(
            db,
            "SELECT ide.name AS name, count(ide.name) AS count
             FROM metadata
             JOIN ide
             ON metadata.preferred_ide=ide.id
             GROUP BY ide.name;"
                .to_string(),
        )
        .await?);
    }

    /// Retrieve the project count sorted by category.
    pub async fn projects_by_category(db: &DatabaseHandler) -> Result<HashMap<String, u32>, Error> {
        return Ok(Self::count_projects(
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
        .await?);
    }

    /// Retrieve the project count sorted by build system
    pub async fn projects_by_build_system(
        db: &DatabaseHandler,
    ) -> Result<HashMap<String, u32>, Error> {
        return Ok(Self::count_projects(
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
        .await?);
    }

    /// Retrieve the total count of all projects.
    pub async fn number_of_projects(db: &DatabaseHandler) -> Result<u32, Error> {
        let row = sqlx::query(
            "SELECT count(id) AS count
             FROM metadata;",
        )
        .fetch_one(db.conn())
        .await?;

        return Ok(row.try_get("count")?);
    }

    /// Execute the given query and return the formatted result.
    async fn count_projects(
        db: &DatabaseHandler,
        query: String,
    ) -> Result<HashMap<String, u32>, Error> {
        let row = sqlx::query(&query).fetch_all(db.conn()).await?;

        let mut counted_projects = HashMap::new();

        for el in row {
            counted_projects.insert(el.try_get("name")?, el.try_get("count")?);
        }

        return Ok(counted_projects);
    }
}
