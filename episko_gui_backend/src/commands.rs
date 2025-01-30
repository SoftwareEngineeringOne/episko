#[allow(clippy::needless_pass_by_value)]
#[tauri::command]
pub fn greet(name: String) -> String {
    episko_lib::greet(&name)
}
