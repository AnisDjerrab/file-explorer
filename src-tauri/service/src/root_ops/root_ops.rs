use libc::{fflush, fgets, fopen, free, fwrite, malloc, mkfifo, open, strnlen, FILE, F_SETFL};
use std::ffi::c_void;

pub struct ProcessInfos {
    pub pipe_in: *mut FILE,  // FILE*
    pub pipe_out: *mut FILE, // FILE*
}

const MAX_IO_BUFFER_SIZE: usize = 4096;

// this is a low level function to read directly from pipe_in without any interferences
pub fn get_stdin_pipe_input(pipe: *mut FILE) -> *mut i8 {
    let buffer = unsafe { malloc(MAX_IO_BUFFER_SIZE) } as *mut i8;
    let ptr_or_err = unsafe { fgets(buffer, MAX_IO_BUFFER_SIZE as i32, pipe) };
    if ptr_or_err.is_null() {
        unsafe { free(buffer as *mut c_void) };
        std::ptr::null_mut()
    } else {
        buffer
    }
}

// this is a low level function to read directly from pipe_in without any interferences
pub fn send_stdout_pipe_output(pipe: *mut FILE, msg: *const i8) {
    unsafe {
        fwrite(
            msg as *const c_void,
            strnlen(msg, MAX_IO_BUFFER_SIZE),
            1,
            pipe,
        );
        fflush(pipe);
    }
}

pub fn establish_comms_with_core_unix(pipe_dir_path: &str) -> *mut ProcessInfos {
    // the pipe_dir_path must not contain any dash
    // initialize the output
    let output: *mut ProcessInfos = Box::into_raw(Box::new(ProcessInfos {
        pipe_in: std::ptr::null_mut(),
        pipe_out: std::ptr::null_mut(),
    }));
    // the first pipe is the pipe from this main process to the service. the stdin equivalent.
    // assemble the hole path into a buffer.
    let pipe_in_path = format!("{}/pipe_core_to_service\0", pipe_dir_path);
    // open the path in RO
    let fd_in: *mut FILE;
    // service side - open read end non-blocking
    let fd_raw = unsafe {
        open(
            pipe_in_path.as_ptr() as *const i8,
            libc::O_RDONLY | libc::O_NONBLOCK,
        )
    };
    // then clear non-blocking for actual use
    unsafe { libc::fcntl(fd_raw, F_SETFL, 0) };
    fd_in = unsafe {
        fopen(
            pipe_in_path.as_ptr() as *const i8,
            b"r\0".as_ptr() as *const i8,
        )
    };
    unsafe { (*output).pipe_in = fd_in }
    // now, it's time to create the output pipe
    let pipe_out_path = format!("{}/pipe_service_to_core\0", pipe_dir_path);
    // create the pipe file & pipe itself
    let mut fd_out: *mut FILE;
    // check if it fails
    if unsafe { mkfifo(pipe_out_path.as_ptr() as *const i8, 0o777) == -1 } {
        return std::ptr::null_mut();
    }
    // we are waiting for the other side to just open the file in RO
    unsafe {
        loop {
            fd_out = fopen(
                pipe_out_path.as_ptr() as *const i8,
                b"w\0".as_ptr() as *const i8,
            );
            if !fd_out.is_null() {
                break;
            }
            std::thread::yield_now();
        }
    }
    unsafe {
        (*output).pipe_out = fd_out;
    }
    output
}
