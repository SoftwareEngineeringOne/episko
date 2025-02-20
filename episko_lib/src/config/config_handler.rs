use std::{env, fs, path::PathBuf};

use crate::{config::CONFIG_FILE_NAME, files::File};

use super::{Config, Error, Result, DIR_NAME};

/// The [`ConfigHandler`] is used to load and save the [`Config`] object
/// to a file as defined in [`ConfigHandler::config_path`].
pub struct ConfigHandler {
    pub config_path: PathBuf,
}

impl ConfigHandler {
    /// Create a new [`ConfigHandler`].
    ///
    /// # Errors
    /// - [`Error::Io`] when creating the config directory fails
    /// - Propogates errors from [`ConfigHandler::get_config_dir`].
    pub fn new() -> Result<Self> {
        let config_path = Self::get_config_dir()?;
        if !config_path.exists() {
            fs::create_dir_all(&config_path)?;
        }

        Ok(Self { config_path })
    }

    /// Load a config from the path saved in the receiver instance.
    /// If no config file exists a default will be created.
    ///
    /// # Errors
    /// - Propogates errors from [`Config::try_default`]
    /// - Propogates errors from [`Config::from_file`]
    pub fn load_config(&self) -> Result<Config> {
        let config_file = self.config_path.join(CONFIG_FILE_NAME);

        if !config_file.exists() {
            let config = Config::try_default()?;
            config.write_file(&config_file)?;
            return Ok(config);
        }

        Ok(Config::from_file(&config_file)?)
    }

    pub fn save_config(&self, config: &Config) -> Result<()> {
        let config_file = self.config_path.join(CONFIG_FILE_NAME);

        Ok(config.write_file(&config_file)?)
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
}
