use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::error::IoError;
use crate::io::{AsyncRead, AsyncWrite};

/// Adapter for [`tokio::io::AsyncRead`] that produces [`AsyncRead`].
pub struct TokioReader<R: tokio::io::AsyncRead> {
    reader: R,
}

impl<R: tokio::io::AsyncRead> TokioReader<R> {
    /// Wraps an instance of [`tokio::io::AsyncRead`].
    #[inline(always)]
    pub fn new(reader: R) -> Self {
        Self { reader }
    }

    /// Extracts an instance of [`tokio::io::AsyncRead`].
    #[inline(always)]
    pub fn extract(self) -> R {
        self.reader
    }
}

impl<R: tokio::io::AsyncRead> From<R> for TokioReader<R> {
    fn from(value: R) -> Self {
        Self::new(value)
    }
}

impl<R: tokio::io::AsyncRead + Unpin> AsyncRead<IoError> for TokioReader<R> {
    #[inline(always)]
    async fn read_exact<'a>(&'a mut self, buf: &'a mut [u8]) -> Result<(), IoError> {
        self.reader.read_exact(buf).await.map_err(IoError::from)?;
        Ok(())
    }
}

/// Adapter for [`tokio::io::AsyncWrite`] that produces [`AsyncWrite`].
pub struct TokioWriter<W: tokio::io::AsyncWrite> {
    writer: W,
}

impl<W: tokio::io::AsyncWrite> TokioWriter<W> {
    /// Wraps an instance of [`tokio::io::AsyncWrite`].
    #[inline(always)]
    pub fn new(writer: W) -> Self {
        Self { writer }
    }

    /// Extracts an instance of [`tokio::io::AsyncWrite`].
    #[inline(always)]
    pub fn extract(self) -> W {
        self.writer
    }
}

impl<W: tokio::io::AsyncWrite + Unpin> AsyncWrite<IoError> for TokioWriter<W> {
    #[inline(always)]
    async fn write_all<'a>(&'a mut self, buf: &'a [u8]) -> Result<(), IoError> {
        self.writer.write_all(buf).await.map_err(IoError::from)
    }

    #[inline]
    async fn flush<'a>(&'a mut self) -> Result<(), IoError> {
        self.writer.flush().await.map_err(IoError::from)
    }
}
