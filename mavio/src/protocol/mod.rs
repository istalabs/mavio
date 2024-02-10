//! # MAVLink protocol
//!
//! MAVLink protocol abstractions.
//!
//! We also re-export from [`mavspec::rust::spec`] crate to provide a full specification of
//! MAVLink-related types.

#[doc(no_inline)]
pub use mavspec::rust::spec::types::{CrcExtra, MessageId};
#[doc(no_inline)]
pub use mavspec::rust::spec::{
    DialectImpl, DialectMessage, DialectSpec, IntoPayload, MavLinkVersion, MessageImpl,
    MessageSpec, Payload,
};

pub(super) mod frame;
pub(super) mod header;
pub(super) mod signature;
pub(crate) mod stx;
pub(super) mod types;

pub use frame::{Frame, FrameBuilder};
pub use header::{Header, HeaderBytes};
pub use signature::{MavTimestamp, SecretKey, Sign, Signature, SignatureConf};
pub use stx::MavSTX;
pub use types::{
    Checksum, CompatFlags, ComponentId, HeaderV1Bytes, HeaderV2Bytes, IncompatFlags, PayloadLength,
    Sequence, SignatureBytes, SignatureLinkId, SignatureTimestampBytes, SignatureValue, SystemId,
};
