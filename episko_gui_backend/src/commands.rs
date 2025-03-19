use std::path::Path;
use tokio::sync::Mutex;
use uuid::Uuid;

use episko_lib::{
    files::File,
    metadata::{Metadata, metadata_handler::MetadataHandler},
};

use crate::{AppState, Error, model::MetadataDco, model::MetadataDto};

#[tauri::command]
pub async fn get_all(state: tauri::State<'_, Mutex<AppState>>) -> Result<Vec<MetadataDto>, Error> {
    let state = state.lock().await;

    let projects = Metadata::all_from_db(&state.db)
        .await?
        .into_iter()
        .map(Into::into)
        .collect();

    Ok(projects)
}

#[tauri::command]
pub async fn get_with_id(
    id: Uuid,
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<MetadataDto, Error> {
    let state = state.lock().await;

    Ok(Metadata::from_db(&state.db, id).await.map(Into::into)?)
}

#[tauri::command]
pub async fn update_metadata(
    id: Uuid,
    updated: MetadataDco,
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<MetadataDto, Error> {
    let state = state.lock().await;

    let metadata = Metadata::from_db(&state.db, id).await?;

    let metadata = updated.update(metadata)?;

    metadata.write_to_db(&state.db).await?;
    metadata.write_file(&metadata.directory)?;

    Ok(metadata.into())
}

#[tauri::command]
pub async fn create_metadata(
    new: MetadataDco,
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<MetadataDto, Error> {
    let metadata = new.create()?;

    let mut state = state.lock().await;

    metadata.write_to_db(&state.db).await?;

    metadata.write_file(&metadata.directory)?;

    state.config_handler.add_saved_file(&metadata.directory);
    state.config_handler.save_config()?;

    Ok(metadata.into())
}

#[tauri::command]
pub async fn load_from_file(
    path: &Path,
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<Uuid, Error> {
    if !path.exists() {
        return Err(Error::BadRequest("given path does not exist".to_string()));
    }
    if !path.is_file() {
        return Err(Error::BadRequest("given path is not a file".to_string()));
    }

    let mut state = state.lock().await;
    load_file(path, &mut state, true).await.map(|el| el.id())
}

#[tauri::command]
pub async fn load_from_directory(
    path: &Path,
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<usize, Error> {
    if !path.exists() {
        return Err(Error::BadRequest("given path does not exist".to_string()));
    }
    if !path.is_dir() {
        return Err(Error::BadRequest(
            "given path is not a directory".to_string(),
        ));
    }

    let mut state = state.lock().await;

    let files = MetadataHandler::search_directory(path)?;

    let mut projects: Vec<Metadata> = Vec::with_capacity(files.len());
    for file in files {
        projects.push(load_file(&file, &mut state, false).await?);
    }

    let ch = &mut state.config_handler;

    ch.add_saved_directory(path);
    ch.save_config()?;

    Ok(projects.len())
}

async fn load_file(
    path: &Path,
    state: &mut AppState,
    save_to_config: bool,
) -> Result<Metadata, Error> {
    let metadata = Metadata::from_file(path)?;

    metadata.write_to_db(&state.db).await?;

    if save_to_config {
        let ch = &mut state.config_handler;
        ch.add_saved_file(path);
        ch.save_config()?;
    }

    Ok(metadata)
}
