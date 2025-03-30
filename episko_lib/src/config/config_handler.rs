use std::collections::HashSet;
use std::path::Path;
use std::{env, fs, path::PathBuf};

use crate::{config::CONFIG_FILE_NAME, files::File};

use super::{Config, Error, Result, DIR_NAME};

/// The [`ConfigHandler`] is used to load and save the [`Config`] object
/// to a file as defined in [`ConfigHandler::config_path`].
pub struct ConfigHandler {
    config_path: PathBuf,
    config: Config,
}

impl ConfigHandler {
    /// Create a new [`ConfigHandler`].
    ///
    /// # Errors
    /// - [`Error::Io`] when creating the config directory fails
    /// - Propogates errors from [`ConfigHandler::get_config_dir`].
    pub fn load() -> Result<Self> {
        let config_path = Self::get_config_dir()?;
        if !config_path.exists() {
            fs::create_dir_all(&config_path)?;
        }

        let config = ConfigHandler::load_config(&config_path)?;

        Ok(Self {
            config_path,
            config,
        })
    }

    #[must_use]
    pub fn config(&self) -> &Config {
        &self.config
    }

    #[must_use]
    pub fn files(&self) -> &HashSet<PathBuf> {
        &self.config.files_to_load
    }

    #[must_use]
    pub fn dirs(&self) -> &HashSet<PathBuf> {
        &self.config.directories_to_load
    }

    /// !TODO!
    ///
    /// # Errors
    /// !TODO!
    pub fn save_config(&self) -> Result<()> {
        let config_file = self.config_path.join(CONFIG_FILE_NAME);

        Ok(self.config.write_file(&config_file)?)
    }

    /// Add a file to the config which will automatically be loaded
    /// when running the GUI application.
    ///
    /// Duplicate entries will be ignored.
    pub fn add_saved_file(&mut self, file: &Path) {
        self.config.files_to_load.insert(file.to_path_buf());
    }

    /// Remove a file from the config, which will no longer be automatically
    /// loaded when running the GUI application.
    ///
    /// Returns `false` when the entry can't be removed, implying it doesn't
    /// exist.
    pub fn remove_saved_file(&mut self, file: &Path) -> bool {
        self.config.files_to_load.remove(&file.to_path_buf())
    }

    /// Add a directory to the config, whose underlying manifest files will automatically be loaded
    /// recursively when running the GUI application.
    ///
    /// Duplicate entries will be ignored.
    pub fn add_saved_directory(&mut self, dir: &Path) {
        self.config.directories_to_load.insert(dir.to_path_buf());
    }

    /// Remove a directory from the config, which will no longer be automatically
    /// searched when running the GUI application.
    ///
    /// Returns `false` when the entry can't be removed, implying it doesn't
    /// exist.
    pub fn remove_saved_directory(&mut self, dir: &Path) -> bool {
        self.config.directories_to_load.remove(&dir.to_path_buf())
    }

    /// Load a config from the path saved in the receiver instance.
    /// If no config file exists a default will be created.
    ///
    /// # Errors
    /// - Propogates errors from [`Config::try_default`]
    /// - Propogates errors from [`Config::from_file`]
    fn load_config(config_path: &Path) -> Result<Config> {
        let config_file = config_path.join(CONFIG_FILE_NAME);

        let config = if config_file.exists() {
            Config::from_file(&config_file)?
        } else {
            let config = Config::try_default()?;
            config.write_file(&config_file)?;
            config
        };

        Ok(config)
    }

    /// Retrieve the default directory for the config location.
    ///
    /// On Unix-like systems the directory is located at:
    /// - `$XDG_CONFIG_DIR/episko` if `$XDG_CONFIG_DIR` is set
    /// - `$HOME/.config/episko` otherwise
    ///
    /// On Windows systems the config directory is located at:
    /// - `%APPDATA%\episko`
    ///
    /// # Errors
    /// - [`Error::Directory`] if none of the relevant environmental variables are set
    /// - [`Error::UnknownOs`] if the os is not windows or unix-like
    fn get_config_dir() -> Result<PathBuf> {
        #[cfg(unix)]
        {
            env::var("XDG_CONFIG_DIR")
                .map(PathBuf::from)
                .map(|p| p.join(DIR_NAME))
                .or_else(|_| {
                    env::var("HOME")
                        .map(PathBuf::from)
                        .map(|p| p.join(".config").join(DIR_NAME))
                })
                .map_err(|_| Error::Directory)
        }

        #[cfg(windows)]
        {
            env::var("APPDATA")
                .map(PathBuf::from)
                .map(|p| p.join(DIR_NAME))
                .map_err(|_| Error::Directory)
        }

        #[cfg(not(any(unix, windows)))]
        {
            Err(Error::UnknownOs(env::consts::OS.to_string()))
        }
    }

    /// for tests only
    #[doc(hidden)]
    #[must_use]
    pub fn in_place() -> Self {
        let config_path = PathBuf::from(".");

        let config = Self::load_config(&config_path).expect("load config for test");

        Self {
            config_path,
            config,
        }
    }
}
