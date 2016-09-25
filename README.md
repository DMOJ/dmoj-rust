# dmoj-rust

A Rust crate for providing helpful methods in online judging.

## Macros

### `print!` and `println!`

This crate exports `print!` and `println!` macros that shadow the prelude versions. These versions are about 10 times faster and fully API compatible, but sacrifice thread safety.
