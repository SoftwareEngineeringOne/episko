#[tauri::command]
pub fn greet(name: String) -> String {
    episko_lib::greet(name)
}
