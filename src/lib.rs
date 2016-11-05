#[macro_use] extern crate lazy_static;
extern crate libc;

mod buf;
mod io;
mod macros;
mod scan;
mod sync;

pub use io::{
    scan,
    stdout,
    flush
};
