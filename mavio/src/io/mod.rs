//! # Basic MAVLink I/O
//!
//! This module includes basic MAVLink I/O utils for reading and writing frames
//! ([`MavLinkFrame`](crate::Frame)).
//!
//! # Targets
//!
//! For `std` environments [`mavio`](crate) uses
//! [`std::io::Read`](https://doc.rust-lang.org/std/io/trait.Read.html)
//! and [`std::io::Write`](https://doc.rust-lang.org/std/io/trait.Write.html) reader and writer.
//!
//! For `no_std` [`mavio`](crate) uses custom `Read` and `Write` traits:
//!
//! ```rust
//! use mavio::error::Result;
//!
//! trait Read {
//!     fn read(&mut self, buf: &mut [u8]) -> Result<usize>;
//!     fn read_exact(&mut self, buf: &mut [u8]) -> Result<()>;
//! }
//!
//! trait Write {
//!     fn write(&mut self, buf: &[u8]) -> Result<usize>;
//!     fn write_all(&mut self, buf: &[u8]) -> Result<()>;
//! }
//! ```
//!
//! In addition, the following `IoError` error is defined for `no_std`:
//!
//! ```rust
//! #[derive(Clone, Debug)]
//! pub enum IoError {
//!     /// Operation was interrupted.
//!     ///
//!     /// In most cases this means that operation can be retried.
//!     Interrupted,
//!     /// Invalid data received.
//!     InvalidData,
//!     /// This operation is unsupported.
//!     Unsupported,
//!     /// Unexpected end-of-file.
//!     ///
//!     /// In most cases this means that smaller amount of bytes are available.
//!     UnexpectedEof,
//!     /// Other error.
//!     Other(String),
//! }
//! ```
//!
//! This error will be wrapped with `no_std` version of [`Error`](crate::error::Error).

mod read_write;
pub use read_write::{Read, Write};
mod async_read_write;
pub use async_read_write::{AsyncRead, AsyncWrite};

pub mod adapters;
#[cfg(feature = "embedded-io-async")]
#[doc(inline)]
pub use adapters::{EmbeddedIoAsyncReader, EmbeddedIoAsyncWriter};
#[cfg(feature = "embedded-io")]
#[doc(inline)]
pub use adapters::{EmbeddedIoReader, EmbeddedIoWriter};
#[cfg(feature = "futures")]
#[doc(inline)]
pub use adapters::{FuturesReader, FuturesWriter};
#[cfg(feature = "std")]
#[doc(inline)]
pub use adapters::{StdIoReader, StdIoWriter};
#[cfg(feature = "tokio")]
#[doc(inline)]
pub use adapters::{TokioReader, TokioWriter};

mod receiver;
pub use receiver::Receiver;

mod sender;
pub use sender::Sender;

mod async_receiver;
pub use async_receiver::AsyncReceiver;

mod async_sender;
pub use async_sender::AsyncSender;
