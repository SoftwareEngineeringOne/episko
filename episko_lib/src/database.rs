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
use thiserror::Error;

pub mod database_handler;
pub mod database_object;

pub mod insert_metadata;
pub mod remove_metadata;
pub mod retrieve_metadata;
pub mod update_metadata;
pub mod validate_stored_metadata;

pub use database_handler::DatabaseHandler;
pub use database_object::DatabaseObject;
use uuid::Uuid;

/// Result type for this module using [`enum@Error`]
pub type Result<T> = std::result::Result<T, Error>;

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
}

#[cfg(test)]
pub async fn setup_db() -> DatabaseHandler {
    use sqlx::migrate::Migrator;
    static MIGRATOR: Migrator = sqlx::migrate!();

    let db = DatabaseHandler::new("sqlite://").await.unwrap();
    MIGRATOR.run(db.conn()).await.unwrap();

    db
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn setup_test_db() {
        setup_db().await;
    }
}
