use std::io::{Error, ErrorKind, SeekFrom};
use std::pin::Pin;
use std::task::{Context, Poll};

use futures::io::{AsyncRead, AsyncBufRead, AsyncWrite, AsyncSeek};

// use std::ops::{Deref, DerefMut};

use ::tokio::io::{
    AsyncRead as TokioAsyncRead,
    AsyncBufRead as TokioAsyncBufRead,
    AsyncWrite as TokioAsyncWrite,
    AsyncSeek as TokioAsyncSeek,
    ReadBuf,
};



/// Provides compatibility between objects implementing [`tokio`](https://docs.rs/tokio)'s async io traits and
/// the corresponding traits defined by the [`futures`](https://docs.rs/futures) crate.
pub struct TokioCompat<T> {
    inner: T,
    seek_in_progress: bool,
}

impl<T> TokioCompat<T> {
    /// Creates a new instance by wrapping the `inner` object.
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            seek_in_progress: false,
        }
    }

    /// Get a reference to the wrapped object.
    pub fn get_ref(&self) -> &T {
        &self.inner
    }

    /// Get a mutable reference to the wrapped object.
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.inner
    }

    /// Consumes the `TokioCompat` object and returns the wrapped object.
    pub fn into_inner(self) -> T {
        self.inner
    }
}

// impl<T> Deref for TokioCompat<T> {
//     type Target = T;
//
//     fn deref(&self) -> &Self::Target {
//         &self.inner
//     }
// }
//
// impl<T> DerefMut for TokioCompat<T> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.inner
//     }
// }
//
// impl<T> AsRef<T> for TokioCompat<T> {
//     fn as_ref(&self) -> &T {
//         &self.inner
//     }
// }
//
// impl<T> AsMut<T> for TokioCompat<T> {
//     fn as_mut(&mut self) -> &mut T {
//         &mut self.inner
//     }
// }

impl<T> AsyncRead for TokioCompat<T>
where
    T: TokioAsyncRead + Unpin,
{
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8]
    ) -> Poll<Result<usize, Error>> {
        let inner = Pin::into_inner(self);

        let inner = Pin::new(&mut inner.inner);

        let mut buf = ReadBuf::new(buf);
        let filled_len = buf.filled().len();

        match TokioAsyncRead::poll_read(inner, cx, &mut buf) {
            Poll::Pending => return Poll::Pending,
            Poll::Ready(Ok(())) => {
                let filled_len = buf.filled().len()-filled_len;

                return Poll::Ready(Ok(filled_len));
            }
            Poll::Ready(Err(err)) => {
                match err.kind() {
                    ErrorKind::WouldBlock => return Poll::Pending,
                    ErrorKind::Interrupted => return Poll::Ready(Err(Error::new(ErrorKind::Other, "Interrupted."))),
                    _ => return Poll::Ready(Err(err))
                }
            }
        }
    }
}

impl<T> AsyncBufRead for TokioCompat<T>
where
    T: TokioAsyncBufRead + Unpin,
{
    fn poll_fill_buf(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<&[u8], Error>> {
        let inner = Pin::into_inner(self);

        let inner = Pin::new(&mut inner.inner);

        match TokioAsyncBufRead::poll_fill_buf(inner, cx) {
            Poll::Pending => return Poll::Pending,
            Poll::Ready(Ok(buf)) => Poll::Ready(Ok(buf)),
            Poll::Ready(Err(err)) => {
                match err.kind() {
                    ErrorKind::WouldBlock => return Poll::Pending,
                    ErrorKind::Interrupted => return Poll::Ready(Err(Error::new(ErrorKind::Other, "Interrupted."))),
                    _ => return Poll::Ready(Err(err))
                }
            }
        }
    }

    fn consume(self: Pin<&mut Self>, amt: usize) {
        let inner = Pin::into_inner(self);

        let inner = Pin::new(&mut inner.inner);

        TokioAsyncBufRead::consume(inner, amt)
    }
}

impl<T> AsyncWrite for TokioCompat<T>
where
    T: TokioAsyncWrite + Unpin,
{
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8]
    ) -> Poll<Result<usize, Error>> {
        let inner = Pin::into_inner(self);

        let inner = Pin::new(&mut inner.inner);

        match TokioAsyncWrite::poll_write(inner, cx, buf) {
            Poll::Pending => return Poll::Pending,
            Poll::Ready(Ok(n)) => Poll::Ready(Ok(n)),
            Poll::Ready(Err(err)) => {
                match err.kind() {
                    ErrorKind::WouldBlock => return Poll::Pending,
                    ErrorKind::Interrupted => return Poll::Ready(Err(Error::new(ErrorKind::Other, "Interrupted."))),
                    _ => return Poll::Ready(Err(err))
                }
            }
        }
    }

    fn poll_flush(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>
    ) -> Poll<Result<(), Error>> {
        let inner = Pin::into_inner(self);

        let inner = Pin::new(&mut inner.inner);

        match TokioAsyncWrite::poll_flush(inner, cx) {
            Poll::Pending => return Poll::Pending,
            Poll::Ready(Ok(())) => Poll::Ready(Ok(())),
            Poll::Ready(Err(err)) => {
                match err.kind() {
                    ErrorKind::WouldBlock => return Poll::Pending,
                    ErrorKind::Interrupted => return Poll::Ready(Err(Error::new(ErrorKind::Other, "Interrupted."))),
                    _ => return Poll::Ready(Err(err))
                }
            }
        }
    }

    fn poll_close(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>
    ) -> Poll<Result<(), Error>> {
        let inner = Pin::into_inner(self);

        let inner = Pin::new(&mut inner.inner);

        match TokioAsyncWrite::poll_shutdown(inner, cx) {
            Poll::Pending => return Poll::Pending,
            Poll::Ready(Ok(())) => Poll::Ready(Ok(())),
            Poll::Ready(Err(err)) => {
                match err.kind() {
                    ErrorKind::WouldBlock => return Poll::Pending,
                    ErrorKind::Interrupted => return Poll::Ready(Err(Error::new(ErrorKind::Other, "Interrupted."))),
                    _ => return Poll::Ready(Err(err))
                }
            }
        }
    }
}

impl<T> AsyncSeek for TokioCompat<T>
where
    T: TokioAsyncSeek + Unpin,
{
    fn poll_seek(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        pos: SeekFrom,
    ) -> Poll<Result<u64, Error>> {
        let inner = Pin::into_inner(self);

        if !inner.seek_in_progress {
            if let Err(err) = Pin::new(&mut inner.inner).start_seek(pos) {
                return Poll::Ready(Err(err));
            }

            inner.seek_in_progress = true;
        }

        match TokioAsyncSeek::poll_complete(Pin::new(&mut inner.inner), cx) {
            Poll::Pending => return Poll::Pending,
            Poll::Ready(result) => {
                inner.seek_in_progress = false;

                match result {
                    Ok(pos) => return Poll::Ready(Ok(pos)),
                    Err(err) => {
                        match err.kind() {
                            ErrorKind::WouldBlock => return Poll::Pending,
                            ErrorKind::Interrupted => return Poll::Ready(Err(Error::new(ErrorKind::Other, "Interrupted."))),
                            _ => return Poll::Ready(Err(err))
                        }
                    }
                }
            }
        }
    }
}
