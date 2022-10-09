use std::io::Read;

use buf::CopyingBufReader;

pub trait Scan<T> {
    fn scan(&mut self) -> T;
}

fn is_digit(b: u8) -> bool {
    b'0' <= b && b <= b'9'
}

fn to_value(b: u8) -> u8 {
    b - b'0'
}

macro_rules! impl_scan_signed_integer {
    ($t:ty) => {
        impl<R: Read> Scan<$t> for CopyingBufReader<R> {
            fn scan(&mut self) -> $t {
                let mut neg = false;
                let mut value;

                loop {
                    match self.peek() {
                        Some(b) => {
                            self.consume(1);

                            if b == b'-' {
                                neg = true;
                            }

                            if b == b'+' || b == b'-' {
                                match self.peek() {
                                    Some(b) if is_digit(b) => {
                                        self.consume(1);
                                        value = to_value(b) as $t;
                                    },
                                    _ => panic!()
                                }
                                break;
                            } else if is_digit(b) {
                                value = to_value(b) as $t;
                                break;
                            }
                        },
                        None => panic!()
                    }
                }

                loop {
                    match self.peek() {
                        Some(b) if is_digit(b) => {
                            self.consume(1);
                            value = 10*value + to_value(b) as $t;
                        },
                        _ => break
                    }
                }

                if neg {
                    -value
                } else {
                    value
                }
            }
        }
    };
}

impl_scan_signed_integer!(i8);
impl_scan_signed_integer!(i16);
impl_scan_signed_integer!(i32);
impl_scan_signed_integer!(i64);
impl_scan_signed_integer!(isize);

macro_rules! impl_scan_unsigned_integer {
    ($t:ty) => {
        impl<R: Read> Scan<$t> for CopyingBufReader<R> {
            fn scan(&mut self) -> $t {
                let mut value;

                loop {
                    match self.peek() {
                        Some(b) => {
                            self.consume(1);

                            if b == b'+' {
                                match self.peek() {
                                    Some(b) if is_digit(b) => {
                                        self.consume(1);
                                        value = to_value(b) as $t;
                                    },
                                    _ => panic!()
                                }
                                break;
                            } else if is_digit(b) {
                                value = to_value(b) as $t;
                                break;
                            }
                        },
                        None => panic!()
                    }
                }

                loop {
                    match self.peek() {
                        Some(b) if is_digit(b) => {
                            self.consume(1);
                            value = 10*value + to_value(b) as $t;
                        },
                        _ => break
                    }
                }

                value
            }
        }
    };
}

impl_scan_unsigned_integer!(u8);
impl_scan_unsigned_integer!(u16);
impl_scan_unsigned_integer!(u32);
impl_scan_unsigned_integer!(u64);
impl_scan_unsigned_integer!(usize);

impl<R: Read> Scan<char> for CopyingBufReader<R> {
    fn scan(&mut self) -> char {
        let c = self.peek().unwrap() as char;
        self.consume(1);
        c
    }
}

