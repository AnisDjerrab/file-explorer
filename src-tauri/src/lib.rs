mod file_abstraction_layer;

use std::fs::File;
use std::io::{BufRead, BufReader};

use tauri_plugin_opener::OpenerExt;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            file_abstraction_layer::get_home_directory,
            file_abstraction_layer::ls_dir,
            file_abstraction_layer::path_exists,
            open_file_with_default_app,
            get_operating_system
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn open_file_with_default_app(app: tauri::AppHandle, path: String) {
    let _ = app
        .opener()
        .open_path(path, None::<&str>)
        .map_err(|e| e.to_string());
}

#[tauri::command]
fn get_operating_system() -> (String, String, String) {
    let OS = std::env::consts::OS.to_string();
    let OS_type = std::env::consts::FAMILY.to_string();
    if OS == "linux" {
        // parse /etc/os-release to get OS name
        let file = File::open("/etc/os-release");
        let file = match file {
            Ok(f) => f,
            Err(_) => return (OS, String::new(), OS_type),
        };
        let bufReader = BufReader::new(file);
        let mut distro_name = String::new();
        for line in bufReader.lines() {
            let content = match line {
                Ok(l) => l,
                Err(_) => String::new(),
            };
            let content: String = content.split_whitespace().collect();
            if &content[0..6] == "NAME=\"" && content.chars().last() == Some('"') {
                distro_name = content[6..(content.len() - 1)].to_string();
                break;
            }
        }
        (OS, distro_name, OS_type)
    } else {
        (OS, String::new(), OS_type)
    }
}
