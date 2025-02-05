use std::{error::Error, path::Path};

use episko_lib::{
    files::File,
    metadata::{property::Property, BuildSystem, Category, Ide, Language, Metadata},
};

fn main() -> Result<(), Box<dyn Error>> {
    let categories = vec![Category::new("Cool"), Category::new("University")];
    let languages = vec![
        Language::with_version("Rust", "1.84"),
        Language::new("Typescript"),
    ];
    let build_systems = vec![
        BuildSystem::with_version("Cargo", "1.83"),
        BuildSystem::with_version("Bun", "1.1.43"),
    ];
    let ide = Ide::new("neovim");
    let metadata = Metadata::builder()
        .title("Hello, World!")
        .directory("./")
        .languages(languages)
        .categories(categories)
        .build_systems(build_systems)
        .preffered_ide(ide)
        .description("SoftwareEngineering Project")
        .build()?;

    metadata.write_file(&metadata.directory())?;

    let metadata = Metadata::from_file(Path::new("./manifest.toml"))?;

    Ok(())
}
