use std::io;
use std::io::{BufRead, Read};

// <R> = placeholder for any type — decided when you create a CustomReader
pub struct CustomReader<R> {
    source: R, // R: whatever type was passed in — no bound needed, we're just storing it
    buf: [u8; 1024],
    start: usize,
    end: usize,
}

// <R: Read> = R can be any type, but it must implement Read so we can call .read() on it
impl<R: Read> CustomReader<R> {
    // source: R = takes ownership of whatever R is (could be TcpStream or &mut TcpStream)
    // -> CustomReader<R> = returns a CustomReader holding that same R
    pub fn new(source: R) -> CustomReader<R> {
        Self {
            source,
            buf: [0; 1024],
            start: 0,
            end: 0,
        }
    }
}

impl<R: Read> Read for CustomReader<R> {
    // &mut self = we borrow this CustomReader with permission to modify it (we advance self.start)
    // out: &mut [u8] = caller lends us a byte slice to write into — &mut so we can fill it
    fn read(&mut self, out: &mut [u8]) -> io::Result<usize> {
        self.fill_buf()?;
        // & = borrow a read-only slice of our internal buffer (no copy, no move)
        let available = &self.buf[self.start..self.end];
        let n = available.len().min(out.len());
        // & = borrow a read-only slice of available to copy from
        out[..n].copy_from_slice(&available[..n]);
        self.start += n;
        Ok(n)
    }
}

impl<R: Read> BufRead for CustomReader<R> {
    // &mut self = we need mut because we may refill self.buf and reset self.start/end
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        if self.start == self.end {
            // &mut self.buf = lend our buffer to .read() with write permission so it can fill it
            self.end = self.source.read(&mut self.buf)?;
            self.start = 0;
        }
        // & = return a read-only borrow into our internal buffer (caller can look, not own)
        Ok(&self.buf[self.start..self.end])
    }

    // &mut self = we modify self.start to mark bytes as consumed
    fn consume(&mut self, amt: usize) {
        self.start = (self.start + amt).min(self.end);
    }

    // &mut self = we call fill_buf and consume which both require mutation
    // buf: &mut String = caller lends us their String with write permission so we can append to it
    fn read_line(&mut self, buf: &mut String) -> io::Result<usize> {
        let mut total = 0; // mut = we add to this each iteration

        loop {
            let filled = self.fill_buf()?;

            if filled.is_empty() {
                break;
            }

            let (slice, done) = match filled.iter().position(|&b| b == b'\n') {
                Some(i) => (&filled[..=i], true), // & = borrow a slice of filled, no copy
                None => (filled, false),          // filled is already a &[u8] borrow
            };

            let s = String::from_utf8_lossy(slice);
            // & = push_str takes &str (a borrow), not an owned String
            buf.push_str(&s);

            let n = slice.len();
            self.consume(n);
            total += n;

            if done {
                break;
            }
        }

        Ok(total)
    }
}
