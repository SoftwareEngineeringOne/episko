use thiserror::Error;

pub mod database_handler;
pub mod database_object;

mod insert_metadata;
mod remove_metadata;
mod retrieve_metadata;
mod update_metadata;
mod validate_stored_metadata;

pub use database_handler::DatabaseHandler;
pub use database_object::DatabaseObject;

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
}
