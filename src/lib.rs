#[macro_use]
extern crate lazy_static;

extern crate libc;

use std::io::{Write, BufWriter, Stdout};

mod sync;
use sync::NotThreadSafe;

lazy_static! {
    pub static ref STDOUT: NotThreadSafe<BufWriter<Stdout>> = {
        unsafe { libc::atexit(flush_stdout_at_exit); }
        NotThreadSafe::new(BufWriter::new(std::io::stdout()))
    };
}

pub fn stdout() -> &'static mut BufWriter<Stdout> {
    unsafe { STDOUT.get().as_mut().unwrap() }
}

pub fn flush() {
    stdout().flush().unwrap();
}

extern "C" fn flush_stdout_at_exit() {
    flush();
}

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => { {
        use std::io::Write;
        writeln!($crate::stdout(), $($arg)*).unwrap();
    } }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => { {
        use std::io::Write;
        write!($crate::stdout(), $($arg)*).unwrap();
    } }
}

#[macro_export]
macro_rules! flush {
    () => { $crate::flush() }
}
