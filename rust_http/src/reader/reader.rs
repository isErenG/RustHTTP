use std::io;
use std::io::{BufRead, Read};
use std::net::TcpStream;

pub struct CustomReader {
    source: TcpStream,
    buf: [u8; 1024],
    start: usize,
    end: usize,
}

impl CustomReader {
    pub fn new(source: TcpStream) -> CustomReader {
        Self {
            source,
            buf: [0; 1024],
            start: 0,
            end: 0,
        }
    }
}

impl Read for CustomReader {
    fn read(&mut self, out: &mut [u8]) -> io::Result<usize> {
        self.fill_buf()?;
        let available = &self.buf[self.start..self.end];
        let n = available.len().min(out.len());
        out[..n].copy_from_slice(&available[..n]);
        self.start += n; // advance cursor
        Ok(n)
    }
}

impl BufRead for CustomReader {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        if self.start == self.end {
            self.end = self.source.read(&mut self.buf)?;
            self.start = 0;
        }
        Ok(&self.buf[self.start..self.end])
    }

    fn consume(&mut self, amt: usize) {
        self.start = (self.start + amt).min(self.end);
    }

    fn read_line(&mut self, buf: &mut String) -> io::Result<usize> {
        let mut total = 0;

        loop {
            let filled = self.fill_buf()?;

            if filled.is_empty() {
                break;
            }

            let (slice, done) = match filled.iter().position(|&b| b == b'\n') {
                Some(i) => (&filled[..=i], true),
                None => (filled, false),
            };

            let s = String::from_utf8_lossy(slice);
            buf.push_str(&s);

            let n = slice.len();
            self.consume(n);
            total+=n;

            if done {
                break;
            }

        }

        return Ok(total);
    }
}
