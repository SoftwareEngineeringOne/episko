use std::marker::PhantomData;
use thiserror::Error;

use sqlx::{sqlite::SqlitePoolOptions, SqliteExecutor, SqlitePool};

mod database_handler;

#[derive(Debug, Error)]
enum Error {
    #[error("Database error")]
    Db(#[from] sqlx::Error),

    #[error("Environment error")]
    Env(#[from] dotenvy::Error),
}
