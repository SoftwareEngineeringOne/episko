use std::{error::Error, path::Path};

use episkos_lib::{
    files::File,
    metadata::Metadata,
};

fn main() -> Result<(), Box<dyn Error>> {
    // let categories = vec![Category::new("Cool"), Category::new("University")];
    // let languages = vec![
    //     Language::new("Rust", Some("1.84")),
    //     Language::new("Typescript", None),
    // ];
    // let build_systems = vec![
    //     BuildSystem::new("Cargo", Some("1.83")),
    //     BuildSystem::new("Bun", Some("1.1.43")),
    // ];
    // let ide = Ide::new("neovim");
    // let metadata = Metadata::builder()
    //     .title("Hello, World!")
    //     .directory(Path::new("./"))?
    //     .languages(languages)
    //     .categories(categories)
    //     .build_systems(build_systems)
    //     .preffered_ide(ide)
    //     .description("SoftwareEngineering Project")
    //     .build()?;
    //
    // println!("Metadata hash: {}", metadata.get_hash()?);
    //
    // metadata.write_file(&metadata.directory())?;
    //
    let metadata = Metadata::from_file(Path::new("./manifest.toml"))?;

    Ok(())
}
