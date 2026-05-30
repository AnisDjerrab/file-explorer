use libc::{fopen, mkfifo, open, stat, FILE, F_SETFL};

pub struct ProcessInfos {
    pub pipe_in: *mut FILE,  // FILE*
    pub pipe_out: *mut FILE, // FILE*
}

pub fn establish_comms_with_service_unix(pipe_dir_path: &str) -> *mut ProcessInfos {
    // the pipe_dir_path must not contain any dash
    // initialize the output
    let output: *mut ProcessInfos = Box::into_raw(Box::new(ProcessInfos {
        pipe_in: std::ptr::null_mut(),
        pipe_out: std::ptr::null_mut(),
    }));
    // the first pipe is the pipe from this main process to the service. the stdout equivalent.
    // assemble the hole path into a buffer.
    let pipe_out_path = format!("{}/pipe_core_to_service\0", pipe_dir_path);
    // create the pipe file & pipe itself
    let mut fd_out: *mut FILE;
    // check if it fails
    println!("hello world");
    if unsafe { mkfifo(pipe_out_path.as_ptr() as *const i8, 0777) == -1 } {
        unsafe { drop(Box::from_raw(output)) };
        return std::ptr::null_mut() as *mut ProcessInfos;
    }
    // we are waiting for the other side to just open the file in RO
    println!("hello world");
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
    println!("hello world");
    unsafe {
        (*output).pipe_out = fd_out;
    }
    // now, it's time to get the input pipe
    // in order to do that, we need to wait for the file to be created
    let pipe_in_path = format!("{}/pipe_service_to_core\0", pipe_dir_path);
    // loop where we test whether the file exists
    let mut st: stat = unsafe { std::mem::zeroed() };
    while unsafe { stat(pipe_in_path.as_ptr() as *const i8, &mut st) } != 0 {
        std::thread::yield_now();
    }
    println!("hello world");
    // service side - open read end non-blocking
    let fd_raw = unsafe {
        open(
            pipe_in_path.as_ptr() as *const i8,
            libc::O_RDONLY | libc::O_NONBLOCK,
        )
    };
    println!("hello world");
    // then clear non-blocking for actual use
    unsafe { libc::fcntl(fd_raw, F_SETFL, 0) };
    let fd_in = unsafe {
        fopen(
            pipe_in_path.as_ptr() as *const i8,
            b"r\0".as_ptr() as *const i8,
        )
    };
    println!("hello world");
    unsafe {
        (*output).pipe_in = fd_in;
    }
    output
}
