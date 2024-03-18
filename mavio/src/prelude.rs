//! # Common imports

pub use crate::error::{Error, FrameError, Result, SpecError};
pub use crate::protocol::{
    Dialect, Endpoint, Frame, MavFrame, MavLinkId, MavLinkVersion, MaybeVersioned, Message,
    Versioned, Versionless, V1, V2,
};
#[cfg(feature = "unsafe")]
pub use crate::utils::TryUpdateFrom;

#[cfg(feature = "async")]
pub use crate::io::{AsyncReceiver, AsyncSender};
pub use crate::io::{Receiver, Sender};
