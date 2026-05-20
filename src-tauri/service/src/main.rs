// include the file_ops namespace copied all the way here via a simple symlink
#[path = "file_ops/file_ops.rs"]
pub mod file_ops;

use libc::free;
use std::ffi::{c_void, CStr, CString};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::os::raw::c_char;
#[repr(C)]
pub struct ProcessInfos {
    pub pipe_in: *mut c_void,  // FILE*
    pub pipe_out: *mut c_void, // FILE*
}
extern "C" {
    fn get_stdin_pipe_input(pipe_in: *mut c_void) -> *mut c_char;
    fn send_stdout_pipe_output(pipe_out: *mut c_void, msg: *const c_char);
    fn establish_comms_with_core_unix(pipe_dir_path: *const c_char) -> *mut ProcessInfos;
}

fn invalid_func() {
    writeln!(
        OpenOptions::new()
            .append(true)
            .create(true)
            .open("debug.log")
            .unwrap(),
        "ERROR: invalid func called"
    )
    .unwrap();
}

fn main() {
    let app_cache_dir;
    #[cfg(target_os = "linux")]
    {
        app_cache_dir = "~/.config/com.anis.file_explorer/";
    }
    if fs::exists(format!("{}{}", app_cache_dir, "PID.txt")).unwrap() {
        fs::remove_file(format!("{}{}", app_cache_dir, "PID.txt")).unwrap();
    }
    writeln!(
        OpenOptions::new()
            .append(true)
            .create(true)
            .open(format!("{}{}", app_cache_dir, "PID.txt"))
            .unwrap(),
        "{}",
        std::process::id()
    )
    .unwrap();
    // now, we must get the stdin/out handlers
    let handlers =
        unsafe { establish_comms_with_core_unix(CString::new(app_cache_dir).unwrap().as_ptr()) };
    loop {
        // keep reeding the stdio input
        unsafe {
            let c_ptr: *mut c_char = get_stdin_pipe_input(handlers.read().pipe_in);
            if c_ptr.is_null() {
                continue;
            }
            // get rid of the fine \n
            *c_ptr.add(CStr::from_ptr(c_ptr).to_bytes().len() - 1) = 0;
            // now, we call the func accordingly
            if CStr::from_ptr(c_ptr).to_str().unwrap() == "path_exists" {
                // we need a single arg
                let arg1: *mut c_char = get_stdin_pipe_input(handlers.read().pipe_in);
                if arg1.is_null() {
                    continue;
                }
                let result: (bool, bool) = file_ops::path_exists(
                    CStr::from_ptr(c_ptr).to_str().unwrap().to_string(),
                    true,
                );
                // transmit the result
                send_stdout_pipe_output(
                    handlers.read().pipe_out,
                    CString::new(result.0.to_string()).unwrap().as_ptr(),
                );
                // the program *should* receive it on the other side.
                // clean up
                free(arg1 as *mut c_void)
            } else {
                invalid_func();
            }
            free(c_ptr as *mut c_void);
        }
    }
}
