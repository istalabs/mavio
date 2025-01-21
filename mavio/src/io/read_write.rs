//! Synchronous [`Read`] / [`Write`] interfaces.

use crate::error::Error;

/// Generic read trait similar to [`std::io::Read`].
///
/// The instances of [`Read`] are required to construct [`Receiver`] that can read
/// MavLink frames.
///
/// Instead of relying on a particular definition of read trait, we allow users to use any library
/// with I/O capabilities.
///
/// Wrappers are available for specific I/O implementations:
///
/// - [`StdIoReader`] for [`std::io::Read`]
/// - [`EmbeddedIoReader`] for [`embedded_io::Read`]
///
/// [`Receiver`]: crate::io::Receiver
/// [`StdIoReader`]: crate::io::StdIoReader
/// [`EmbeddedIoReader`]: crate::io::EmbeddedIoReader
pub trait Read<Err: Into<Error>> {
    /// Read the exact number of bytes required to fill buffer.
    ///
    /// Mimics the corresponding method from [`std::io::Read`].
    ///
    /// # Errors
    ///
    /// Returns generic error in case of I/O failure.
    fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), Err>;
}

/// Generic write trait similar to [`std::io::Write`].
///
/// The instances of [`Write`] are required to construct [`Sender`] that can write
/// MavLink frames.
///
/// Instead of relying on a particular definition of write trait, we allow users to use any library
/// with I/O capabilities.
///
/// Wrappers are available for specific I/O implementations:
///
/// - [`StdIoWriter`] for [`std::io::Write`]
/// - [`EmbeddedIoWriter`] for [`embedded_io::Write`]
///
/// [`Sender`]: crate::io::Sender
/// [`StdIoWriter`]: crate::io::StdIoWriter
/// [`EmbeddedIoWriter`]: crate::io::EmbeddedIoWriter
pub trait Write<Err: Into<Error>> {
    /// Attempts to write an entire buffer into this writer.
    ///
    /// Mimics the corresponding method from [`std::io::Write`].
    ///
    /// # Errors
    ///
    /// Returns generic error in case of I/O failure.
    fn write_all(&mut self, buf: &[u8]) -> Result<(), Err>;

    /// Flush this output stream, ensuring that all intermediately buffered
    /// contents reach their destination.
    fn flush(&mut self) -> Result<(), Err>;
}
