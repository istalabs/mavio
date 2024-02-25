//! Common types.

use crate::consts::{
    HEADER_V1_SIZE, HEADER_V2_SIZE, SIGNATURE_LENGTH, SIGNATURE_TIMESTAMP_LENGTH,
    SIGNATURE_VALUE_LENGTH,
};

/// MAVLink system `ID`.
///
/// `ID` of system (vehicle) sending the message.
pub type SystemId = u8;

/// MAVLink component `ID`.
///
/// `ID` of component sending the message.
pub type ComponentId = u8;

/// Packet sequence number.
pub type Sequence = u8;

/// Payload length.
pub type PayloadLength = u8;

/// MAVLink packet checksum.
///
/// MAVLink checksum is encoded with little endian (low byte, high byte).
///
/// # Links
///
///  * [`Frame::checksum`](crate::Frame::checksum).
///  * [`Frame::calculate_crc`](crate::Frame::calculate_crc).
pub type Checksum = u16;

/// `MAVLink 1` header as array of bytes.
pub type HeaderV1Bytes = [u8; HEADER_V1_SIZE];
/// `MAVLink 2` header as array of bytes.
pub type HeaderV2Bytes = [u8; HEADER_V2_SIZE];

/// `MAVLink 2` signature as array of bytes.
///
/// # Links
///
///  * [`Signature`](crate::protocol::Signature).
///  * [MAVLink 2 message signing](https://mavlink.io/en/guide/message_signing.html).
pub type SignatureBytes = [u8; SIGNATURE_LENGTH];
/// `MAVLink 2` signature link ID.
///
/// # Links
///
///  * [`Signature`](crate::protocol::Signature).
///  * `link id` field in [MAVLink 2 message signing](https://mavlink.io/en/guide/message_signing.html).
pub type SignatureLinkId = u8;
/// `MAVLink 2` signature timestamp.
///
/// # Links
///
///  * [`Signature`](crate::protocol::Signature).
///  * `tm.timestamp` field in [MAVLink 2 message signing](https://mavlink.io/en/guide/message_signing.html).
pub type SignatureTimestampBytes = [u8; SIGNATURE_TIMESTAMP_LENGTH];
/// `MAVLink 2` signature value.
///
/// # Links
///
///  * [`Signature`](crate::protocol::Signature).
///  * `signature` field in [MAVLink 2 message signing](https://mavlink.io/en/guide/message_signing.html).
pub type SignatureValue = [u8; SIGNATURE_VALUE_LENGTH];

/// Return type for operations which may lead to data corruption.
///
/// The caller explicitly accepts the consequences, retrieving the result by calling
/// [`Unsafe::accept_unsafe`].
pub struct Unsafe<T>(T);
impl<T> Unsafe<T> {
    /// Creates an unsafe wrapper for a value.
    #[must_use]
    #[inline(always)]
    pub fn new(value: T) -> Self {
        Self(value)
    }

    /// Accept the danger and retrieve wrapped value.
    #[must_use]
    #[inline(always)]
    pub fn accept_unsafe(self) -> T {
        self.0
    }
}
