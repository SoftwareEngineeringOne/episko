use super::{DatabaseHandler, Result, dao::MetadataDao};
use crate::metadata::Metadata;

use uuid::Uuid;

impl Metadata {
    /// !TODO!
    ///
    /// # Errors
    /// !TODO!
    pub async fn from_db(db: &DatabaseHandler, id: Uuid) -> Result<Self> {
        Ok(
            sqlx::query_as::<_, MetadataDao>(&retrieve_query(&QueryFilter::Id))
                .bind(id)
                .fetch_one(db.conn())
                .await?
                .try_into()?,
        )
    }

    /// !TODO!
    ///
    /// # Errors
    /// !TODO!
    pub async fn all_from_db(db: &DatabaseHandler) -> Result<Vec<Self>> {
        let daos: Vec<MetadataDao> =
            sqlx::query_as::<_, MetadataDao>(&retrieve_query(&QueryFilter::None))
                .fetch_all(db.conn())
                .await?;

        let metadata = daos
            .into_iter()
            .map(|dao| Ok(dao.try_into()?))
            .collect::<Result<Vec<Metadata>>>()?;

        Ok(metadata)
    }
}

enum QueryFilter {
    Id,
    None,
}

fn retrieve_query(filter: &QueryFilter) -> String {
    static BASE_QUERY: &str = "
SELECT
    metadata.id,
    metadata.directory,
    metadata.title,
    metadata.description,
    metadata.repository_url,
    metadata.created,
    metadata.updated,
    metadata.checksum,
    ide.name AS preferred_ide_name,
   COALESCE(
        json_group_array(
            DISTINCT json_object('name', category.name)
        ) FILTER(WHERE category.name IS NOT NULL),
        '[]'
    ) AS categories,
    COALESCE(
        json_group_array(
            DISTINCT json_object('name', language.name, 'version', language.version)
        ) FILTER(WHERE language.name IS NOT NULL),
        '[]'
    ) AS languages,
    COALESCE(
        json_group_array(
            DISTINCT json_object('name', build_system.name, 'version', build_system.version)
        ) FILTER(WHERE build_system.name IS NOT NULL),
        '[]'
    ) AS build_systems
FROM metadata
LEFT JOIN ide ON metadata.preferred_ide = ide.id
LEFT JOIN rel_metadata_category rmc ON metadata.id = rmc.metadata_id
LEFT JOIN category ON rmc.category_id = category.id
LEFT JOIN rel_metadata_language rml ON metadata.id = rml.metadata_id
LEFT JOIN language ON rml.language_id = language.id
LEFT JOIN rel_metadata_build_system rmbs ON metadata.id = rmbs.metadata_id
LEFT JOIN build_system ON rmbs.build_system_id = build_system.id
-- FILTER_PLACEHOLDER
GROUP BY metadata.id;
    ";

    match filter {
        QueryFilter::Id => BASE_QUERY.replace("-- FILTER_PLACEHOLDER", "WHERE metadata.id = ?"),
        QueryFilter::None => BASE_QUERY.replace("-- FILTER_PLACEHOLDER", ""),
    }
}
