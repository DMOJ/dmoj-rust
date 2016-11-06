# dmoj-rust

A Rust crate for providing helpful methods in online judging.

- [Usage](#usage)
- [Provided macros](#provided-macros)
    - [`print!`, `println!`](#print-println)
    - [`flush!`](#flush)
    - [`scan!`](#scan)

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

### `scan!`

```rust
scan!(T) -> T;
scan!(T1, ..., Tn) -> (T1, ..., Tn);
```

A macro for scanning values from stdin. Currently, only scanning integers is supported.
Note that the scanner will continue to read stdin until it finds something that looks like an integer.
If the macro is called with multiple type arguments a tuple of the values will be returned, otherwise
the value itself is returned.

#### Example

```rust
#[macro_use] extern crate dmoj;

fn main() {
    // For example, if stdin contains " 2020 \n  +4 test \n   -19" then

    print!("{:?}", scan!(u64));      // prints "2020", and
    print!("{:?}", scan!(i16, i16)); // prints "(4, -19)"
}
```
