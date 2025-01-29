use camino::Utf8PathBuf;
use color_eyre::Result;
use episkos_lib::{files::File, metadata::Metadata};

pub fn remove_manifest(file: &Utf8PathBuf) -> Result<()> {
    Ok(Metadata::remove_file(file.as_std_path())?)
}
