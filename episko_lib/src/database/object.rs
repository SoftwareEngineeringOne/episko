use std::{future::Future, pin::Pin};

use super::Result;
use sqlx::{sqlite::SqliteRow, FromRow, SqliteExecutor};

pub type BoxedFuture<'r, T> = Pin<Box<dyn Future<Output = T> + Send + 'r>>;
pub use episko_derive::DatabaseObject;

pub trait DatabaseObject: Sized + for<'r> FromRow<'r, SqliteRow> {
    type Id;

    fn write_to_db<'e>(
        &'e self,
        executor: impl SqliteExecutor<'e> + 'e,
    ) -> BoxedFuture<'e, Result<()>>;

    fn from_db<'e>(
        id: Self::Id,
        executor: impl SqliteExecutor<'e> + 'e,
    ) -> BoxedFuture<'e, Result<Self>>;

    fn exists<'e>(
        &'e self,
        executor: impl SqliteExecutor<'e> + 'e,
    ) -> BoxedFuture<'e, Result<bool>>;

    fn remove_from_db<'e>(
        &'e self,
        executor: impl SqliteExecutor<'e> + 'e,
    ) -> BoxedFuture<'e, Result<()>>;
}
