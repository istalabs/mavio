use embedded_io_async::ReadExactError;

use crate::error::{IoError, IoErrorKind};
use crate::io::{AsyncRead, AsyncWrite};

/// Adapter for [`embedded_io_async::Read`] that produces [`AsyncRead`].
pub struct EmbeddedIoAsyncReader<R: embedded_io_async::Read> {
    reader: R,
}

impl<R: embedded_io_async::Read> EmbeddedIoAsyncReader<R> {
    /// Wraps an instance of [`embedded_io_async::Read`].
    #[inline(always)]
    pub fn new(reader: R) -> Self {
        Self { reader }
    }

    /// Extracts an instance of [`embedded_io_async::Read`].
    #[inline(always)]
    pub fn extract(self) -> R {
        self.reader
    }
}

impl<R: embedded_io_async::Read> From<R> for EmbeddedIoAsyncReader<R> {
    fn from(value: R) -> Self {
        Self::new(value)
    }
}

impl<R: embedded_io_async::Read> AsyncRead<IoError> for EmbeddedIoAsyncReader<R> {
    #[inline(always)]
    async fn read_exact<'a>(&'a mut self, buf: &'a mut [u8]) -> Result<(), IoError> {
        self.reader.read_exact(buf).await.map_err(|err| match err {
            ReadExactError::UnexpectedEof => IoError::from(IoErrorKind::UnexpectedEof),
            ReadExactError::Other(err) => IoError::from_embedded_io_error(err),
        })
    }
}

/// Adapter for [`embedded_io_async::Write`] that produces [`AsyncWrite`].
pub struct EmbeddedIoAsyncWriter<W: embedded_io_async::Write> {
    writer: W,
}

impl<W: embedded_io_async::Write> EmbeddedIoAsyncWriter<W> {
    /// Wraps an instance of [`embedded_io_async::Write`].
    #[inline(always)]
    pub fn new(writer: W) -> Self {
        Self { writer }
    }

    /// Extracts an instance of [`embedded_io_async::Write`].
    #[inline(always)]
    pub fn extract(self) -> W {
        self.writer
    }
}

impl<W: embedded_io_async::Write> AsyncWrite<IoError> for EmbeddedIoAsyncWriter<W> {
    #[inline(always)]
    async fn write_all<'a>(&'a mut self, buf: &'a [u8]) -> Result<(), IoError> {
        self.writer
            .write_all(buf)
            .await
            .map_err(IoError::from_embedded_io_error)
    }

    #[inline]
    async fn flush(&mut self) -> Result<(), IoError> {
        self.writer
            .flush()
            .await
            .map_err(IoError::from_embedded_io_error)
    }
}
