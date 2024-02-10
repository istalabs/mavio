//! # MAVLink protocol
//!
//! MAVLink protocol abstractions.
//!
//! We also re-export from [`mavspec::rust::spec`] crate to provide a full specification of MAVLink-related types.

// Re-export from `mavspec::rust::spec`
#[doc(no_inline)]
pub use mavspec::rust::spec::types::{CrcExtra, MessageId};
#[doc(no_inline)]
pub use mavspec::rust::spec::{
    DialectImpl, DialectMessage, DialectSpec, IntoPayload, MavLinkVersion, MessageImpl,
    MessageSpec, Payload,
};

// Signature
pub(super) mod signature;
pub use signature::{MavTimestamp, SecretKey, Sign, Signature, SignatureConf};

// Magic byte (STX)
pub(crate) mod stx;
pub use stx::MavSTX;

// Header
pub(super) mod header;
pub use header::{Header, HeaderBytes};

// MAVLink frame
pub(super) mod frame;
pub use frame::{Frame, FrameBuilder};

// Common types
pub(super) mod types;
pub use types::{
    Checksum, CompatFlags, ComponentId, HeaderV1Bytes, HeaderV2Bytes, IncompatFlags, PayloadLength,
    Sequence, SignatureBytes, SignatureLinkId, SignatureTimestampBytes, SignatureValue, SystemId,
};
