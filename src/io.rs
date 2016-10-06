use std::io::{self, Write, BufWriter, Stdout};
use std::str::{self, FromStr};
use std::ptr;
use std::slice;
use std::ops::Range;

use libc::{self, size_t, c_void};

use nom::IResult;

use sync::NotThreadSafe;
use scan::{int, float};
// use scan;

const DEFAULT_BUF_SIZE: usize = 64 * 1024;
static mut STDIN_BUF: [u8; DEFAULT_BUF_SIZE] = [0; DEFAULT_BUF_SIZE];

lazy_static! {
    pub static ref STDIN: NotThreadSafe<Stdin<'static>> = {
        NotThreadSafe::new(
            Stdin::new(unsafe { &mut STDIN_BUF })
        )
    };
}

lazy_static! {
    pub static ref STDOUT: NotThreadSafe<BufWriter<Stdout>> = {
        unsafe { libc::atexit(flush_stdout_at_exit); }
        NotThreadSafe::new(BufWriter::new(io::stdout()))
    };
}

pub fn stdin() -> &'static mut Stdin<'static> {
    unsafe { STDIN.get().as_mut().unwrap() }
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


///////////////////////////////////////////////////////////////


pub struct Stdin<'a> {
    buf: &'a mut [u8],
    win: Range<usize>
}

impl<'a> Stdin<'a> {
    fn new(buf: &'a mut [u8]) -> Stdin<'a> {
        Stdin { buf: buf, win: 0..0 }
    }

    fn refill(&mut self, keep: usize) {
        if keep > 0 {
            // assert!(2*keep <= self.buf.len());
            unsafe {
                ptr::copy_nonoverlapping(
                    self.buf.as_ptr().offset((self.buf.len() - keep) as isize),
                    self.buf.as_mut_ptr(),
                    keep
                );
            }
        }

        let size = unsafe {
            libc::read(
                libc::STDIN_FILENO,
                self.buf.as_mut_ptr().offset(keep as isize) as *mut c_void,
                (self.buf.len() - keep) as size_t // this is wrong `read` returns < buf.len() bytes
            ) as usize
        };

        self.win = 0..size;
    }

    fn read_byte(&mut self) -> u8 {
        if self.win.len() == 0 { self.refill(0); }

        let value = self.buf[self.win.start];
        self.win.start += 1;
        value
    }

    fn read_char(&mut self) -> char {
        self.read_byte() as char
    }

    fn scan_float(&mut self) -> &[u8] {
        self.scan_nom_parser(float::float)
    }

    fn scan_unsigned_integer(&mut self) -> &[u8] {
        self.scan_nom_parser(int::unsigned_integer)
    }

    fn scan_signed_integer(&mut self) -> &[u8] {
        self.scan_nom_parser(int::signed_integer)
    }

    fn scan_nom_parser<F>(&mut self, scanner: F) -> &[u8]
        where F: Fn(&[u8]) -> IResult<&[u8], &[u8]> {

        loop {
            if self.win.len() == 0 {
                self.refill(0);
            }

            match scanner(&self.buf[self.win.clone()]) {
                IResult::Error(_) => {
                    self.win.start += 1;
                    continue;
                },
                IResult::Done(_, res) => {
                    self.win.start += res.len();
                    return unsafe {
                        slice::from_raw_parts(res.as_ptr(), res.len())
                    };
                },
                IResult::Incomplete(_) => {
                    // Fall through the match to workaround lexical lifetimes.
                }
            }

            let window_size = self.win.len();
            self.refill(window_size);
        }
    }
}

pub fn read_char() -> char {
    stdin().read_char()
}

pub fn read_byte() -> u8 {
    stdin().read_byte()
}

/// Scanning

pub fn scan<T>() -> T where Stdin<'static>: Scan<T> {
    stdin().scan()
}

pub trait Scan<T> {
    fn scan(&mut self) -> T;
}

impl Scan<f32> for Stdin<'static> {
    fn scan(&mut self) -> f32 {
        scan_bytes(self.scan_float())
    }
}

impl Scan<f64> for Stdin<'static> {
    fn scan(&mut self) -> f64 {
        scan_bytes(self.scan_float())
    }
}

impl Scan<u8> for Stdin<'static> {
    fn scan(&mut self) -> u8 {
        scan_bytes(self.scan_unsigned_integer())
    }
}

impl Scan<u16> for Stdin<'static> {
    fn scan(&mut self) -> u16 {
        scan_bytes(self.scan_unsigned_integer())
    }
}

impl Scan<u32> for Stdin<'static> {
    fn scan(&mut self) -> u32 {
        scan_bytes(self.scan_unsigned_integer())
    }
}

impl Scan<u64> for Stdin<'static> {
    fn scan(&mut self) -> u64 {
        scan_bytes(self.scan_unsigned_integer())
    }
}

fn scan_bytes<T>(bytes: &[u8]) -> T where T: FromStr {
    unsafe {
        match str::from_utf8_unchecked(bytes).parse::<T>() {
            Ok(value) => return value,
            _ => unreachable!()
        }
    }
}
