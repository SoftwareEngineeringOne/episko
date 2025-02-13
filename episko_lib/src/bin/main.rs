use std::{error::Error, path::Path, str::FromStr};

use episko_lib::{
    database::{DatabaseHandler, DatabaseObject},
    files::File,
    metadata::{property::Property, BuildSystem, Category, Ide, Language, Metadata},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let connection_str = String::from_str("sqlite://episko.db")?;

    // println!("Connecting..");
    // let conn = SqlitePoolOptions::new()
    //     .max_connections(1)
    //     .connect(&connection_str)
    //     .await?;

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

    let db = DatabaseHandler::default().await?;
    metadata.write_to_db(&db).await?;
    metadata.write_file(Path::new("./manifest1.toml"))?;

    let metadata3 = Metadata::from_file(Path::new("./manifest1.toml"))?;
    println!("Read metadata from file: {:#?}", metadata3);

    let id = metadata.id();

    let hash1 = metadata.get_hash()?;
    println!("Written metadata with hash: {:#?}", hash1);

    println!("Retrieving from db...");

    let metadata2 = Metadata::from_db(&db, id).await?;
    let hash2 = metadata2.get_hash()?;
    metadata2.write_file(Path::new("./manifest2.toml"))?;

    println!("Received metadata with hash: {:#?}", hash2);

    println!("Hashes are equal: {}", hash1 == hash2);

    Ok(())
}
