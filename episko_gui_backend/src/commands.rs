use serde::Serialize;
use std::path::Path;
use tokio::sync::Mutex;
use uuid::Uuid;

use episko_lib::{
    database::{retrieve_metadata::Pagination, DatabaseObject, Filter},
    files::File,
    metadata::{metadata_handler::MetadataHandler, Category, Language, Metadata, MetadataPreview},
    statistics::{statistic_handler::StatisticHandler, Statistic},
};

use crate::{model::MetadataDco, model::MetadataDto, AppState, Error};

static PAGE_SIZE: u32 = 10;

#[derive(Serialize, Debug)]
pub struct PagedData<T> {
    total_size: u32,
    page_size: u32,
    page_number: u32,
    data: Vec<T>,
}

#[tauri::command]
pub async fn init_cache(state: tauri::State<'_, Mutex<AppState>>) -> Result<(), Error> {
    let state = state.lock().await;

    let files = state.config_handler.files();
    for file in files {
        Metadata::from_file(file)?.write_to_db(&state.db).await?;
    }

    let dirs = state.config_handler.dirs();
    for dir in dirs {
        let files = MetadataHandler::search_directory(dir)?;
        for file in &files {
            Metadata::from_file(file)?.write_to_db(&state.db).await?;
        }
    }

    Ok(())
}

#[tauri::command]
pub async fn get_all(
    page_number: u32,
    filter: Filter,
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<PagedData<MetadataPreview>, Error> {
    let state = state.lock().await;

    let projects = Metadata::all_preview_from_db(
        Some(Pagination::new(page_number, PAGE_SIZE)),
        filter.clone(),
        &state.db,
    )
    .await?;

    Ok(PagedData {
        total_size: Metadata::amount_cached(filter.query, &state.db).await?,
        page_size: PAGE_SIZE,
        page_number,
        data: projects,
    })
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
pub async fn get_all_categories(
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<Vec<Category>, Error> {
    let state = state.lock().await;

    Ok(Category::all_names(state.db.conn()).await?)
}

#[tauri::command]
pub async fn get_all_languages(
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<Vec<Language>, Error> {
    let state = state.lock().await;

    Ok(Language::all_names(state.db.conn()).await?)
}

#[tauri::command]
pub async fn get_statistics(state: tauri::State<'_, Mutex<AppState>>) -> Result<Statistic, Error> {
    let state = state.lock().await;

    Ok(StatisticHandler::generate_statistics(&state.db).await?)
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

    metadata.update_in_db(&state.db).await?;
    metadata.write_file(&metadata.directory)?;

    Ok(metadata.into())
}

#[tauri::command]
pub async fn create_metadata(
    created: MetadataDco,
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<Uuid, Error> {
    let metadata = created.create()?;

    let mut state = state.lock().await;

    metadata.write_to_db(&state.db).await?;

    metadata.write_file(&metadata.directory)?;

    state.config_handler.add_saved_file(&metadata.directory);
    state.config_handler.save_config()?;

    Ok(metadata.id)
}

#[tauri::command]
pub async fn delete_metadata(
    metadata: MetadataDto,
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<(), Error> {
    let mut state = state.lock().await;

    let metadata: Metadata = metadata.into();

    state.config_handler.remove_saved_file(&metadata.directory);
    state.config_handler.save_config()?;

    metadata.remove_from_db(&state.db).await?;
    Metadata::remove_file(&metadata.directory)?;

    Ok(())
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
