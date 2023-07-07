use tauri::{AppHandle, Manager};

#[tauri::command]
pub fn open_link(app: AppHandle, url: String) {
  tauri::api::shell::open(&app.shell_scope(), url, None).unwrap()
}
