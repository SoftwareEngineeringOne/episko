use std:: path::{Path, PathBuf};

use super::Config;

pub struct ConfigController{
    path: PathBuf,
    config: Config,
}

impl ConfigController {

    pub fn load_config() -> Result<Config, Error> {
        Ok(self.config)
    }

    pub fn add_saved_file(file: &Path) -> Result<(), Error> {
        Ok(())
    }
    
    pub fn add_saved_directory(dir: &Path) -> Result<(), Error> {
        Ok(())
    }
}