impl<R: Read> Scan<String> for CopyingBufReader<R> {
    fn scan(&mut self) -> String {
        let mut s = String::new();

        if self.peek().unwrap() as char == '\n' {
            self.consume(1);
        }
        loop {
            let c = self.peek().unwrap_or(b'\n') as char;
            if c == '\n' {
                // self.consume(1);
                break;
            } else {
                self.consume(1);
                s.push(c);
            }
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use buf::CopyingBufReader;
    use super::Scan;

    #[test]
    fn scanning_signed_integers_works_correctly() {
        let mut buf = CopyingBufReader::with_capacity(4, &b"8 123 +398 -52112 -3345111111"[..]);
        assert_eq!(Scan::<i8>::scan(&mut buf), 8);
        assert_eq!(Scan::<i16>::scan(&mut buf), 123);
        assert_eq!(Scan::<i32>::scan(&mut buf), 398);
        assert_eq!(Scan::<i64>::scan(&mut buf), -52112);
        assert_eq!(Scan::<isize>::scan(&mut buf), -3345111111);
    }

    #[test]
    #[should_panic]
    fn scanning_signed_integers_panics_correctly_1() {
        let mut buf = CopyingBufReader::with_capacity(4, &b""[..]);
        Scan::<i64>::scan(&mut buf);
    }

    #[test]
    #[should_panic]
    fn scanning_signed_integers_panics_correctly_2() {
        let mut buf = CopyingBufReader::with_capacity(4, &b" "[..]);
        Scan::<i64>::scan(&mut buf);
    }

    #[test]
    #[should_panic]
    fn scanning_signed_integers_panics_correctly_3() {
        let mut buf = CopyingBufReader::with_capacity(4, &b"+ 1"[..]);
        Scan::<i64>::scan(&mut buf);
    }

    #[test]
    #[should_panic]
    fn scanning_signed_integers_panics_correctly_4() {
        let mut buf = CopyingBufReader::with_capacity(4, &b"- 1"[..]);
        Scan::<i64>::scan(&mut buf);
    }

    #[test]
    fn scanning_unsigned_integers_works_correctly() {
        let mut buf = CopyingBufReader::with_capacity(4, &b"8 123 +398 -52112 -3345111111"[..]);
        assert_eq!(Scan::<u8>::scan(&mut buf), 8);
        assert_eq!(Scan::<u16>::scan(&mut buf), 123);
        assert_eq!(Scan::<u32>::scan(&mut buf), 398);
        assert_eq!(Scan::<u64>::scan(&mut buf), 52112);
        assert_eq!(Scan::<usize>::scan(&mut buf), 3345111111);
    }

    #[test]
    #[should_panic]
    fn scanning_unsigned_integers_panics_correctly_1() {
        let mut buf = CopyingBufReader::with_capacity(4, &b""[..]);
        Scan::<u64>::scan(&mut buf);
    }

    #[test]
    #[should_panic]
    fn scanning_unsigned_integers_panics_correctly_2() {
        let mut buf = CopyingBufReader::with_capacity(4, &b" "[..]);
        Scan::<u64>::scan(&mut buf);
    }

    #[test]
    #[should_panic]
    fn scanning_unsigned_integers_panics_correctly_3() {
        let mut buf = CopyingBufReader::with_capacity(4, &b"+ 1"[..]);
        Scan::<u64>::scan(&mut buf);
    }

    #[test]
    fn scanning_chars_works_correctly() {
        let mut buf = CopyingBufReader::with_capacity(4, &b"abcdefghi"[..]);
        assert_eq!(Scan::<char>::scan(&mut buf), 'a');
        assert_eq!(Scan::<char>::scan(&mut buf), 'b');
        assert_eq!(Scan::<char>::scan(&mut buf), 'c');
        assert_eq!(Scan::<char>::scan(&mut buf), 'd');
        assert_eq!(Scan::<char>::scan(&mut buf), 'e');
        assert_eq!(Scan::<char>::scan(&mut buf), 'f');
        assert_eq!(Scan::<char>::scan(&mut buf), 'g');
        assert_eq!(Scan::<char>::scan(&mut buf), 'h');
        assert_eq!(Scan::<char>::scan(&mut buf), 'i');
    }

    #[test]
    fn scanning_strings_works_correctly() {
        let mut buf = CopyingBufReader::with_capacity(4, &b"crazy\npeter\npan\n1\n\nhotdog"[..]);
        assert_eq!(Scan::<String>::scan(&mut buf), String::from("crazy"));
        assert_eq!(Scan::<String>::scan(&mut buf), String::from("peter"));
        assert_eq!(Scan::<String>::scan(&mut buf), String::from("pan"));
        assert_eq!(Scan::<String>::scan(&mut buf), String::from("1"));
        assert_eq!(Scan::<String>::scan(&mut buf), String::new());
        assert_eq!(Scan::<String>::scan(&mut buf), String::from("hotdog"));
    }
}
