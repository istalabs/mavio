use embedded_io::ReadExactError;

use crate::error::{IoError, IoErrorKind};
use crate::io::{Read, Write};

/// Adapter for [`embedded_io::Read`] that produces [`Read`].
pub struct EmbeddedIoReader<R: embedded_io::Read> {
    reader: R,
}

impl<R: embedded_io::Read> EmbeddedIoReader<R> {
    /// Wraps an instance of [`embedded_io::Read`].
    #[inline(always)]
    pub fn new(reader: R) -> Self {
        Self { reader }
    }

    /// Extracts an instance of [`embedded_io::Read`].
    #[inline(always)]
    pub fn extract(self) -> R {
        self.reader
    }
}

impl<R: embedded_io::Read> From<R> for EmbeddedIoReader<R> {
    fn from(value: R) -> Self {
        Self::new(value)
    }
}

impl<R: embedded_io::Read> Read<IoError> for EmbeddedIoReader<R> {
    #[inline(always)]
    fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), IoError> {
        self.reader.read_exact(buf).map_err(|err| match err {
            ReadExactError::UnexpectedEof => IoError::from(IoErrorKind::UnexpectedEof),
            ReadExactError::Other(err) => IoError::from_embedded_io_error(err),
        })
    }
}

/// Adapter for [`embedded_io::Write`] that produces [`Write`].
pub struct EmbeddedIoWriter<W: embedded_io::Write> {
    writer: W,
}

impl<W: embedded_io::Write> EmbeddedIoWriter<W> {
    /// Wraps an instance of [`embedded_io::Write`].
    #[inline(always)]
    pub fn new(writer: W) -> Self {
        Self { writer }
    }

    /// Extracts an instance of [`embedded_io::Write`].
    #[inline(always)]
    pub fn extract(self) -> W {
        self.writer
    }
}

impl<W: embedded_io::Write> Write<IoError> for EmbeddedIoWriter<W> {
    #[inline(always)]
    fn write_all(&mut self, buf: &[u8]) -> Result<(), IoError> {
        self.writer
            .write_all(buf)
            .map_err(IoError::from_embedded_io_error)
    }

    #[inline]
    fn flush(&mut self) -> Result<(), IoError> {
        self.writer.flush().map_err(IoError::from_embedded_io_error)
    }
}
