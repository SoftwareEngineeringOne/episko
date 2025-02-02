use std::{error::Error, str::FromStr};

use episko_lib::{
    database::{handler::DatabaseHandler, object::DatabaseObject},
    metadata::{property::Property, BuildSystem, Category, Ide, Language, Metadata},
};
use sqlx::sqlite::{SqliteError, SqlitePoolOptions};

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
    db.insert_metadata(metadata).await?;

    Ok(())
}
