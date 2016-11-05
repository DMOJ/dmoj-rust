use std::ptr;
use std::io::Read;

use io::DEFAULT_BUF_SIZE;

pub struct CopyingBufReader<R> {
    inner: R,
    pub buf: Box<[u8]>,
    pub pos: usize,
    pub amt: usize
}

impl<R: Read> CopyingBufReader<R> {
    pub fn new(inner: R) -> CopyingBufReader<R> {
        CopyingBufReader::with_capacity(DEFAULT_BUF_SIZE, inner)
    }

    pub fn with_capacity(capacity: usize, inner: R) -> CopyingBufReader<R> {
        CopyingBufReader {
            inner: inner,
            buf: vec![0; capacity].into_boxed_slice(),
            pos: 0,
            amt: 0
        }
    }

    pub fn refill(&mut self) {
        let buf_kept = self.amt - self.pos;
        let buf_len = self.buf.len();

        unsafe {
            ptr::copy(
                self.buf.as_ptr().offset(self.pos as isize),
                self.buf.as_mut_ptr(),
                buf_kept
            );
        }

        self.amt = buf_kept + self.inner.read(&mut self.buf[buf_kept..buf_len]).unwrap();
        self.pos = 0;
    }

    pub fn peek(&mut self) -> Option<u8> {
        if self.pos == self.amt {
            self.refill();
        }
        
        if self.amt > 0 {
            Some(unsafe { *self.buf.get_unchecked(self.pos) })
        } else {
            None
        }
    }

    pub fn consume(&mut self, amt: usize) {
        assert!(self.pos + amt <= self.amt);
        self.pos += amt;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut buf = CopyingBufReader::with_capacity(4, &b"abcdefghijkl"[..]);
        assert_eq!(buf.peek().unwrap(), b'a');
        assert_eq!(buf.peek().unwrap(), b'a');
        buf.consume(2);
        assert_eq!(buf.peek().unwrap(), b'c');
        buf.consume(1);
        assert_eq!(buf.peek().unwrap(), b'd');
        buf.consume(1);
        assert_eq!(buf.peek().unwrap(), b'e');
        buf.consume(1);
        assert_eq!(buf.peek().unwrap(), b'f');
        buf.refill();
        assert_eq!(buf.peek().unwrap(), b'f');
    }
}
