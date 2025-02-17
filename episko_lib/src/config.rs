//! This module contains everything relevant regarding the config
//! used by the gui and cli application.
//!
//! # Config file
//! The config file is used to define the path of the cache database and
//! which directories and files should be loaded by the gui application.
//!
//! It can be edited manually, using the cli or using the gui.
//!
//! By default it is placed as defined in [`ConfigHandler::get_config_dir`].
//!
//! The serialiaztion and deserialization of the file is done using
//! [`serde`] and the [`files::File`] trait.
use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    env, fs, io,
    path::{Path, PathBuf},
};
use thiserror::Error;

use crate::files;

pub mod config_handler;
pub use config_handler::ConfigHandler;

pub static DIR_NAME: &str = "episko";
pub static DB_FILE_NAME: &str = "cache.db";
pub static CONFIG_FILE_NAME: &str = "config.toml";

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub(crate) database_path: PathBuf,
    pub directories_to_load: HashSet<PathBuf>,
    pub files_to_load: HashSet<PathBuf>,
}

impl Config {
    /// Try to create a [`Config`] object using the default path
    /// for the database directory.
    ///
    /// # Errors
    /// - [`Error::Directory`] when the parent directory of the db can't be found
    /// - [`Error::Io`] when creating a non existent db directory fails
    /// - Propogates errors from [`Config::generate_db_path`]
    pub fn try_default() -> Result<Self> {
        let database_path = Self::generate_db_path()?;
        let database_dir = database_path.parent().ok_or(Error::Directory)?;

        if !database_dir.exists() {
            fs::create_dir_all(database_dir)?;
        }

        Ok(Self {
            database_path,
            directories_to_load: HashSet::default(),
            files_to_load: HashSet::default(),
        })
    }

    /// Add a file to the config which will automatically be loaded
    /// when running the GUI application.
    ///
    /// Duplicate entries will be ignored.
    pub fn add_saved_file(&mut self, file: &Path) {
        self.files_to_load.insert(file.to_path_buf());
    }

    /// Remove a file from the config, which will no longer be automatically
    /// loaded when running the GUI application.
    ///
    /// Returns `false` when the entry can't be removed, implying it doesn't
    /// exist.
    pub fn remove_saved_file(&mut self, file: &Path) -> bool {
        self.files_to_load.remove(&file.to_path_buf())
    }

    /// Add a directory to the config, whose underlying manifest files will automatically be loaded
    /// recursively when running the GUI application.
    ///
    /// Duplicate entries will be ignored.
    pub fn add_saved_directory(&mut self, dir: &Path) {
        self.directories_to_load.insert(dir.to_path_buf());
    }

    /// Remove a directory from the config, which will no longer be automatically
    /// searched when running the GUI application.
    ///
    /// Returns `false` when the entry can't be removed, implying it doesn't
    /// exist.
    pub fn remove_saved_directory(&mut self, dir: &Path) -> bool {
        self.directories_to_load.remove(&dir.to_path_buf())
    }

    /// Generate the default path for where the sqlite database should be located.
    /// On Unix-like systems it is placed in `$XDG_CACHE_HOME/episko/cache.db` or
    /// `$HOME/.cache/episko/cache.db` if the former is not set.
    /// On Windows systems the database will be placed in `%LOCALAPPDATA%/episko/cache.db`
    ///
    /// # Errors
    /// - [`Error::EnvironmentVar`] when `XDG_CACHE_HOME` or `LOCALAPPDATA` are not set
    /// - [`Error::UnknownOs] when not on a Unix-like or Windows system
    fn generate_db_path() -> Result<PathBuf> {
        #[cfg(unix)]
        {
            env::var("XDG_CACHE_HOME")
                .map(PathBuf::from)
                .map(|p| p.join(DIR_NAME).join(DB_FILE_NAME))
                .or_else(|_| {
                    env::var("HOME")
                        .map(PathBuf::from)
                        .map(|p| p.join(".cache").join(DIR_NAME).join(DB_FILE_NAME))
                })
                .map_err(|_| Error::EnvironmentVar)
        }

        #[cfg(windows)]
        {
            env::var("LOCALAPPDATA")
                .map(PathBuf::from)
                .map(|p| p.join(DIR_NAME).join(DB_FILE_NAME))
                .map_err(|_| Error::EnvironmentVar)
        }

        #[cfg(not(any(unix, windows)))]
        {
            Err(Error::UnknownOs(env::consts::OS.to_string()))
        }
    }
}

/// Errors used within the [`crate::config`] module.
#[derive(Debug, Error)]
pub enum Error {
    #[error("unknown os: `{0}")]
    UnknownOs(String),

    #[error("unable to find environmental variables")]
    EnvironmentVar,

    #[error("unable to find config dir")]
    Directory,

    #[error("an io error occured")]
    Io(#[from] io::Error),

    #[error("failed to interact with file")]
    File(#[from] files::Error),
}
