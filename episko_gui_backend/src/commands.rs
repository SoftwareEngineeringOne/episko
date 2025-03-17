use std::path::Path;
use tokio::sync::Mutex;
use uuid::Uuid;

use episko_lib::{
    files::File,
    metadata::{Metadata, metadata_handler::MetadataHandler},
};

use crate::AppState;

#[tauri::command]
pub async fn get_all(state: tauri::State<'_, Mutex<AppState>>) -> Result<Vec<Metadata>, String> {
    let state = state.lock().await;

    let projects = Metadata::all_from_db(&state.db)
        .await
        .map_err(|err| err.to_string())?;

    Ok(projects)
}

#[tauri::command]
pub async fn get_with_id(
    id: Uuid,
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<Metadata, String> {
    let state = state.lock().await;

    Metadata::from_db(&state.db, id)
        .await
        .map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn load_from_file(
    path: &Path,
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<Uuid, String> {
    if !path.exists() || !path.is_file() {
        return Err(String::from("Invalid path"));
    }
    let mut state = state.lock().await;
    load_file(path, &mut state, true).await.map(|el| el.id())
}

#[tauri::command]
pub async fn load_from_directory(
    path: &Path,
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<usize, String> {
    if !path.exists() || !path.is_dir() {
        return Err(String::from("Invalid path"));
    }
    let mut state = state.lock().await;

    let files = MetadataHandler::search_directory(path).map_err(|err| err.to_string())?;

    let mut projects: Vec<Metadata> = Vec::with_capacity(files.len());
    for file in files {
        projects.push(load_file(&file, &mut state, false).await?);
    }

    let ch = &mut state.config_handler;

    ch.add_saved_directory(path);
    ch.save_config().map_err(|err| err.to_string())?;

    Ok(projects.len())
}

async fn load_file(
    path: &Path,
    state: &mut AppState,
    save_to_config: bool,
) -> Result<Metadata, String> {
    let metadata = Metadata::from_file(path).map_err(|err| err.to_string())?;

    metadata
        .write_to_db(&state.db)
        .await
        .map_err(|err| err.to_string())?;

    if save_to_config {
        let ch = &mut state.config_handler;
        ch.add_saved_file(path);
        ch.save_config().map_err(|err| err.to_string())?;
    }

    Ok(metadata)
}
