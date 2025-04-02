#![deny(clippy::pedantic)]
//! # Library of the CLI
//!
//! This module contains little help functions.

use color_eyre::{eyre::eyre, Result};

pub mod cli;
pub mod creation;
pub mod removal;
pub mod validation;

pub use creation::create_manifest;
use episko_lib::{config::Config, database::DatabaseHandler};
pub use removal::remove_manifest;
pub use validation::{cache_manifest, validate_manifest};

pub trait ComplexArg {
    /// Parse a ":" seperated, two parted argument into
    /// a [`(String, String)`] tuple.
    ///
    /// # Example
    /// ```
    /// use episko_cli::ComplexArg;
    ///
    /// let language_arg = "Rust:1.84".to_string();
    ///
    /// let tuple: (String, String) = language_arg.parse_tuple().unwrap();
    ///
    /// println!("Language: {}", tuple.0); // "Rust"
    /// println!("Version: {}", tuple.1); // "1.84"
    ///
    /// ```
    /// # Errors
    /// - [`color_eyre::Report`] when the given String does not contain 1 or 2
    ///   parts.
    fn parse_tuple(self) -> Result<(String, String)>;
}

impl ComplexArg for String {
    fn parse_tuple(self) -> Result<(String, String)> {
        let parts: Vec<&str> = self.split(':').collect();

        // Name has to be given, version is optional
        match parts.len() {
            1 => Ok((parts[0].to_string(), String::default())),
            2 => Ok((parts[0].to_string(), parts[1].to_string())),
            _ => Err(eyre!("invalid input")),
        }
    }
}

/// Connect to the cache database by creating a [`DatabaseHandler`].
///
/// # Errors
/// - Error report when connecting to the database/creating the
///   [`DatabaseHandler`] fails.
#[cfg(not(test))]
pub async fn connect_to_db(config: &Config) -> Result<DatabaseHandler> {
    Ok(DatabaseHandler::with_config(config).await?)
}

#[cfg(test)]
#[doc(hidden)]
pub async fn connect_to_db(_: &Config) -> Result<DatabaseHandler> {
    let db = DatabaseHandler::in_memory().await;

    sqlx::migrate!()
        .run(db.conn())
        .await
        .expect("setup db for tests");

    Ok(db)
}
