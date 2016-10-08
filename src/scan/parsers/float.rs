use nom::digit;

named!(sign, recognize!(one_of!("+-")));
named!(dot, recognize!(char!('.')));

named!(float_mantissa, recognize!(
    alt!(
        chain!(digit ~ tuple!(dot, opt!(digit))?, || {}) |
        chain!(dot ~ digit?, || {})
    )
));

named!(float_exponent, recognize!(
    chain!(one_of!("eE") ~ sign? ~ digit, || {})
));

named!(pub float, recognize!(
    chain!(sign? ~ float_mantissa ~ float_exponent?, || {})
));


#[cfg(test)]
mod tests {
    use nom::IResult::*;
    use super::float;

    #[test]
    fn it_scans_floats() {
        assert_eq!(float(b"123 "),         Done(&b" "[..], &b"123"[..]));
        assert_eq!(float(b"123. "),        Done(&b" "[..], &b"123."[..]));
        assert_eq!(float(b"123.45 "),      Done(&b" "[..], &b"123.45"[..]));
        assert_eq!(float(b".45 "),         Done(&b" "[..], &b".45"[..]));
        assert_eq!(float(b"123e7 "),       Done(&b" "[..], &b"123e7"[..]));
        assert_eq!(float(b"123.e7 "),      Done(&b" "[..], &b"123.e7"[..]));
        assert_eq!(float(b"123.45e7 "),    Done(&b" "[..], &b"123.45e7"[..]));
        assert_eq!(float(b".45e7 "),       Done(&b" "[..], &b".45e7"[..]));
        assert_eq!(float(b"123e+7 "),      Done(&b" "[..], &b"123e+7"[..]));
        assert_eq!(float(b"123.e+7 "),     Done(&b" "[..], &b"123.e+7"[..]));
        assert_eq!(float(b"123.45e+7 "),   Done(&b" "[..], &b"123.45e+7"[..]));
        assert_eq!(float(b".45e+7 "),      Done(&b" "[..], &b".45e+7"[..]));
        assert_eq!(float(b"123e-7 "),      Done(&b" "[..], &b"123e-7"[..]));
        assert_eq!(float(b"123.e-7 "),     Done(&b" "[..], &b"123.e-7"[..]));
        assert_eq!(float(b"123.45e-7 "),   Done(&b" "[..], &b"123.45e-7"[..]));
        assert_eq!(float(b".45e-7 "),      Done(&b" "[..], &b".45e-7"[..]));
        assert_eq!(float(b"123E7 "),       Done(&b" "[..], &b"123E7"[..]));
        assert_eq!(float(b"123.E7 "),      Done(&b" "[..], &b"123.E7"[..]));
        assert_eq!(float(b"123.45E7 "),    Done(&b" "[..], &b"123.45E7"[..]));
        assert_eq!(float(b".45E7 "),       Done(&b" "[..], &b".45E7"[..]));
        assert_eq!(float(b"123E+7 "),      Done(&b" "[..], &b"123E+7"[..]));
        assert_eq!(float(b"123.E+7 "),     Done(&b" "[..], &b"123.E+7"[..]));
        assert_eq!(float(b"123.45E+7 "),   Done(&b" "[..], &b"123.45E+7"[..]));
        assert_eq!(float(b".45E+7 "),      Done(&b" "[..], &b".45E+7"[..]));
        assert_eq!(float(b"123E-7 "),      Done(&b" "[..], &b"123E-7"[..]));
        assert_eq!(float(b"123.E-7 "),     Done(&b" "[..], &b"123.E-7"[..]));
        assert_eq!(float(b"123.45E-7 "),   Done(&b" "[..], &b"123.45E-7"[..]));
        assert_eq!(float(b".45E-7 "),      Done(&b" "[..], &b".45E-7"[..]));

        assert_eq!(float(b"+123 "),         Done(&b" "[..], &b"+123"[..]));
        assert_eq!(float(b"+123. "),        Done(&b" "[..], &b"+123."[..]));
        assert_eq!(float(b"+123.45 "),      Done(&b" "[..], &b"+123.45"[..]));
        assert_eq!(float(b"+.45 "),         Done(&b" "[..], &b"+.45"[..]));
        assert_eq!(float(b"+123e7 "),       Done(&b" "[..], &b"+123e7"[..]));
        assert_eq!(float(b"+123.e7 "),      Done(&b" "[..], &b"+123.e7"[..]));
        assert_eq!(float(b"+123.45e7 "),    Done(&b" "[..], &b"+123.45e7"[..]));
        assert_eq!(float(b"+.45e7 "),       Done(&b" "[..], &b"+.45e7"[..]));
        assert_eq!(float(b"+123e+7 "),      Done(&b" "[..], &b"+123e+7"[..]));
        assert_eq!(float(b"+123.e+7 "),     Done(&b" "[..], &b"+123.e+7"[..]));
        assert_eq!(float(b"+123.45e+7 "),   Done(&b" "[..], &b"+123.45e+7"[..]));
        assert_eq!(float(b"+.45e+7 "),      Done(&b" "[..], &b"+.45e+7"[..]));
        assert_eq!(float(b"+123e-7 "),      Done(&b" "[..], &b"+123e-7"[..]));
        assert_eq!(float(b"+123.e-7 "),     Done(&b" "[..], &b"+123.e-7"[..]));
        assert_eq!(float(b"+123.45e-7 "),   Done(&b" "[..], &b"+123.45e-7"[..]));
        assert_eq!(float(b"+.45e-7 "),      Done(&b" "[..], &b"+.45e-7"[..]));
        assert_eq!(float(b"+123E7 "),       Done(&b" "[..], &b"+123E7"[..]));
        assert_eq!(float(b"+123.E7 "),      Done(&b" "[..], &b"+123.E7"[..]));
        assert_eq!(float(b"+123.45E7 "),    Done(&b" "[..], &b"+123.45E7"[..]));
        assert_eq!(float(b"+.45E7 "),       Done(&b" "[..], &b"+.45E7"[..]));
        assert_eq!(float(b"+123E+7 "),      Done(&b" "[..], &b"+123E+7"[..]));
        assert_eq!(float(b"+123.E+7 "),     Done(&b" "[..], &b"+123.E+7"[..]));
        assert_eq!(float(b"+123.45E+7 "),   Done(&b" "[..], &b"+123.45E+7"[..]));
        assert_eq!(float(b"+.45E+7 "),      Done(&b" "[..], &b"+.45E+7"[..]));
        assert_eq!(float(b"+123E-7 "),      Done(&b" "[..], &b"+123E-7"[..]));
        assert_eq!(float(b"+123.E-7 "),     Done(&b" "[..], &b"+123.E-7"[..]));
        assert_eq!(float(b"+123.45E-7 "),   Done(&b" "[..], &b"+123.45E-7"[..]));
        assert_eq!(float(b"+.45E-7 "),      Done(&b" "[..], &b"+.45E-7"[..]));

        assert_eq!(float(b"-123 "),         Done(&b" "[..], &b"-123"[..]));
        assert_eq!(float(b"-123. "),        Done(&b" "[..], &b"-123."[..]));
        assert_eq!(float(b"-123.45 "),      Done(&b" "[..], &b"-123.45"[..]));
        assert_eq!(float(b"-.45 "),         Done(&b" "[..], &b"-.45"[..]));
        assert_eq!(float(b"-123e7 "),       Done(&b" "[..], &b"-123e7"[..]));
        assert_eq!(float(b"-123.e7 "),      Done(&b" "[..], &b"-123.e7"[..]));
        assert_eq!(float(b"-123.45e7 "),    Done(&b" "[..], &b"-123.45e7"[..]));
        assert_eq!(float(b"-.45e7 "),       Done(&b" "[..], &b"-.45e7"[..]));
        assert_eq!(float(b"-123e+7 "),      Done(&b" "[..], &b"-123e+7"[..]));
        assert_eq!(float(b"-123.e+7 "),     Done(&b" "[..], &b"-123.e+7"[..]));
        assert_eq!(float(b"-123.45e+7 "),   Done(&b" "[..], &b"-123.45e+7"[..]));
        assert_eq!(float(b"-.45e+7 "),      Done(&b" "[..], &b"-.45e+7"[..]));
        assert_eq!(float(b"-123e-7 "),      Done(&b" "[..], &b"-123e-7"[..]));
        assert_eq!(float(b"-123.e-7 "),     Done(&b" "[..], &b"-123.e-7"[..]));
        assert_eq!(float(b"-123.45e-7 "),   Done(&b" "[..], &b"-123.45e-7"[..]));
        assert_eq!(float(b"-.45e-7 "),      Done(&b" "[..], &b"-.45e-7"[..]));
        assert_eq!(float(b"-123E7 "),       Done(&b" "[..], &b"-123E7"[..]));
        assert_eq!(float(b"-123.E7 "),      Done(&b" "[..], &b"-123.E7"[..]));
        assert_eq!(float(b"-123.45E7 "),    Done(&b" "[..], &b"-123.45E7"[..]));
        assert_eq!(float(b"-.45E7 "),       Done(&b" "[..], &b"-.45E7"[..]));
        assert_eq!(float(b"-123E+7 "),      Done(&b" "[..], &b"-123E+7"[..]));
        assert_eq!(float(b"-123.E+7 "),     Done(&b" "[..], &b"-123.E+7"[..]));
        assert_eq!(float(b"-123.45E+7 "),   Done(&b" "[..], &b"-123.45E+7"[..]));
        assert_eq!(float(b"-.45E+7 "),      Done(&b" "[..], &b"-.45E+7"[..]));
        assert_eq!(float(b"-123E-7 "),      Done(&b" "[..], &b"-123E-7"[..]));
        assert_eq!(float(b"-123.E-7 "),     Done(&b" "[..], &b"-123.E-7"[..]));
        assert_eq!(float(b"-123.45E-7 "),   Done(&b" "[..], &b"-123.45E-7"[..]));
        assert_eq!(float(b"-.45E-7 "),      Done(&b" "[..], &b"-.45E-7"[..]));
    }

    #[test]
    fn it_scans_weird_floats() {
        assert_eq!(float(b"1+1 "),          Done(&b"+1 "[..], &b"1"[..]));
        assert_eq!(float(b"-.45.6E-7 "),    Done(&b".6E-7 "[..], &b"-.45"[..]));
    }
}
