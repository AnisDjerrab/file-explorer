mod file_operations;

use tauri_plugin_opener::OpenerExt;

#[cfg(target_os = "android")]
use tauri_plugin_android_fs::{AndroidFsExt, PublicGeneralPurposeDir, PublicImageDir, Result, PrivateDir};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_android_fs::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![file_operations::get_home_directory, file_operations::ls_dir, request_directory_access, open_file_with_default_app])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn open_file_with_default_app(app: tauri::AppHandle, path: String) {
    let _ = app.opener().open_path(path, None::<&str>).map_err(|e| e.to_string());
}

// this is the func to let the app ask the user for minimal storage access
#[tauri::command]
async fn request_directory_access(app: tauri::AppHandle) {
    #[cfg(target_os = "android")]
    {
        let api = app.android_fs_async();

        // Request permission for PublicStorage
        //
        // NOTE:
        // Please enable 'legacy_storage_permission' feature,
        // for Android 9 or lower.
        if !api.public_storage().request_permission().await.ok().unwrap_or(false) {
            return 
        }

        // Save to new file.
        // 
        // Destination:
        // ~/Pictures/MyApp/my-image.png
        api.public_storage().write_new(
            // Storage volume (e.g. internal storage, SD card). 
            // If None, use primary storage volume
            None, 

            // Base directory. 
            // One of: PublicImageDir, PublicVideoDir, PublicAudioDir, PublicGeneralPurposeDir
            PublicImageDir::Pictures, 

            // Relative file path.
            // The parent directories will be created recursively.
            "MyApp/my-image.png",

            // Mime type.
            Some("image/png"),

            // Contents to save
            &[]
        ).await.ok();



        // Get any available volume other than the primary one if possible.
        // e.g. SD card, USB drive
        let volume = api
            .public_storage()
            .get_volumes().await.ok()
            .unwrap()
            .into_iter()
            .find(|v| !v.is_primary && !v.is_readonly);

        // Create an empty file
        // and mark it pending (hidden from other apps).
        let uri = match api.public_storage().create_new_file_with_pending(
                volume.as_ref().map(|v| &v.id),
                PublicGeneralPurposeDir::Documents,
                "MyApp/2025-9-14/data.txt",
                Some("text/plain")).await {
            Ok(u) => u,
            Err(_) => return,
        };
        let mut file = match api.open_file_writable(&uri).await {
            Ok(f) => f,
            Err(_) => return,
        };

        // Write content in blocking thread
        let result = tauri::async_runtime::spawn_blocking(move || -> Result<()> {
            use std::io::Write;

            // Write content
            file.write_all(&[])?;

            Ok(())
        }).await.map_err(Into::into).and_then(|r| r);

        // Handle error
        if let Err(err) = result {
            api.remove_file(&uri).await.ok();
            return 
        }

        // Clear pending state
        api.public_storage().set_pending(&uri, false).await.ok();

        // Register with Gallery
        api.public_storage().scan(&uri).await.ok();
        // Get the absolute path.
        // Apps can fully manage entries within those directories with 'std::fs'.
        let ps = api.private_storage();
        let cache_dir_path: std::path::PathBuf = ps.resolve_path(PrivateDir::Cache).await.unwrap();
        let data_dir_path: std::path::PathBuf = ps.resolve_path(PrivateDir::Data).await.unwrap();

        // Since these locations may contain files created by other Tauri plugins or webview systems, 
        // it is recommended to add a subdirectory with a unique name.
        let cache_dir_path = cache_dir_path.join("01K6049FVCD4SAGMAB6X20SA5S");
        let data_dir_path = data_dir_path.join("01K6049FVCD4SAGMAB6X20SA5S");
    }
}