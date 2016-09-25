#[macro_use]
extern crate lazy_static;

mod sync;

use std::io::{BufWriter, Stdout};
use sync::NotThreadSafe;

lazy_static! {
    pub static ref STDOUT: NotThreadSafe<BufWriter<Stdout>> = {
        NotThreadSafe::new(BufWriter::new(std::io::stdout()))
    };
}

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => { {
        use std::io::Write;

        unsafe {
            let stdout = $crate::STDOUT.get().as_mut().unwrap();
            writeln!(stdout, $($arg)*).unwrap();
        }
    } }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => { {
        use std::io::Write;

        unsafe {
            let stdout = $crate::STDOUT.get().as_mut().unwrap();
            write!(stdout, $($arg)*).unwrap();
        }
    } }
}
