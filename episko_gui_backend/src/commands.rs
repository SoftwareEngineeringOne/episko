use tokio::sync::Mutex;
use uuid::Uuid;

use episko_lib::{
    files::File,
    metadata::{metadata_handler::MetadataHandler, Metadata},
};

use crate::AppState;

#[allow(clippy::needless_pass_by_value)]
#[tauri::command]
pub fn greet(name: String) -> String {
    episko_lib::greet(&name)
}

#[tauri::command]
pub async fn all(state: tauri::State<'_, Mutex<AppState>>) -> Result<Vec<Metadata>, String> {
    let state = state.lock().await;

    // let dirs = &state.config.directories_to_load;
    //
    // let mut projects: Vec<Metadata> = vec![];
    // for dir in dirs {
    //     let files = MetadataHandler::search_directory(&dir).map_err(|e| e.to_string())?;
    //
    //     for file in files {
    //         let data = Metadata::from_file(&file);
    //         match data {
    //             Ok(project) => projects.push(project),
    //             Err(e) => println!("Error: {:#?}", e),
    //         }
    //     }
    // }

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

    Ok(Metadata::from_db(&state.db, id)
        .await
        .map_err(|err| err.to_string())?)
}
