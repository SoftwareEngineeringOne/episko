use super::{
    dao::{ConversionError, MetadataDao, MetadataPreviewDao},
    DatabaseHandler, Filter, Result,
};
use crate::metadata::{Metadata, MetadataPreview};
use sqlx::{QueryBuilder, Row};

use uuid::Uuid;

impl Metadata {
    /// Retrieves a single [`Metadata`] entry by ID from the database
    ///
    /// # Errors
    /// Returns `Err` if database query fails or data conversion fails
    pub async fn from_db(db: &DatabaseHandler, id: Uuid) -> Result<Self> {
        let query = build_query(QueryFilter::Id, None);
        let dao: MetadataDao = sqlx::query_as(&query).bind(id).fetch_one(db.conn()).await?;

        Ok(dao.try_into()?)
    }

    /// Retrieves paginated [`Metadata`] entries from database
    ///
    /// # Errors
    /// Returns `Err` if database query fails or data conversion fails
    pub async fn all_from_db(
        pagination: Option<Pagination>,
        db: &DatabaseHandler,
    ) -> Result<Vec<Self>> {
        let query = build_query(QueryFilter::None, pagination.as_ref());
        let mut query = sqlx::query_as::<_, MetadataDao>(&query);

        if let Some(p) = &pagination {
            let offset = p.offset();
            query = query.bind(p.page_size).bind(offset);
        }

        let daos = query.fetch_all(db.conn()).await?;
        convert_daos(daos)
    }

    /// Retrieves paginated [`MetadataPreview`] entries with optional search
    ///
    /// # Errors
    /// Returns `Err` if database query fails or data conversion fails
    pub async fn all_preview_from_db(
        pagination: Option<Pagination>,
        filter: Filter,
        db: &DatabaseHandler,
    ) -> Result<Vec<MetadataPreview>> {
        let sql = build_query(QueryFilter::Complex(filter.clone()), pagination.as_ref());
        let mut query = sqlx::query_as::<_, MetadataPreviewDao>(&sql);

        if let Some(search) = filter.query {
            query = query.bind(format!("%{search}%"));
        }

        if let Some(category) = filter.category {
            query = query.bind(category);
        }

        if let Some(language) = filter.language {
            query = query.bind(language);
        }

        if let Some(p) = &pagination {
            query = query.bind(p.page_size).bind(p.offset());
        }

        let daos = query.fetch_all(db.conn()).await?;
        convert_daos(daos)
    }

    /// Counts total [`Metadata`] entries with optional search filter
    ///
    /// # Errors
    /// Returns `Err` if database query fails
    pub async fn amount_cached(query: Option<String>, db: &DatabaseHandler) -> Result<u32> {
        let mut builder = QueryBuilder::new("SELECT COUNT(id) as count FROM metadata");

        if let Some(search) = &query {
            builder
                .push(" WHERE title LIKE ")
                .push_bind(format!("%{search}%"));
        }

        let row = builder.build().fetch_one(db.conn()).await?;
        Ok(row.try_get("count")?)
    }
}

#[derive(Debug)]
pub struct Pagination {
    page_number: u32,
    page_size: u32,
}

impl Pagination {
    #[must_use]
    pub const fn new(page_number: u32, page_size: u32) -> Self {
        Self {
            page_number,
            page_size,
        }
    }

    #[must_use]
    pub const fn offset(&self) -> u32 {
        self.page_number.saturating_sub(1) * self.page_size
    }
}

enum QueryFilter {
    Id,
    Complex(Filter),
    None,
}

fn build_query(filter: QueryFilter, pagination: Option<&Pagination>) -> String {
    let mut query = String::from(
        r"
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
        ",
    );

    match filter {
        QueryFilter::Id => query.push_str("WHERE metadata.id = ?"),
        QueryFilter::Complex(filter) => {
            let mut sep = " WHERE";
            if filter.query.is_some() {
                query.push_str(&format!("{sep} metadata.title LIKE ?"));
                sep = " AND";
            }

            if filter.category.is_some() {
                query.push_str(&format!("{sep} category.name LIKE ?"));
                sep = " AND";
            }
            if filter.language.is_some() {
                query.push_str(&format!("{sep} language.name LIKE ?"));
            }
        }
        QueryFilter::None => {}
    }

    query.push_str(" GROUP BY metadata.id ORDER BY metadata.updated DESC");

    if pagination.is_some() {
        query.push_str(" LIMIT ? OFFSET ?");
    }

    query
}

