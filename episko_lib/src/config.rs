use std::path::{Path, PathBuf};
use thiserror::Error;

pub mod config_handler;

pub type Result<T> = std::result::Result<T, Error>;

pub struct Config {
    pub directories_to_load: Vec<PathBuf>,
    pub files_to_load: Vec<PathBuf>,
    pub database_path: PathBuf,
}

impl Config {
    pub fn add_saved_file(&mut self, file: &Path) -> Result<()> {
        Ok(())
    }

    pub fn add_saved_directory(&mut self, dir: &Path) -> Result<()> {
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("tbc")]
    Generic,
}

