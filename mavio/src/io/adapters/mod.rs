//! I/O adapters.

#[cfg(feature = "embedded-io")]
mod embedded_io;
#[cfg(feature = "embedded-io-async")]
mod embedded_io_async;
#[cfg(feature = "futures")]
mod futures;
#[cfg(feature = "std")]
mod std_io;
#[cfg(feature = "tokio")]
mod tokio;

#[cfg(feature = "embedded-io")]
pub use embedded_io::{EmbeddedIoReader, EmbeddedIoWriter};
#[cfg(feature = "embedded-io-async")]
pub use embedded_io_async::{EmbeddedIoAsyncReader, EmbeddedIoAsyncWriter};
#[cfg(feature = "futures")]
pub use futures::{FuturesReader, FuturesWriter};
#[cfg(feature = "std")]
pub use std_io::{StdIoReader, StdIoWriter};
#[cfg(feature = "tokio")]
pub use tokio::{TokioReader, TokioWriter};
