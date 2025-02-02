use thiserror::Error;

pub mod handler;
pub mod object;
pub use object::DatabaseObject;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Database error")]
    Db(#[from] sqlx::Error),

    #[error("Environment error")]
    Env(#[from] dotenvy::Error),
}
