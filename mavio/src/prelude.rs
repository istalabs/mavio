//! # Common imports

pub use crate::error::{Error, FrameError, Result, SpecError};
pub use crate::protocol::{
    Dialect, Endpoint, Frame, MavFrame, MavLinkId, MavLinkVersion, MaybeVersioned, Message,
    Versioned, Versionless, V1, V2,
};

pub use crate::io::{AsyncReceiver, AsyncSender};
pub use crate::io::{Receiver, Sender};

#[cfg(feature = "std")]
pub use crate::io::{StdIoReader, StdIoWriter};

#[cfg(feature = "tokio")]
pub use crate::io::{TokioReader, TokioWriter};

#[cfg(feature = "futures")]
pub use crate::io::{FuturesReader, FuturesWriter};

#[cfg(feature = "embedded-io")]
pub use crate::io::{EmbeddedIoReader, EmbeddedIoWriter};

#[cfg(feature = "embedded-io-async")]
pub use crate::io::{EmbeddedIoAsyncReader, EmbeddedIoAsyncWriter};

#[cfg(feature = "unsafe")]
pub use crate::utils::TryUpdateFrom;
