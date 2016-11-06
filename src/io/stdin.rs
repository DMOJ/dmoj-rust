use std::io::{Read, Result, Error};
use libc::{self, c_void, size_t};

use sync::NotThreadSafe;
use buf::CopyingBufReader;

lazy_static! {
    static ref STDIN: NotThreadSafe<CopyingBufReader<Stdin>> = {
        NotThreadSafe::new(CopyingBufReader::new(Stdin::new()))
    };
}

pub fn stdin() -> &'static mut CopyingBufReader<Stdin> {
    unsafe { STDIN.get().as_mut().unwrap() }
}

pub struct Stdin;

impl Stdin {
    fn new() -> Stdin {
        Stdin {}
    }
}

impl Read for Stdin {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let ret = unsafe {
            libc::read(
                libc::STDIN_FILENO,
                buf.as_mut_ptr() as *mut c_void,
                buf.len() as size_t
            )
        };

        if ret == -1 {
            Err(Error::last_os_error())
        } else {
            Ok(ret as usize)
        }
    }
}
