#![deny(clippy::pedantic)]
//! # Episko Library
//!
//! This library is part of the projekt [Episko](https://github.com/SoftwareEngineeringOne/episkos).
//! It provides functionality and interfaces to create and manage
//! metadata for projects.
//!
//! ## Structure
//! The library is structured into the following modules:
//! - metadata
//! - files
//!
//! The metadata module is part of the core crate, while the files module
//! is placed under the "files" feature flag, which is however enabled by
//! default.
//!
//! Detailed documentation can be found within each module.

#[cfg(feature = "files")]
pub mod files;
pub mod metadata;

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
