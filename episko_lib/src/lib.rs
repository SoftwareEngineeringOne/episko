// #![deny(clippy::pedantic)]
//! # Episko Library
//!
//! This library is part of the projekt [Episko](https://github.com/SoftwareEngineeringOne/episko).
//! It provides functionality and interfaces to create and manage
//! metadata for projects.
//!
//! ## Structure
//! The library is structured into the following modules:
//! - metadata
//! - files
//! - database
//!
//! The metadata module is part of the core crate, while the files module
//! is placed under the "files" feature flag, which is however enabled by
//! default.
//!
//! Disabling default features may lead to errors, as currently macros
//! are used in the core crate that don't work without some of the
//! features.
//! This needs to be adressed at a later point in the project.
//!
//! Detailed documentation can be found within each module.

pub mod config;
#[cfg(feature = "database")]
pub mod database;
#[cfg(feature = "files")]
pub mod files;
pub mod metadata;
pub mod config;

/// Trait to perform a self consuming action based on a condition.
pub trait ApplyIf: Sized {
    #[must_use]
    fn apply_if<T>(self, value: Option<T>, f: fn(Self, T) -> Self) -> Self;
}

/// Placeholder used as a command by tauri
#[must_use]
pub fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}
