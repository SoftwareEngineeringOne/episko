use std::{error::Error, str::FromStr};

use episko_lib::{
    database::object::DatabaseObject,
    metadata::{property::Property, Category, Language},
};
use sqlx::sqlite::SqlitePoolOptions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let connection_str = String::from_str("sqlite://episko.db")?;

    println!("Connecting..");
    let conn = SqlitePoolOptions::new()
        .max_connections(1)
        .connect(&connection_str)
        .await?;

    let lang1 = Language::with_version("Rust", "1.84");
    let lang2 = Language::with_version("Rust", "1.83");

    let id1 = lang1.generate_id();
    let id2 = lang2.generate_id();

    lang1.remove_from_db(&conn).await?;

    let exists1 = lang1.exists(&conn).await?;
    let exists2 = lang2.exists(&conn).await?;

    println!("Language 1 exists: {}", exists1);
    println!("Language 2 exists: {}", exists2);

    // category.write_to_db(&conn).await?;

    // let categories = vec![Category::new("Cool"), Category::new("University")];
    // let languages = vec![
    //     Language::with_version("Rust", "1.84"),
    //     Language::new("Typescript"),
    // ];
    // let build_systems = vec![
    //     BuildSystem::with_version("Cargo", "1.83"),
    //     BuildSystem::with_version("Bun", "1.1.43"),
    // ];
    // let ide = Ide::new("neovim");
    // let metadata = Metadata::builder()
    //     .title("Hello, World!")
    //     .directory("./")
    //     .languages(languages)
    //     .categories(categories)
    //     .build_systems(build_systems)
    //     .preffered_ide(ide)
    //     .description("SoftwareEngineering Project")
    //     .build()?;
    //
    // metadata.write_file(&metadata.directory())?;
    //
    // let metadata = Metadata::from_file(Path::new("./manifest.toml"))?;

    Ok(())
}
