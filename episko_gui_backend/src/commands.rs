use std::{ops::Deref, path::Path};
use tokio::sync::Mutex;
use uuid::Uuid;

use episko_lib::{
    ApplyIf,
    files::File,
    metadata::{Metadata, MetadataBuilder, metadata_handler::MetadataHandler},
};

use crate::{AppState, MetadataDco, MetadataDto};

#[tauri::command]
pub async fn get_all(state: tauri::State<'_, Mutex<AppState>>) -> Result<Vec<MetadataDto>, String> {
    let state = state.lock().await;

    let projects = Metadata::all_from_db(&state.db)
        .await
        .map_err(|err| err.to_string())?
        .into_iter()
        .map(Into::into)
        .collect();

    Ok(projects)
}

#[tauri::command]
pub async fn get_with_id(
    id: Uuid,
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<MetadataDto, String> {
    let state = state.lock().await;

    Metadata::from_db(&state.db, id)
        .await
        .map_err(|err| err.to_string())
        .map(Into::into)
}

#[tauri::command]
pub async fn update_metadata(
    id: Uuid,
    updated: MetadataDco,
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<MetadataDto, String> {
    let state = state.lock().await;

    let metadata = Metadata::from_db(&state.db, id)
        .await
        .map_err(|err| err.to_string())?;

    let metadata = metadata
        .update()
        .directory_path(&updated.directory)
        .title(&updated.title)
        .categories(updated.categories)
        .languages(updated.languages)
        .build_systems(updated.build_systems)
        .apply_if(updated.preffered_ide, MetadataBuilder::preffered_ide)
        .apply_if(updated.description.as_deref(), MetadataBuilder::description)
        .apply_if(
            updated.repository_url.as_deref(),
            MetadataBuilder::repository_url,
        )
        .build()
        .map_err(|err| err.to_string())?;

    metadata
        .write_to_db(&state.db)
        .await
        .map_err(|err| err.to_string())?;
    metadata
        .write_file(&metadata.directory)
        .map_err(|err| err.to_string())?;

    Ok(metadata.into())
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
