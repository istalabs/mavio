//! # MAVLink protocol
//!
//! MAVLink protocol abstractions.
//!
//! We also re-export from [`mavspec::rust::spec`] crate to provide a full specification of
//! MAVLink-related types.

/// <sup>[`mavspec`](https://crates.io/crates/mavspec)</sup>
#[doc(inline)]
pub use mavspec::rust::spec::types::{CrcExtra, MessageId};
/// <sup>[`mavspec`](https://crates.io/crates/mavspec)</sup>
#[doc(inline)]
pub use mavspec::rust::spec::{
    Dialect, IntoPayload, MavLinkVersion, Message, MessageSpec, Payload,
};

mod endpoint;
pub(super) mod flags;
pub(super) mod frame;
pub(super) mod frame_builder;
pub(super) mod header;
pub(super) mod header_builder;
pub(super) mod marker;
mod sequencer;
pub(super) mod signature;
pub(crate) mod stx;
pub(super) mod types;

pub use endpoint::{Endpoint, MavLinkId};
pub use flags::{CompatFlags, IncompatFlags};
pub use frame::Frame;
pub use frame_builder::FrameBuilder;
pub use header::{Header, HeaderBytes};
pub use header_builder::HeaderBuilder;
pub use marker::{MaybeVersioned, Versioned, Versionless, V1, V2};
pub use sequencer::{IntoSequencer, Sequencer};
pub use signature::{MavTimestamp, SecretKey, Sign, Signature, SignatureConf};
pub use stx::MavSTX;
pub use types::{
    Behold, Checksum, ComponentId, HeaderV1Bytes, HeaderV2Bytes, PayloadLength, Sequence,
    SignatureBytes, SignatureTimestampBytes, SignatureValue, SignedLinkId, SystemId,
};
