/// Printing


#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => { {
        use std::io::Write;
        writeln!($crate::stdout(), $($arg)*).unwrap();
    } }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => { {
        use std::io::Write;
        write!($crate::stdout(), $($arg)*).unwrap();
    } }
}

#[macro_export]
macro_rules! flush {
    () => { $crate::flush() }
}


/// Reading


#[macro_export]
macro_rules! readchar {
    () => { $crate::read_char() }
}

#[macro_export]
macro_rules! readbyte {
    () => { $crate::read_byte() }
}


/// Scanning


#[macro_export]
macro_rules! scan {
    ($arg:tt) => { $crate::scan::<$arg>() };
    ($($arg:tt)+) => { ($($crate::scan::<$arg>(),)*) };
}
