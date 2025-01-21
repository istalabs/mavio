use crate::io::{Read, Write};

/// Adapter for [`std::io::Read`] that produces [`Read`].
///
/// # Examples
///
/// Construct a receiver from [`std::net::TcpStream`].
///
/// ```rust,no_run
/// # #[cfg(not(feature = "std"))]
/// # fn main() {}
/// # #[cfg(feature = "std")]
/// # fn main() -> mavio::error::Result<()> {
/// use std::net::TcpStream;
/// use mavio::prelude::*;
///
/// // Create a TCP client receiver
/// let reader = StdIoReader::new(TcpStream::connect("0.0.0.0:5600")?);
/// let mut receiver = Receiver::versionless(reader);
///
/// # Ok(())
/// # }
/// ```
pub struct StdIoReader<R: std::io::Read> {
    reader: R,
}

impl<R: std::io::Read> StdIoReader<R> {
    /// Wraps an instance of [`std::io::Read`].
    #[inline(always)]
    pub fn new(reader: R) -> Self {
        Self { reader }
    }

    /// Extracts an instance of [`std::io::Read`].
    ///
    /// # Examples
    ///
    /// Wrap [`std::net::TcpStream`] as reader and extract it later.
    ///
    /// ```rust,no_run
    /// # #[cfg(not(feature = "std"))]
    /// # fn main() {}
    /// # #[cfg(feature = "std")]
    /// # fn main() -> mavio::error::Result<()> {
    /// use std::net::TcpStream;
    /// use mavio::io::StdIoReader;
    ///
    /// // Create a TCP client receiver
    /// let wrapped_reader = StdIoReader::new(TcpStream::connect("0.0.0.0:5600")?);
    /// /* perform operations */
    /// let reader = wrapped_reader.extract();
    ///
    /// # Ok(())
    /// # }
    /// ```
    #[inline(always)]
    pub fn extract(self) -> R {
        self.reader
    }
}

impl<R: std::io::Read> From<R> for StdIoReader<R> {
    fn from(value: R) -> Self {
        Self::new(value)
    }
}

impl<R: std::io::Read> Read<std::io::Error> for StdIoReader<R> {
    #[inline(always)]
    fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), std::io::Error> {
        self.reader.read_exact(buf)
    }
}

/// Adapter for [`std::io::Write`] that produces [`Write`].
///
/// # Examples
///
/// Construct a sender from [`std::net::TcpStream`].
///
/// ```rust,no_run
/// # #[cfg(not(feature = "std"))]
/// # fn main() {}
/// # #[cfg(feature = "std")]
/// # fn main() -> mavio::error::Result<()> {
/// use std::net::TcpStream;
/// use mavio::prelude::*;
///
/// // Create a TCP client sender
/// let writer = StdIoWriter::new(TcpStream::connect("0.0.0.0:5600")?);
/// let mut sender = Sender::versionless(writer);
///
/// # Ok(())
/// # }
/// ```
pub struct StdIoWriter<W: std::io::Write> {
    writer: W,
}

impl<W: std::io::Write> StdIoWriter<W> {
    /// Wraps an instance of [`std::io::Write`].
    #[inline(always)]
    pub fn new(writer: W) -> Self {
        Self { writer }
    }

    /// Extracts an instance of [`std::io::Write`].
    ///
    /// # Examples
    ///
    /// Wrap [`std::net::TcpStream`] as writer and extract it later.
    ///
    /// ```rust,no_run
    /// # #[cfg(not(feature = "std"))]
    /// # fn main() {}
    /// # #[cfg(feature = "std")]
    /// # fn main() -> mavio::error::Result<()> {
    /// use std::net::TcpStream;
    /// use mavio::io::StdIoWriter;
    ///
    /// // Create a TCP client receiver
    /// let wrapped_writer = StdIoWriter::new(TcpStream::connect("0.0.0.0:5600")?);
    /// /* perform operations */
    /// let writer = wrapped_writer.extract();
    ///
    /// # Ok(())
    /// # }
    /// ```
    #[inline(always)]
    pub fn extract(self) -> W {
        self.writer
    }
}

impl<W: std::io::Write> From<W> for StdIoWriter<W> {
    fn from(value: W) -> Self {
        Self::new(value)
    }
}

impl<W: std::io::Write> Write<std::io::Error> for StdIoWriter<W> {
    #[inline(always)]
    fn write_all(&mut self, buf: &[u8]) -> Result<(), std::io::Error> {
        self.writer.write_all(buf)
    }

    #[inline]
    fn flush(&mut self) -> Result<(), std::io::Error> {
        self.writer.flush()
    }
}
