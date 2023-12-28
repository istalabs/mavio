//! # MAVLink protocol
//!
//! MAVLink protocol abstractions.
//!
//! We also re-export from [`mavspec::rust::spec`] crate to provide a full specification of MAVLink-related types.

use crate::consts::{
    HEADER_V1_SIZE, HEADER_V2_SIZE, SIGNATURE_LENGTH, SIGNATURE_TIMESTAMP_LENGTH,
    SIGNATURE_VALUE_LENGTH,
};

// Re-export from `mavspec::rust::spec`
#[doc(no_inline)]
pub use mavspec::rust::spec::types::{CrcExtra, MessageId};
#[doc(no_inline)]
pub use mavspec::rust::spec::{
    DialectSpec, IntoPayload, MavLinkVersion, MessageImpl, MessageSpec, Payload,
};

// Signature
pub(crate) mod signature;
pub use signature::{MavTimestamp, SecretKey, Sign, Signature, SignatureConf};
// Magic byte (STX)
pub(crate) mod stx;
pub use stx::MavSTX;
// Header
pub(crate) mod header;
pub use header::{Header, HeaderBuilder, HeaderBytes};
// MAVLink frame
pub(crate) mod frame;
pub use frame::{Frame, FrameBuilder};

/// MAVLink packet checksum.
///
/// MAVLink checksum is encoded with little endian (low byte, high byte).
///
/// # Links
///
///  * [`Frame::checksum`].
///  * [`Frame::calculate_crc`].
pub type Checksum = u16;

/// `MAVLink 1` header as array of bytes.
pub type HeaderV1Bytes = [u8; HEADER_V1_SIZE];
/// `MAVLink 2` header as array of bytes.
pub type HeaderV2Bytes = [u8; HEADER_V2_SIZE];

/// `MAVLink 2` incompatibility flags.
pub type IncompatFlags = u8;
/// `MAVLink 2` compatibility flags.
pub type CompatFlags = u8;

/// `MAVLink 2` signature as array of bytes.
///
/// # Links
///
///  * [`Signature`].
///  * [MAVLink 2 message signing](https://mavlink.io/en/guide/message_signing.html).
pub type SignatureBytes = [u8; SIGNATURE_LENGTH];
/// `MAVLink 2` signature link ID.
///
/// # Links
///
///  * [`Signature`].
///  * `link id` field in [MAVLink 2 message signing](https://mavlink.io/en/guide/message_signing.html).
pub type SignatureLinkId = u8;
/// `MAVLink 2` signature timestamp.
///
/// # Links
///
///  * [`Signature`].
///  * `tm.timestamp` field in [MAVLink 2 message signing](https://mavlink.io/en/guide/message_signing.html).
pub type SignatureTimestampBytes = [u8; SIGNATURE_TIMESTAMP_LENGTH];
/// `MAVLink 2` signature value.
///
/// # Links
///
///  * [`Signature`].
///  * `signature` field in [MAVLink 2 message signing](https://mavlink.io/en/guide/message_signing.html).
pub type SignatureValue = [u8; SIGNATURE_VALUE_LENGTH];
