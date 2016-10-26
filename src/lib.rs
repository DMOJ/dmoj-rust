#[macro_use] extern crate lazy_static;
extern crate libc;

mod macros;
mod sync;
mod io;

pub use io::{
    stdout,
    flush
};
