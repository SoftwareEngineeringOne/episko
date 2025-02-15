use serde::{Deserialize, Serialize};
use std::{
    env, fs, io,
    path::{Path, PathBuf},
};
use thiserror::Error;

use crate::files;

pub mod config_handler;

pub static DIR_NAME: &'static str = "episko";
pub static DB_FILE_NAME: &'static str = "cache.db";
pub static CONFIG_FILE_NAME: &'static str = "config.toml";

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub database_path: PathBuf,
    pub directories_to_load: Vec<PathBuf>,
    pub files_to_load: Vec<PathBuf>,
}

impl Config {
    pub fn try_default() -> Result<Self> {
        let database_path = Self::generate_db_path()?;
        let database_dir = database_path.parent().ok_or(Error::Directory)?;

        if !database_dir.exists() {
            fs::create_dir_all(database_dir)?;
        }

        Ok(Self {
            database_path,
            directories_to_load: vec![],
            files_to_load: vec![],
        })
    }

    pub fn add_saved_file(&mut self, file: &Path) {
        self.files_to_load.push(file.to_path_buf())
    }

    pub fn add_saved_directory(&mut self, dir: &Path) {
        self.directories_to_load.push(dir.to_path_buf())
    }

    fn generate_db_path() -> Result<PathBuf> {
        #[cfg(unix)]
        {
            return env::var("XDG_CACHE_HOME")
                .map(PathBuf::from)
                .map(|p| p.join(DIR_NAME).join(DB_FILE_NAME))
                .or_else(|_| {
                    env::var("HOME")
                        .map(PathBuf::from)
                        .map(|p| p.join(".cache").join(DIR_NAME).join(DB_FILE_NAME))
                })
                .map_err(|_| Error::Directory);
        }

        #[cfg(windows)]
        {
            return env::var("LOCALAPPDATA")
                .map(PathBuf::from)
                .map(|p| p.join(DIR_NAME).join(DB_FILE_NAME))
                .map_err(|_| Error::Directory);
        }

        #[cfg(not(any(unix, windows)))]
        {
            return Err(Error::UnknownOs(env::consts::OS.to_string()));
        }
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("unknown os: `{0}")]
    UnknownOs(String),

    #[error("unable to find config dir")]
    Directory,

    #[error("an io error occured")]
    Io(#[from] io::Error),

    #[error("failed to interact with file")]
    File(#[from] files::Error),
}
