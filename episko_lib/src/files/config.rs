use crate::config::Config;

use super::{file_handler::FileHandler, File, Result};

impl File for Config {
    fn write_file(&self, path: &std::path::Path) -> Result<()> {
        FileHandler::write_file(self, path)
    }

    fn from_file(path: &std::path::Path) -> Result<Self> {
        FileHandler::read_file(path)
    }
}
