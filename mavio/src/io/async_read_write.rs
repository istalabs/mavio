//! Asynchronous [`AsyncRead`] / [`AsyncWrite`] interfaces.

use core::future::Future;

use crate::error::Error;

/// Generic asynchronous read trait similar to [`std::io::Read`].
///
/// The instances of [`AsyncRead`] are required to construct [`AsyncReceiver`] that can read
/// MavLink frames.
///
/// Instead of relying on a particular definition of read trait, we allow users to use any library
/// with I/O capabilities.
///
/// Wrappers are available for specific I/O implementations:
///
/// - [`TokioReader`] for [`tokio::io::AsyncRead`]
/// - [`EmbeddedIoAsyncReader`] for [`embedded_io_async::Read`]
///
/// [`AsyncReceiver`]: crate::io::AsyncReceiver
/// [`TokioReader`]: crate::io::TokioReader
/// [`EmbeddedIoAsyncReader`]: crate::io::EmbeddedIoAsyncReader
pub trait AsyncRead<Err: Into<Error>> {
    /// Reads asynchronously the exact number of bytes required to fill the buffer.
    ///
    /// Mimics the corresponding method from [`std::io::Read`].
    ///
    /// # Errors
    ///
    /// Returns generic error in case of I/O failure.
    fn read_exact<'a>(&'a mut self, buf: &'a mut [u8]) -> impl Future<Output = Result<(), Err>>;
}

/// Generic asynchronous write trait similar to [`std::io::Write`].
///
/// The instances of [`AsyncWrite`] are required to construct [`AsyncSender`] that can write
/// MavLink frames.
///
/// Instead of relying on a particular definition of write trait, we allow users to use any library
/// with I/O capabilities.
///
/// Wrappers are available for specific I/O implementations:
///
/// - [`TokioWriter`] for [`tokio::io::AsyncWrite`]
/// - [`EmbeddedIoAsyncWriter`] for [`embedded_io_async::Write`]
///
/// [`AsyncSender`]: crate::io::AsyncSender
/// [`TokioWriter`]: crate::io::TokioWriter
/// [`EmbeddedIoAsyncWriter`]: crate::io::EmbeddedIoAsyncWriter
pub trait AsyncWrite<Err: Into<Error>> {
    /// Attempts to write an entire buffer into this writer.
    ///
    /// Mimics the corresponding method from [`std::io::Write`].
    ///
    /// # Errors
    ///
    /// Returns generic error in case of I/O failure.
    fn write_all<'a>(&'a mut self, buf: &'a [u8]) -> impl Future<Output = Result<(), Err>>;

    /// Flush this output stream, ensuring that all intermediately buffered
    /// contents reach their destination.
    fn flush(&mut self) -> impl Future<Output = Result<(), Err>>;
}
