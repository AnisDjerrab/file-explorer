// include the file_ops namespace copied all the way here via a simple symlink
#[path = "file_ops/file_ops.rs"]
pub mod file_ops;

use libc::free;
use std::ffi::{CStr, CString};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::os::raw::c_char;
use tauri::Manager;

extern "C" {
    fn get_stdin_pipe_input() -> *mut c_char;
}

// now declare the string/function table
pub const FUNC_TABLE: &[(&str, usize)] = &[("path_exists", {
    let f: fn(String) -> bool = file_ops::path_exists;
    f as usize
})];

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
    if fs::exists(format!(
        "{}{}",
        app.path().app_cache_dir().unwrap(),
        "PID.txt"
    ))
    .unwrap()
    {
        fs::remove_file(format!(
            "{}{}",
            app.path().app_cache_dir().unwrap(),
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
                app.path().app_cache_dir().unwrap(),
                "PID.txt"
            ))
            .unwrap(),
        std::process::id()
    )
    .unwrap();
    loop {
        // keep reeding the stdio input
        unsafe {
            let c_ptr: *mut c_char = get_stdin_pipe_input();
            if c_ptr.is_null() {
                continue;
            }
            // get rid of the fine \n
            *c_ptr.add(CStr::from_ptr(ptr).to_bytes().len() - 1) = 0;
            // now, we compare what was transmitted with a pre-defined list
            let func_addr = FUNC_TABLE
                .binary_search_by_key(&CStr::from_ptr(c_ptr).to_str().unwrap(), |&(m, _)| m)
                .ok()
                .map(|i| FUNC_TABLE[i].1)
                .unwrap_or({
                    let f: fn() = invalid_func;
                    f as usize
                });
            // now, we call the func accordingly
            if *c_ptr == "path_exists" {
                // we need a single arg
                let arg1: *mut c_char = get_stdin_pipe_input();
                if arg1.is_null() {
                    continue;
                }
                let result: bool =
                    func_addr(&CStr::from_ptr(c_ptr).to_str().unwrap().to_string(), true);
                // transmit the result
                println!("{result}");
                // the program *should* receive it on the other side.
                // clean up
                free(arg1)
            } else {
                func_addr();
            }
            free(c_ptr);
        }
    }
}
