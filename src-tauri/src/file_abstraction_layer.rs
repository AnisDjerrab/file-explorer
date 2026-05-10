#[path = "file_ops/file_ops.rs"]
pub mod file_ops;

#[tauri::command]
pub fn get_home_directory() -> String {
    file_ops::get_home_directory()
}

#[tauri::command]
pub fn ls_dir(
    path: String,
    sort_method: String,
    sort_method_dfs: String,
) -> (Vec<String>, Vec<String>, Vec<String>) {
    file_ops::ls_dir(path, sort_method, sort_method_dfs)
}

#[tauri::command]
pub fn path_exists(path: String) -> bool {
    let mut service = std::env::current_exe().unwrap();
    service.pop();
    service.push("service");
    std::process::Command::new(service)
        .spawn()
        .expect("failed to launch service");
    file_ops::path_exists(path)
}
