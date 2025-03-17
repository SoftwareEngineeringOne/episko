#![deny(clippy::pedantic)]
#![allow(clippy::used_underscore_binding)]
use episko_lib::{
    config::ConfigHandler,
    database::DatabaseHandler,
    files::File as _,
    metadata::{Metadata, metadata_handler::MetadataHandler},
};
use tauri::Manager;

mod commands;
use commands::{get_all, get_with_id, load_from_directory, load_from_file};
use tokio::sync::Mutex;

struct AppState {
    pub db: DatabaseHandler,
    pub config_handler: ConfigHandler,
}
impl AppState {
    pub fn new(db: DatabaseHandler, config_handler: ConfigHandler) -> Self {
        Self { db, config_handler }
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
            load_from_file,
            load_from_directory,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
