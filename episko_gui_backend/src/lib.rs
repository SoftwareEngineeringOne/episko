#![deny(clippy::pedantic)]
use episko_lib::{
    config::{Config, ConfigHandler},
    database::DatabaseHandler,
    files::File as _,
    metadata::{metadata_handler::MetadataHandler, Metadata},
};
use tauri::{Builder, Manager};

mod commands;
use commands::{all, get_with_id, greet};
use tokio::sync::Mutex;

struct AppState {
    pub db: DatabaseHandler,
    pub config: Config,
    pub metadata_handler: MetadataHandler,
}
impl AppState {
    pub fn new(db: DatabaseHandler, config: Config) -> Self {
        let metadata_handler = MetadataHandler::new();
        Self {
            db,
            config,
            metadata_handler,
        }
    }
}

/// Docs will be added
///
/// # Panics
///
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let config_handler = ConfigHandler::new()?;
    let config = config_handler.load_config()?;

    let db = DatabaseHandler::with_config(&config).await?;

    let files = &config.files_to_load;
    for file in files {
        Metadata::from_file(file)?.write_to_db(&db).await?;
    }

    let dirs = &config.directories_to_load;
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
            app.manage(Mutex::new(AppState::new(db, config)));
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, all, get_with_id])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
