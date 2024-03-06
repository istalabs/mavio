//! # Common imports

pub use crate::errors::{Error, FrameError, Result, SpecError};
pub use crate::protocol::{
    Dialect, Endpoint, Frame, MavLinkId, MavLinkVersion, MaybeVersioned, Message, Versioned,
    Versionless, V1, V2,
};

#[cfg(feature = "async")]
pub use crate::io::{AsyncReceiver, AsyncSender};
pub use crate::io::{Receiver, Sender};
