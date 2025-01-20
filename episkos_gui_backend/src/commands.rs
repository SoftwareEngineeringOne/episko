#[tauri::command]
pub fn greet(name: String) -> String {
    episkos_lib::greet(name)
}
