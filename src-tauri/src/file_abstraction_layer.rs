#[path = "file_ops/file_ops.rs"]
pub mod file_ops;

use std::ffi::{c_char, c_void};
#[repr(C)]
pub struct ProcessInfos {
    pub success: bool,
    pub stdin: *mut c_void,  // FILE*
    pub stdout: *mut c_void, // FILE*
    pub stderr: *mut c_void, // FILE*
}

unsafe extern "C" {
    fn establish_comms_with_service_unix(pipe_dir_path: *const c_char) -> *mut ProcessInfos;
}

use std::process::{Child, ChildStderr, ChildStdin, ChildStdout, Command, Stdio};
use std::sync::Mutex;
use tauri::Manager;

struct process_metadata {
    launched: bool,
    stdin: Option<ChildStdin>,
    stdout: Option<ChildStdout>,
    stderr: Option<ChildStderr>,
    pid: u32,
}

static service: Mutex<process_metadata> = Mutex::new(process_metadata {
    launched: (false),
    stdin: (None),
    stdout: (None),
    stderr: (None),
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
    let mut service = std::env::current_exe().unwrap();
    service.pop();
    service.push("service");
    std::process::Command::new(service)
        .spawn()
        .expect("failed to launch service");
    file_ops::path_exists(path)
}

fn launch_service_as_root() {
    #[cfg(target_os = "linux")]
    {
        let mut pkexec = std::process::Command::new("pkexec")
            .args(&["service"])
            .spawn()
            .unwrap();
        let status = pkexec.wait().unwrap();
        if (status.code().unwrap() != 0) {
            return;
        }
        *service.lock().unwrap().launched = true;
    }
    if (*service.lock().unwrap().launched) {
        // now, call the good old C code to do all the job that rust can't
        // low level stuff coming. we need to establish comms.
        #[cfg(unix)]
        {
            while !fs::exists(format!(
                "{}{}",
                app.path().app_cache_dir().unwrap(),
                "PID.txt"
            ))
            .unwrap()
            {
                std::thread::yield_now();
            }
            let pipe_path = CString::new(app.path().app_cache_dir().unwrap())
                .expect("String contained interior nul byte");
            let pipe_path_ptr: *const c_char = c_string.as_ptr();
            let established_comms = unsafe { establish_comms_with_service_unix(pipe_path_ptr) };
        }
    }
}
