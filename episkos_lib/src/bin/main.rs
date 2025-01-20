use episkos_lib::metadata::Metadata;
use std::{error::Error, path::Path};

fn main() -> Result<(), Box<dyn Error>> {
    let metadata = Metadata::builder()
        .title("Hello, World!")
        .directory(Path::new("./"))?
        .build()?;
    let path = Path::new(".");
    println!("Path: {:#?}", path);
    println!("Absolut: {:#?}", path.canonicalize()?);

    println!("Metadata: {:#?}", metadata.directory);

    metadata.write_manifest()?;

    let read_metadata = Metadata::from_manifest(Path::new("."))?;
    println!("Read: {:#?}", read_metadata);

    Ok(())
}
