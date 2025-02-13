//! Module containing the [`DatabaseObject`] trait.
use std::{future::Future, pin::Pin};

use super::Result;
use sqlx::{sqlite::SqliteRow, FromRow, SqliteExecutor};

pub type BoxedFuture<'r, T> = Pin<Box<dyn Future<Output = T> + Send + 'r>>;
pub use episko_derive::DatabaseObject;

/// This trait should be implemented on sub-properties of [`crate::metadata::Metadata`]
/// such as [`crate::metadata::Category`].
///
/// It provides a simple interface for database operations, used to simplify
/// operations on [`crate::metadata::Metadata`].
///
/// An `update` method is omitted on purpose. E.g. categories should be editable,
/// instead new ones should be created/old ones removed.
///
/// The trait can be implemented using macros from the `episko_derive` crate.
///
/// # Example
/// ```ignore
/// use episko_lib::database::DatabaseObject;
/// use sqlx::FromRow;
///
/// #[derive(DatabaseObject, FromRow)]
/// #[db(table = "example_property")]
/// struct ExampleProperty {
///     #[db(col = "id")]
///     id: u32, // a field with the column name "id" required
///     #[db(col = "name")]
///     name: String, // all other fields are optional
///     #[db(col = "version")]
///     version: Option<String>,
/// }
/// ```
pub trait DatabaseObject: Sized + for<'r> FromRow<'r, SqliteRow> {
    type Id;

    /// Write the given instance of the object to the database.
    fn write_to_db<'e>(
        &'e self,
        executor: impl SqliteExecutor<'e> + 'e,
    ) -> BoxedFuture<'e, Result<()>>;

    /// Retrieve the given object from the database.
    fn from_db<'e>(
        id: Self::Id,
        executor: impl SqliteExecutor<'e> + 'e,
    ) -> BoxedFuture<'e, Result<Self>>;

    /// Check if the given object exists in the database
    fn exists<'e>(
        &'e self,
        executor: impl SqliteExecutor<'e> + 'e,
    ) -> BoxedFuture<'e, Result<bool>>;

    /// Remove the given object from the database
    fn remove_from_db<'e>(
        &'e self,
        executor: impl SqliteExecutor<'e> + 'e,
    ) -> BoxedFuture<'e, Result<()>>;
}
