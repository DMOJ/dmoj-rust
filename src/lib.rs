#[macro_use] extern crate nom;
#[macro_use] extern crate lazy_static;
extern crate libc;

mod macros;
mod io;
mod scan;
mod sync;

pub use io::{
    scan,
    stdout,
    read_char,
    read_byte
};
