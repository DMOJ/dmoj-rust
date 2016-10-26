use std::io::{self, Write, BufWriter, Stdout};
use libc;
use sync::NotThreadSafe;

lazy_static! {
    static ref STDOUT: NotThreadSafe<BufWriter<Stdout>> = {
        extern "C" fn flush_stdout_at_exit() {
            flush();
        }

        unsafe {
            libc::atexit(flush_stdout_at_exit);
        }

        NotThreadSafe::new(BufWriter::new(io::stdout()))
    };
}

pub fn stdout() -> &'static mut BufWriter<Stdout> {
    unsafe { STDOUT.get().as_mut().unwrap() }
}

pub fn flush() {
    stdout().flush().unwrap();
}
