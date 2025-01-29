use camino::Utf8PathBuf;
use color_eyre::Result;
use episkos_lib::{files::File, metadata::Metadata};

pub fn validate_manifest(file: &Utf8PathBuf) -> Result<()> {
    Ok(Metadata::validate_file(file.as_std_path())?)
}

pub fn cache_manifest(_file: &Utf8PathBuf) -> Result<()> {
    todo!()
}
