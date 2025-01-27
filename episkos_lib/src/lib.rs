//! # Episkos Library
//!
//! This library provides tools for handling project metadata, including reading, writing,
//! and manipulating metadata files.
//!
//! ## Features
//! - **Files**: Enabled with the `files` feature, includes file handling and serialization utilities.
//! - **Metadata**: Core utilities for managing project metadata.#[cfg(feature = "files")]

#[cfg(feature = "files")]
pub mod files;
#[macro_use]
pub mod metadata;

pub fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}
