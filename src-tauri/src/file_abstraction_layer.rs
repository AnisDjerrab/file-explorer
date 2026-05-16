#[path = "file_ops/file_ops.rs"]
pub mod file_ops;

use libc::{fread, fwrite};
use std::ffi::{c_char, c_void, CString};
#[repr(C)]
pub struct ProcessInfos {
    pub pipe_in: *mut c_void,  // FILE*
    pub pipe_out: *mut c_void, // FILE*
}

unsafe extern "C" {
    fn establish_comms_with_service_unix(pipe_dir_path: *const c_char) -> *mut ProcessInfos;
}

use std::fs::metadata;
use std::os::unix::fs::MetadataExt;
use std::process::{Child, ChildStderr, ChildStdin, ChildStdout, Command, Stdio};
use std::sync::Mutex;
use tauri::Manager;

pub struct ProcessMetadata {
    pub running: bool,
    pub pipe_in: *mut c_void,
    pub pipe_out: *mut c_void,
    pub pid: i32,
}

static SERVICE: Mutex<process_metadata> = Mutex::new(process_metadata {
    running: (false),
    pipe_in: (0),
    pipe_out: (0),
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
    let output = file_ops::path_exists(path);
    if output.1 == true {
        // try to launch the root service
        let status = launch_service_as_root();
        if status {
            let mut metadata = SERVICE.lock().unwrap();
            let out_msg = CString::new("path_exists");
            fwrite(out_msg.as_ptr(), strlen(out_msg), 1, metadata.pipe_out);
            // now, wait to read what it returns
            unsafe {
                let in_msg = malloc(4096);
                fread(in_msg, 4096, 1, metadata.pipe_in);
                if in_msg == "false" {
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
    #[cfg(target_os = "linux")]
    {
        let mut pkexec = std::process::Command::new("pkexec")
            .args(&["service"])
            .spawn()
            .unwrap();
        let status = pkexec.wait().unwrap();
        if (status.code().unwrap() != 0) {
            return false;
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
            let mut metadata = SERVICE.lock().unwrap();
            metadata.pipe_in = established_comms.pipe_in;
            metadata.pipe_out = established_comms.pipe_out;
            let pid: i32 = std::fs::read_to_string(app.cache().cache_dir().unwrap())
                .unwrap_or_default()
                .lines()
                .next()
                .unwrap_or("")
                .trim()
                .parse()
                .unwrap_or(0);
            if (pid == 0) {
                return false;
            }
            metadata.pid = pid;
            metadata.running = true;
            true
        }
    } else {
        false
    }
}
