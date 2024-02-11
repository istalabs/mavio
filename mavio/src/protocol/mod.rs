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

pub(super) mod flags;
pub(super) mod frame;
pub(super) mod frame_builder;
pub(super) mod header;
pub(super) mod header_builder;
pub(super) mod marker;
pub(super) mod signature;
pub(crate) mod stx;
pub(super) mod types;

pub use flags::{CompatFlags, IncompatFlags};
pub use frame::Frame;
pub use frame_builder::FrameBuilder;
pub use header::{Header, HeaderBytes};
pub use header_builder::HeaderBuilder;
pub use marker::{MaybeVersioned, Versioned, Versionless, V1, V2};
pub use signature::{MavTimestamp, SecretKey, Sign, Signature, SignatureConf};
pub use stx::MavSTX;
pub use types::{
    Checksum, ComponentId, HeaderV1Bytes, HeaderV2Bytes, PayloadLength, Sequence, SignatureBytes,
    SignatureLinkId, SignatureTimestampBytes, SignatureValue, SystemId,
};
