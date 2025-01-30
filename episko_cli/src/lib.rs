//! # Library of the CLI
//! 
//! This module contains little help functions.

use color_eyre::{eyre::eyre, Result};

pub mod cli;
pub mod creation;
pub mod removal;
pub mod validation;

pub use creation::create_manifest;
pub use removal::remove_manifest;
pub use validation::{cache_manifest, validate_manifest};

pub trait ComplexArg {
    fn parse_tuple(self) -> Result<(String, String)>;
}

impl ComplexArg for String {
    fn parse_tuple(self) -> Result<(String, String)> {
        let parts: Vec<&str> = self.split(":").collect();

        // Name has to be given, version is optional
        match parts.len() {
            1 => Ok((parts[0].to_string(), String::default())),
            2 => Ok((parts[0].to_string(), parts[1].to_string())),
            _ => Err(eyre!("invalid input")),
        }
    }
}
