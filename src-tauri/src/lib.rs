mod file_operations;


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![file_operations::get_home_directory])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

