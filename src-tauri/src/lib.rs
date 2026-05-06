mod file_operations;

use tauri_plugin_opener::OpenerExt;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![file_operations::get_home_directory, file_operations::ls_dir, open_file_with_default_app])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn open_file_with_default_app(app: tauri::AppHandle, path: String) {
    let _ = app.opener().open_path(path, None::<&str>).map_err(|e| e.to_string());
}