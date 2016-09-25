# dmoj-rust

A Rust crate for providing helpful methods in online judging.

## Macros

### `println!` and `print!`

This crate exports `println!` and `print!` macros that shadow the prelude versions. These versions are about 10 times faster, fully API compatible, but sacrifice thread safety.
