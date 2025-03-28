//! # Database Module
//!
//! This module contains code regarding the interaction with
//! the database, which is used as a persistent cache.
//!
//! ## [`DatabaseHandler`]
//!
//! The [`DatabaseHandler`] is mainly used for establishing and managing
//! the connection to the sqlite database using [`sqlx::sqlite::SqlitePool`].
//!
//! ## [`DatabaseObject`]
//!
//! The [`DatabaseObject`] is implemented by sub-properties of [`crate::metadata::Metadata`]
//! such as [`crate::metadata::Category`].
//! The implementation can be done via a macro defined in the `episko_derive` crate.
//!
//! ## [`crate::metadata::Metadata`]
//!
//! The database functions regarding [`crate::metadata::Metadata`] are implemented directly
//! on the struct.
//! The implementations can be found in the submodules:
//! - [`insert_metadata`]
//! - [`retrieve_metadata`]
//! - [`update_metadata`]
//! - [`remove_metadata`]
//! - [`validate_stored_metadata`]
use serde::Deserialize;
use thiserror::Error;

pub mod database_handler;
pub mod database_object;

pub mod insert_metadata;
pub mod remove_metadata;
pub mod retrieve_metadata;
pub mod update_metadata;
pub mod validate_stored_metadata;

mod dao;

pub use database_handler::DatabaseHandler;
pub use database_object::DatabaseObject;
use uuid::Uuid;

/// Result type for this module using [`enum@Error`]
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Deserialize, Clone)]
pub struct Filter {
    pub query: Option<String>,
    pub language: Option<String>,
    pub category: Option<String>,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Db(#[from] sqlx::Error),

    #[error("Environment error")]
    Env(#[from] dotenvy::Error),

    #[error("DateTime error")]
    ParseDateTime(#[from] chrono::format::ParseError),

    #[error("Uuid error")]
    Uuid(#[from] uuid::Error),

    #[error("Build error")]
    Build(#[from] crate::metadata::builder::Error),

    #[error("Migration error")]
    Migration(#[from] sqlx::migrate::MigrateError),

    #[error("unable to create checksum: {0}")]
    Checksum(String),

    #[error("manifest was not found")]
    NotFound(Uuid),

    #[error("async: {0}")]
    Async(String),

    #[error("failed to create metadata instance: {0}")]
    Conversion(#[from] dao::ConversionError),
}

#[cfg(test)]
pub mod db_test {
    use std::collections::HashSet;

    use super::*;
    use crate::{
        metadata::{property::Property as _, Metadata, *},
        ApplyIf as _,
    };
    use chrono::{TimeDelta, Utc};

    pub async fn fill_db(amount: usize, db: &DatabaseHandler) {
        let test_data = generate_test_metadata(amount);

        for el in test_data {
            el.write_to_db(db).await.expect("writing test data");
        }
    }

    pub fn generate_test_metadata(count: usize) -> Vec<Metadata> {
        let base_time = Utc::now();
        let ides = ["VSCode", "IntelliJ", "Sublime", "Vim"];
        let categories = ["Web", "CLI", "GUI", "Embedded", "AI"];
        let languages = ["Rust", "Python", "JavaScript", "Go", "C++"];
        let build_systems = ["Cargo", "Make", "CMake", "NPM", "Bazel"];

        (0..count)
            .map(|i| {
                let offset = TimeDelta::try_days(i as i64).unwrap();

                Metadata::builder()
                    .title(&format!("Test Project {}", i + 1))
                    .directory(".")
                    .apply_if(
                        Some(categories[i % categories.len()]),
                        MetadataBuilder::add_category,
                    )
                    .add_language(Language::with_version(
                        languages[i % languages.len()],
                        &format!("1.{}", i),
                    ))
                    .apply_if(
                        (i % 2 == 0).then_some(Ide::new(ides[i % ides.len()])),
                        MetadataBuilder::preferred_ide,
                    )
                    .add_build_system(BuildSystem::with_version(
                        build_systems[i % build_systems.len()],
                        &format!("2.{}", i),
                    ))
                    .apply_if(
                        (i % 3 == 0).then_some("Sample description"),
                        MetadataBuilder::description,
                    )
                    .apply_if(
                        (i % 4 == 0).then_some("https://github.com/example/project"),
                        MetadataBuilder::repository_url,
                    )
                    .created(base_time - offset)
                    .updated(base_time)
                    .build()
                    .expect("Should generate valid metadata")
            })
            .collect()
    }

    #[test]
    fn test_generated_metadata() {
        let test_data = generate_test_metadata(5);
        assert_eq!(test_data.len(), 5);

        let ids: HashSet<Uuid> = test_data.iter().map(|m| m.id).collect();
        assert_eq!(ids.len(), 5);
    }
}

#[cfg(test)]
mod test {
    use sqlx::SqlitePool;

    use crate::database::{db_test::fill_db, DatabaseHandler};

    #[sqlx::test]
    async fn setup_test_db(conn: SqlitePool) {
        let db = DatabaseHandler::with_conn(conn);
        fill_db(25, &db).await;
    }
}
