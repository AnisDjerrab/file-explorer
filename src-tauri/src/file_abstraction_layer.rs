#[path = "file_ops/file_ops.rs"]
pub mod file_ops;
#[path = "root_ops/root_ops.rs"]
pub mod root_ops;

use libc::{fread, fwrite, malloc, strlen, FILE};
use std::ffi::{c_char, c_void, CString};

use std::sync::Mutex;

pub struct RawPtr(*mut FILE);

unsafe impl Send for RawPtr {}
unsafe impl Sync for RawPtr {}

pub struct ProcessMetadata {
    pub running: bool,
    pub pipe_in: RawPtr,
    pub pipe_out: RawPtr,
    pub pid: i32,
}

static SERVICE: Mutex<ProcessMetadata> = Mutex::new(ProcessMetadata {
    running: (false),
    pipe_in: RawPtr(std::ptr::null_mut()),
    pipe_out: RawPtr(std::ptr::null_mut()),
    pid: (0),
});

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
    let output = file_ops::path_exists(path, false);
    if output.1 == true {
        // try to launch the root service
        let status = launch_service_as_root();
        if status {
            unsafe {
                let metadata = SERVICE.lock().unwrap();
                let out_msg = CString::new("path_exists").unwrap();
                fwrite(
                    out_msg.as_ptr() as *const c_void,
                    strlen(out_msg.as_ptr()),
                    1,
                    metadata.pipe_out.0 as *mut FILE,
                );
                // now, wait to read what it returns
                let in_msg = malloc(4096) as *mut c_char;
                fread(
                    in_msg as *mut c_void,
                    4096,
                    1,
                    metadata.pipe_in.0 as *mut FILE,
                );
                if in_msg == "false".as_ptr() as *mut c_char {
                    false
                } else {
                    true
                }
            }
        } else {
            output.0
        }
    } else {
        output.0
    }
}

fn launch_service_as_root() -> bool {
    let app_cache_dir: String;
    #[cfg(target_os = "linux")]
    {
        let user = std::env::var("USER").unwrap_or_else(|_| "user".to_string());
        let mut pkexec = std::process::Command::new("pkexec")
            .args(&[format!(
                "{}{}{}",
                std::env::current_exe().unwrap().parent().unwrap().display(),
                std::path::MAIN_SEPARATOR,
                "service"
            )])
            .arg("--user")
            .arg(user)
            .spawn()
            .unwrap();
        let status = pkexec.wait().unwrap();
        if status.code().unwrap_or(-1) != 0 {
            return false;
        }
        SERVICE.lock().unwrap().running = true;
    }
    if SERVICE.lock().unwrap().running {
        // now, call the rust code that handles pipe in root_ops
        // low level stuff coming. we need to establish comms.
        #[cfg(unix)]
        {
            let app_cache_dir: String;
            #[cfg(target_os = "linux")]
            {
                app_cache_dir = format!(
                    "/home/{}/.cache/com.anis.file_explorer/",
                    std::env::var("USER").expect("ERR_HOME_NOT_FOUND")
                );
            }
            while !std::fs::exists(format!("{}{}", app_cache_dir, "PID.txt")).unwrap_or(false) {
                std::thread::yield_now();
            }
            let established_comms = root_ops::establish_comms_with_service_unix(&app_cache_dir);
            if !established_comms.is_null() {
                let mut metadata = SERVICE.lock().unwrap();
                metadata.pipe_in = unsafe { RawPtr((*established_comms).pipe_in) };
                metadata.pipe_out = unsafe { RawPtr((*established_comms).pipe_out) };
                let pid: i32 = std::fs::read_to_string(app_cache_dir)
                    .unwrap_or_default()
                    .lines()
                    .next()
                    .unwrap_or("")
                    .trim()
                    .parse()
                    .unwrap_or(0);
                if pid == 0 {
                    return false;
                }
                metadata.pid = pid;
                metadata.running = true;
                true
            } else {
                false
            }
        }
    } else {
        false
    }
}
