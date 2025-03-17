#![deny(clippy::pedantic)]
#![allow(clippy::used_underscore_binding)]
use chrono::{DateTime, Utc};
use episko_lib::{
    config::ConfigHandler,
    database::DatabaseHandler,
    files::File as _,
    metadata::{BuildSystem, Category, Ide, Language, Metadata, metadata_handler::MetadataHandler},
};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use tauri::Manager;

mod commands;
use commands::{get_all, get_with_id, load_from_directory, load_from_file, update_metadata};
use std::path::PathBuf;
use tokio::sync::Mutex;
use uuid::Uuid;

struct AppState {
    pub db: DatabaseHandler,
    pub config_handler: ConfigHandler,
}
impl AppState {
    pub fn new(db: DatabaseHandler, config_handler: ConfigHandler) -> Self {
        Self { db, config_handler }
    }
}

/// Dto data transfer object
#[derive(Serialize, Deserialize, Debug)]
struct MetadataDto {
    id: Uuid,
    directory: PathBuf,
    title: String,
    description: Option<String>,
    categories: Vec<Category>,
    languages: Vec<Language>,
    build_systems: Vec<BuildSystem>,
    preffered_ide: Option<Ide>,
    repository_url: Option<String>,
    created: DateTime<Utc>,
    updated: DateTime<Utc>,
}

/// Dco data creation object
#[derive(Serialize, Deserialize, Debug)]
struct MetadataDco {
    directory: PathBuf,
    title: String,
    description: Option<String>,
    categories: Vec<Category>,
    languages: Vec<Language>,
    build_systems: Vec<BuildSystem>,
    preffered_ide: Option<Ide>,
    repository_url: Option<String>,
}

impl MetadataDco {
    fn create() -> Result<Metadata, String> {
        todo!()
    }
}

impl From<Metadata> for MetadataDto {
    fn from(metadata: Metadata) -> MetadataDto {
        MetadataDto {
            id: metadata.id,
            directory: metadata.directory,
            title: metadata.title,
            description: metadata.description,
            categories: metadata.categories,
            languages: metadata.languages,
            build_systems: metadata.build_systems,
            preffered_ide: metadata.preffered_ide,
            repository_url: metadata.repository_url,
            created: metadata.created,
            updated: metadata.updated,
        }
    }
}

/// !TODO!
///
/// # Errors
/// !TODO!
///
/// # Panics
/// !TODO!
///
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let config_handler = ConfigHandler::load()?;

    let db = DatabaseHandler::with_config(config_handler.config()).await?;

    let files = config_handler.files();
    for file in files {
        Metadata::from_file(file)?.write_to_db(&db).await?;
    }

    let dirs = config_handler.dirs();
    for dir in dirs {
        let files = MetadataHandler::search_directory(dir)?;
        for file in &files {
            Metadata::from_file(file)?.write_to_db(&db).await?;
        }
    }

    tauri::async_runtime::set(tokio::runtime::Handle::current());

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            app.manage(Mutex::new(AppState::new(db, config_handler)));
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_all,
            get_with_id,
            update_metadata,
            load_from_file,
            load_from_directory,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
