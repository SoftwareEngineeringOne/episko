use crate::metadata::{self, Metadata};

use super::{file_handler::FileHandler, File};

impl File for Metadata {
    type Error = metadata::Error;

    fn write_file(&self, path: &std::path::Path) -> Result<(), Self::Error> {
        Ok(FileHandler::write_file(self, path)?)
    }

    fn from_file(path: &std::path::Path) -> Result<Self, Self::Error> {
        let path = match path.is_file() {
            true => path.to_path_buf(),
            false => path.join("manifest.toml"),
        }
        .canonicalize()?;

        let mut metadata: Metadata = FileHandler::read_file(&path)?;
        metadata.update_directory(path);
        Ok(metadata)
    }
}
