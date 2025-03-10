use std::{
    error::Error,
    path::{Path, PathBuf},
    str::FromStr,
};

use episko_lib::{
    config::{config_handler::ConfigHandler, Config},
    database::{DatabaseHandler, DatabaseObject},
    files::File,
    metadata::{
        metadata_handler::MetadataHandler, property::Property, BuildSystem, Category, Ide,
        Language, Metadata,
    },
};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let connection_str = String::from_str("sqlite://episko.db")?;

    let ch = ConfigHandler::new()?;

    let mut config = ch.load_config()?;
    let db = DatabaseHandler::with_config(&config).await?;

    let mut metadata_handler = MetadataHandler::new();

    let dir = PathBuf::from("/home/simon/2_Uni");

    // let locations = MetadataHandler::search_directory(&dir)?;
    // println!("Found manifest at: ");
    // for manifest in locations.iter() {
    //     println!("\t{}", manifest.display());
    // }
    // println!("Total of {} manifests found.", locations.len());
    //
    // metadata_handler.load_from_config(&config)?;
    //
    // println!(
    //     "Loaded {} manifests from config!",
    //     metadata_handler.loaded_metadata.len(),
    // );
    //
    // ch.save_config(&config)?;

    // let db = DatabaseHandler::new(&connection_str).await?;

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
    let id = Uuid::from_str("c4aec118-dc18-46e8-9a2f-3dfbc2b5da40")?;
    let metadata = Metadata::builder()
        .id(id)
        .title("Hello, World!")
        .directory("./")
        .languages(languages)
        .categories(categories)
        .build_systems(build_systems)
        .preffered_ide(ide)
        .description("SoftwareEngineering Project")
        .build()?;

    let equal = metadata.validate_db(&db).await?;
    println!("Database is up to date: {}", equal);
    println!("Updating...");
    metadata.update_in_db(&db).await?;
    let equal = metadata.validate_db(&db).await?;
    println!("Database is up to date: {}", equal);

    // metadata.write_to_db(&db).await?;

    // let metadata3 = Metadata::from_file(Path::new("./manifest1.toml"))?;
    // println!("Read metadata from file: {:#?}", metadata3);
    //
    // let id = metadata.id();
    //
    // let hash1 = metadata.get_hash()?;
    // println!("Written metadata with hash: {:#?}", hash1);
    //
    // println!("Retrieving from db...");
    //
    // let metadata2 = Metadata::from_db(&db, id).await?;
    // let hash2 = metadata2.get_hash()?;
    // metadata2.write_file(Path::new("./manifest2.toml"))?;
    //
    // println!("Received metadata with hash: {:#?}", hash2);
    //
    // println!("Hashes are equal: {}", hash1 == hash2);
    //
    Ok(())
}
