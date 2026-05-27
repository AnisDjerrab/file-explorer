// include the file_ops namespace copied all the way here via a simple symlink
#[path = "file_ops/file_ops.rs"]
pub mod file_ops;

use libc::free;
use std::ffi::{c_void, CStr, CString};
use std::fs::{self, OpenOptions};
use std::fs::{DirBuilder, File};
use std::io::Write;
use std::os::raw::c_char;
use std::ptr::null;
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
            .open(
                std::env::current_exe()
                    .unwrap()
                    .parent()
                    .unwrap()
                    .join("debug.log")
            )
            .unwrap(),
        "ERROR: invalid func called"
    )
    .unwrap();
}

fn write_to_log_file(msg: &str) {
    _ = writeln!(
        OpenOptions::new()
            .append(true)
            .create(true)
            .open(
                std::env::current_exe()
                    .unwrap()
                    .parent()
                    .unwrap()
                    .join("debug.log")
            )
            .unwrap_or(std::fs::File::open("/dev/null").unwrap()),
        "{}",
        msg
    );
}

fn main() {
    write_to_log_file("INFO: successfully began the service.");
    if fs::exists(format!(
        "{}{}",
        format!(
            "{}{}",
            std::env::current_exe().unwrap().parent().unwrap().display(),
            std::path::MAIN_SEPARATOR
        ),
        "PID.txt"
    ))
    .unwrap()
    {
        fs::remove_file(format!(
            "{}{}",
            format!(
                "{}{}",
                std::env::current_exe().unwrap().parent().unwrap().display(),
                std::path::MAIN_SEPARATOR
            ),
            "PID.txt"
        ))
        .unwrap();
    }
    writeln!(
        OpenOptions::new()
            .append(true)
            .create(true)
            .open(format!(
                "{}{}",
                format!(
                    "{}{}",
                    std::env::current_exe().unwrap().parent().unwrap().display(),
                    std::path::MAIN_SEPARATOR
                ),
                "PID.txt"
            ))
            .unwrap(),
        "{}",
        std::process::id()
    )
    .unwrap();
    // statically decide where the cache dir is
    let app_cache_dir: String;
    #[cfg(target_os = "linux")]
    {
        // get the actual logged-in user (not root)
        use std::os::unix::fs::PermissionsExt;
        let user = std::env::args()
            .skip_while(|a| a != "--user")
            .nth(1)
            .unwrap_or_else(|| "user".to_string());
        app_cache_dir = format!("/home/{}/.cache/com.anis.file-explorer/", user).to_string();
        let mut builder = DirBuilder::new();
        builder.recursive(true);
        builder.create(&app_cache_dir).unwrap();
        let mut perms = fs::metadata(&app_cache_dir).unwrap().permissions();
        perms.set_mode(0o777);
        let _ = fs::set_permissions(&app_cache_dir, perms);
    }
    // now, we must get the stdin/out handlers
    let handlers =
        unsafe { establish_comms_with_core_unix(CString::new(app_cache_dir).unwrap().as_ptr()) };
    if handlers as *const i64 == null() as *const i64 {
        write_to_log_file("ERROR: something bad occured while creating pipes");
    } else {
        write_to_log_file("INFO: successfully etablished comms with the main program.");
        loop {
            // keep reeding the stdio input
            unsafe {
                let c_ptr: *mut c_char = get_stdin_pipe_input(handlers.read().pipe_in);
                write_to_log_file("INFO: received job to do.");
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
}
