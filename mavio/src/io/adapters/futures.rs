use futures::io::{AsyncReadExt, AsyncWriteExt};

use crate::error::IoError;
use crate::io::{AsyncRead, AsyncWrite};

/// Adapter for [`futures::io::AsyncRead`] that produces [`AsyncRead`].
pub struct FuturesReader<R: futures::io::AsyncRead> {
    reader: R,
}

impl<R: futures::io::AsyncRead> FuturesReader<R> {
    /// Wraps an instance of [`futures::io::AsyncRead`].
    #[inline(always)]
    pub fn new(reader: R) -> Self {
        Self { reader }
    }

    /// Extracts an instance of [`futures::io::AsyncRead`].
    #[inline(always)]
    pub fn extract(self) -> R {
        self.reader
    }
}

impl<R: futures::io::AsyncRead> From<R> for FuturesReader<R> {
    fn from(value: R) -> Self {
        Self::new(value)
    }
}

impl<R: futures::io::AsyncRead + Unpin> AsyncRead<IoError> for FuturesReader<R> {
    #[inline(always)]
    async fn read_exact<'a>(&'a mut self, buf: &'a mut [u8]) -> Result<(), IoError> {
        self.reader.read_exact(buf).await.map_err(IoError::from)?;
        Ok(())
    }
}

/// Adapter for [`futures::io::AsyncWrite`] that produces [`AsyncWrite`].
pub struct FuturesWriter<W: futures::io::AsyncWrite> {
    writer: W,
}

impl<W: futures::io::AsyncWrite> FuturesWriter<W> {
    /// Wraps an instance of [`futures::io::AsyncWrite`].
    #[inline(always)]
    pub fn new(writer: W) -> Self {
        Self { writer }
    }

    /// Extracts an instance of [`futures::io::AsyncWrite`].
    #[inline(always)]
    pub fn extract(self) -> W {
        self.writer
    }
}

impl<W: futures::io::AsyncWrite + Unpin> AsyncWrite<IoError> for FuturesWriter<W> {
    #[inline(always)]
    async fn write_all<'a>(&'a mut self, buf: &'a [u8]) -> Result<(), IoError> {
        self.writer.write_all(buf).await.map_err(IoError::from)
    }

    #[inline]
    async fn flush<'a>(&'a mut self) -> Result<(), IoError> {
        self.writer.flush().await.map_err(IoError::from)
    }
}
