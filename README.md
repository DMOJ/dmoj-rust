# dmoj-rust

A Rust crate for providing helpful methods in online judging.

## Macros

### `print!`, `println!` and `flush!`

This crate exports `print!` and `println!` macros that shadow the prelude versions. These versions are about 10 times faster and fully API compatible, but sacrifice thread safety.

Note: For now, you must manually `flush!` at the end of your program to ensure the stdout buffer is flushed:

```rust
#[macro_use] extern crate dmoj;

fn main() {
    print!("Hello,");
    println!(" World!");
    flush!();
}
```