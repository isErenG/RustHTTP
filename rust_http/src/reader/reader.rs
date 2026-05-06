use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::{AsyncBufRead, AsyncRead, ReadBuf};

pub struct CustomAsyncReader<R> {
    source: R,
    buf: [u8; 1024],
    start: usize,
    end: usize,
}

impl<R: AsyncRead + Unpin> CustomAsyncReader<R> {
    pub fn new(source: R) -> CustomAsyncReader<R> {
        Self {
            source,
            buf: [0; 1024],
            start: 0,
            end: 0,
        }
    }
}

impl<R: AsyncRead + Unpin> AsyncRead for CustomAsyncReader<R> {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        let me = self.get_mut();

        if me.start == me.end {
            let mut read_buf = ReadBuf::new(&mut me.buf);
            match Pin::new(&mut me.source).poll_read(cx, &mut read_buf) {
                Poll::Pending => return Poll::Pending,
                Poll::Ready(Err(e)) => return Poll::Ready(Err(e)),
                Poll::Ready(Ok(())) => {
                    me.end = read_buf.filled().len();
                    me.start = 0;
                }
            }
        }

        let available = &me.buf[me.start..me.end];
        let n = available.len().min(buf.remaining());
        buf.put_slice(&available[..n]);
        me.start += n;
        Poll::Ready(Ok(()))
    }
}

impl<R: AsyncRead + Unpin> AsyncBufRead for CustomAsyncReader<R> {
    fn poll_fill_buf(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<&[u8]>> {
        let me = self.get_mut();

        if me.start == me.end {
            let mut read_buf = ReadBuf::new(&mut me.buf);
            match Pin::new(&mut me.source).poll_read(cx, &mut read_buf) {
                Poll::Pending => return Poll::Pending,
                Poll::Ready(Err(e)) => return Poll::Ready(Err(e)),
                Poll::Ready(Ok(())) => {
                    me.end = read_buf.filled().len();
                    me.start = 0;
                }
            }
        }

        Poll::Ready(Ok(&me.buf[me.start..me.end]))
    }

    fn consume(mut self: Pin<&mut Self>, amt: usize) {
        let me = self.get_mut();
        me.start = (me.start + amt).min(me.end);
    }
}
