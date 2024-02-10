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
//! use mavio::errors::Result;
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
//! This error will be wrapped with `no_std` version of [`Error`](crate::errors::Error).

#[cfg(not(feature = "std"))]
pub(crate) mod no_std;
#[cfg(not(feature = "std"))]
pub use no_std::{Read, Write};
#[cfg(feature = "std")]
#[doc(hidden)]
pub use std::io::{Read, Write};

pub(crate) mod receiver;
#[cfg(feature = "unstable")]
pub use receiver::FrameIterator;
pub use receiver::Receiver;

pub(crate) mod sender;
pub use sender::Sender;

#[cfg(feature = "tokio")]
mod async_receiver;
#[cfg(feature = "tokio")]
pub use async_receiver::AsyncReceiver;

#[cfg(feature = "tokio")]
mod async_sender;
#[cfg(feature = "tokio")]
pub use async_sender::AsyncSender;