fn convert_daos<T, U>(daos: Vec<T>) -> Result<Vec<U>>
where
    T: TryInto<U, Error = ConversionError>,
{
    daos.into_iter()
        .map(|el| Ok(el.try_into()?))
        .collect::<Result<Vec<_>>>()
}

#[cfg(test)]
mod tests {
    use crate::database::db_test::fill_db;

    use super::*;
    use sqlx::SqlitePool;
    use uuid::Uuid;

    #[sqlx::test]
    async fn test_from_db_valid_id(conn: SqlitePool) {
        let db = DatabaseHandler::with_conn(conn);

        let expected = Metadata::builder()
            .title("test")
            .directory(".")
            .build()
            .unwrap();

        expected.write_to_db(&db).await.unwrap();

        // Act
        let result = Metadata::from_db(&db, expected.id).await;

        // Assert
        // assert!(result.is_ok());
        let metadata = result.unwrap();
        assert_eq!(metadata.get_hash().unwrap(), expected.get_hash().unwrap());
    }

    #[sqlx::test]
    async fn test_from_db_invalid_id(conn: SqlitePool) {
        let db = DatabaseHandler::with_conn(conn);
        fill_db(25, &db).await;
        let invalid_id = Uuid::new_v4();

        let result = Metadata::from_db(&db, invalid_id).await;

        assert!(result.is_err());
    }

    #[sqlx::test]
    async fn test_db_pagination(conn: SqlitePool) {
        let db = DatabaseHandler::with_conn(conn);
        fill_db(50, &db).await;
        let pagination_10 = Pagination::new(1, 10);
        let pagination_20 = Pagination::new(1, 20);

        let result_10 =
            Metadata::all_preview_from_db(Some(pagination_10), Filter::default(), &db).await;
        let result_20 =
            Metadata::all_preview_from_db(Some(pagination_20), Filter::default(), &db).await;
        let result_all = Metadata::all_preview_from_db(None, Filter::default(), &db).await;

        assert!(result_10.is_ok());
        assert!(result_20.is_ok());
        assert!(result_all.is_ok());
        let metadata_10 = result_10.unwrap();
        let metadata_20 = result_20.unwrap();
        let metadata_all = result_all.unwrap();

        assert!(metadata_10.len() == 10);
        assert!(metadata_20.len() == 20);
        assert!(metadata_all.len() == 50);
    }

    #[sqlx::test]
    async fn test_db_search(conn: SqlitePool) {
        let db = DatabaseHandler::with_conn(conn);
        fill_db(25, &db).await;

        let test_metadata = Metadata::builder()
            .title("test")
            .directory(".")
            .build()
            .unwrap();

        test_metadata
            .write_to_db(&db)
            .await
            .expect("write test data with title to db");

        let search_query = "test";
        let pagination = Pagination::new(1, 10);

        let result =
            Metadata::all_preview_from_db(Some(pagination), Filter::with_query(search_query), &db)
                .await;

        assert!(result.is_ok());
        let previews = result.unwrap();

        assert!(!previews.is_empty());
        assert!(previews.len() <= 10);
    }

    #[sqlx::test]
    async fn test_amount_cached(conn: SqlitePool) {
        let db = DatabaseHandler::with_conn(conn);
        fill_db(37, &db).await;

        let result = Metadata::amount_cached(None, &db).await;

        assert!(result.is_ok());
        let amount = result.unwrap();
        assert_eq!(amount, 37);
    }

    #[sqlx::test]
    async fn test_pagination_offset() {
        let pagination = Pagination::new(2, 10);

        let offset = pagination.offset();

        assert_eq!(offset, 10);
    }

    #[sqlx::test]
    async fn test_page_number_zero(conn: SqlitePool) {
        let db = DatabaseHandler::with_conn(conn);
        fill_db(41, &db).await;
        let pagination = Pagination::new(0, 10);

        let result = Metadata::all_preview_from_db(Some(pagination), Filter::default(), &db).await;

        assert!(result.is_ok());
        let previews = result.unwrap();

        assert_eq!(previews.len(), 10)
    }
}
