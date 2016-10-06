use nom::digit;

named!(sign, recognize!(one_of!("+-")));
named!(plus, recognize!(char!('+')));

named!(pub unsigned_integer, recognize!(
    chain!(plus? ~ digit, || {})
));

named!(pub signed_integer, recognize!(
    chain!(sign? ~ digit, || {})
));

#[cfg(test)]
mod tests {
    use nom::IResult::*;
    use super::{unsigned_integer, signed_integer};

    #[test]
    fn it_scans_unsigned_integers() {
        assert_eq!(unsigned_integer(b"123 "),   Done(&b" "[..], &b"123"[..]));
        assert_eq!(unsigned_integer(b"+123 "),  Done(&b" "[..], &b"+123"[..]));
    }

    #[test]
    fn it_scans_signed_integers() {
        assert_eq!(signed_integer(b"123 "),     Done(&b" "[..], &b"123"[..]));
        assert_eq!(signed_integer(b"+123 "),    Done(&b" "[..], &b"+123"[..]));
        assert_eq!(signed_integer(b"-123 "),    Done(&b" "[..], &b"-123"[..]));
    }
}
