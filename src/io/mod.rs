mod stdin;
mod stdout;

use scan::Scan;
use buf::CopyingBufReader;

pub const DEFAULT_BUF_SIZE: usize = 8 * 1024;

pub use self::stdin::{stdin, Stdin};
pub use self::stdout::{
    stdout,
    flush
};

pub fn scan<T>() -> T where CopyingBufReader<Stdin>: Scan<T> {
    Scan::<T>::scan(stdin())
}
