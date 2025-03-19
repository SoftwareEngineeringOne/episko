#![deny(clippy::pedantic)]
#![allow(clippy::used_underscore_binding)]
use episko_lib::{
    config::ConfigHandler,
    database::DatabaseHandler,
};
use state::AppState;
use tauri::Manager;
use tokio::sync::Mutex;

mod commands;
use commands::{
    create_metadata, get_all, get_with_id, init_cache, load_from_directory, load_from_file,
    update_metadata,
};

pub mod model;
pub mod state;

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

    tauri::async_runtime::set(tokio::runtime::Handle::current());

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            app.manage(Mutex::new(AppState::new(db, config_handler)));
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            init_cache,
            get_all,
            get_with_id,
            update_metadata,
            create_metadata,
            load_from_file,
            load_from_directory,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Database(#[from] episko_lib::database::Error),

    #[error(transparent)]
    File(#[from] episko_lib::files::Error),

    #[error(transparent)]
    Builder(#[from] episko_lib::metadata::builder::Error),

    #[error(transparent)]
    Config(#[from] episko_lib::config::Error),

    #[error(transparent)]
    Metadata(#[from] episko_lib::metadata::Error),

    #[error("bad request: {0}")]
    BadRequest(String),
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
