//! # MAVLink protocol
//!
//! MAVLink protocol abstractions.
//!
//! We also re-export from [`mavspec::rust::spec`] crate to provide a full specification of
//! MAVLink-related types.
//!
//! ## Frames
//!
//! The key MAVLink entity is a [`Frame`], that represents a packet containing message body and
//! additional metadata as specified by MAVLink [serialization](https://mavlink.io/en/guide/serialization.html)
//! protocol. Each frame has a [`header`](Frame::header) and [`payload`](Frame::payload). Frames can
//! be decoded into messages using [`Frame::decode`]. To build a frame you need a [`FrameBuilder`]
//! that provides an interface for constructing valid frames both manually and from existing
//! messages.
//!
//! ## Message Signing
//!
//! `MAVLink 2` protocol is capable of signing frames allowing the receiver to authenticate frame's
//! sender. To sign a frame you can either use [`Frame<V2>::add_signature`] or
//! [`SigningConf::apply`]. The latter will sign only `MAVLink 2` frames keeping `MAVLink 1`
//! frames untouched.
//!
//! The signing algorithm is split into [`Sign`] and [`Signer`]. The former is a trait that provides
//! `sha256_48`, a `MAVLink 2` specific hashing algorithm. The latter takes an implementor of
//! [`Sign`] and feeds frame data into it. This library provides and implementor of [`Sign`], the
//! [`MavSha256`](crate::utils::MavSha256), for `std` targets. All `no_std` targets should implement
//! their own algorithm based on their platform-specific access to random value generators.

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
pub use marker::{MaybeVersioned, Unset, Versioned, Versionless, V1, V2};
pub use sequencer::{IntoSequencer, Sequencer};
pub use signature::{MavTimestamp, SecretKey, Sign, Signature, Signer, SigningConf};
pub use stx::MavSTX;
pub use types::{
    Behold, Checksum, ComponentId, HeaderV1Bytes, HeaderV2Bytes, PayloadLength, Sequence,
    SignatureBytes, SignatureTimestampBytes, SignatureValue, SignedLinkId, SystemId,
};
