# dmoj-rust

A Rust crate for providing helpful methods in online judging.

- [Usage](#usage)
- [Provided macros](#provided-macros)
    - [`print!`, `println!`](#print-println)
    - [`flush!`](#flush)

## Usage

```rust
#[macro_use] extern crate dmoj;

fn main() {
    println!("Hello, World!");
}
```

## Provided macros

### `print!`, `println!`

This crate provides `print!` and `println!` macros that shadow the prelude versions. These versions are about 10 times faster and fully API compatible, but sacrifice thread safety.

#### Example

```rust
#[macro_use] extern crate dmoj;

fn main() {
    print!("Hello, ");
    println!("World!");
}
```

### `flush!`

Flushes the stdout buffer.

#### Example

```rust
#[macro_use] extern crate dmoj;

use std::thread;
use std::time::Duration;

fn main() {
    print!("Hello,");
    flush!();
    thread::sleep(Duration::from_secs(2));
    println!(" World!");
}
```
