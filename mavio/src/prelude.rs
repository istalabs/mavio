//! Common imports.

pub use crate::errors::{Error, FrameError, Result, SpecError};
pub use crate::protocol::{
    Dialect, Frame, MavLinkVersion, MaybeVersioned, Message, Versioned, Versionless, V1, V2,
};
