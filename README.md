# dmoj-rust

A Rust crate for providing helpful methods in online judging.

- [Usage](#usage)
- [Provided macros](#provided-macros)

## Usage

```rust
#[macro_use]
extern crate dmoj;

fn main() {
    println!("Hello, World!");
}
```

## Provided macros

### `print!`, `println!`

This crate provides `print!` and `println!` macros that shadow the prelude versions. These versions are about 10 times faster and fully API compatible, but sacrifice thread safety.
