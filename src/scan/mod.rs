pub use parsers::*;

/// Scanning

pub fn scan<T>() -> T where Stdin<'static>: Scan<T> {
    stdin().scan()
}

pub trait Scan<T> {
    fn scan(&mut self) -> T;
}

fn parse_scanned_bytes<T>(bytes: &[u8]) -> T where T: FromStr {
    unsafe {
        match str::from_utf8_unchecked(bytes).parse::<T>() {
            Ok(value) => return value,
            _ => unreachable!()
        }
    }
}

macro_rules! impl_scan {
    ($x:ident, $t:ty) => {
        impl Scan<$t> for Stdin<'static> {
            fn scan(&mut self) -> $t {
                parse_scanned_bytes(self.$x())
            }
        }
    };
}

impl_scan!(scan_float, f32);
impl_scan!(scan_float, f64);

impl_scan!(scan_unsigned_integer, u8);
impl_scan!(scan_unsigned_integer, u16);
impl_scan!(scan_unsigned_integer, u32);
impl_scan!(scan_unsigned_integer, u64);
impl_scan!(scan_unsigned_integer, usize);

impl_scan!(scan_signed_integer, i8);
impl_scan!(scan_signed_integer, i16);
impl_scan!(scan_signed_integer, i32);
impl_scan!(scan_signed_integer, i64);
impl_scan!(scan_signed_integer, isize);
