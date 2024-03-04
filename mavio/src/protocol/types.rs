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
/// `MAVLink 2` signed link `ID`.
///
/// Link `ID` is an identifier of a communication channel in
/// [MAVLink 2 message signing](https://mavlink.io/en/guide/message_signing.html) protocol.
///
/// # Links
///
///  * [`Signature`](crate::protocol::Signature).
pub type SignedLinkId = u8;
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

/// Return type for operations which require attention from the caller.
///
/// Such operations may lead to data corruption or return data that may be misleading in some
/// circumstances. In other scenarios, [`Behold`] marks seemingly "innocent" methods or functions,
/// that perform costly operations, clone large amounts of data, or spawn treads. By any means, this
/// is just a reminder for the caller that certain aspects of the operation require their close
/// attention.
///
/// In some sense, [`Behold`] serves as an opinionated replacement for a relatively widespread
/// practice of marking with `unsafe` methods and functions that, while being safe from the Rust
/// perspective, still require certain care from the caller. We've decided to refrain from such
/// practice since: (a) some projects may restrict using unsafe Rust, (b) our use cases do not
/// strictly coincide with operations, that may lead to undecided behavior (even in wider sense).
///
/// Once [`Behold`] is obtained, the caller can either explicitly accept the consequences,
/// retrieving the result by calling [`Behold::unwrap`], or discard the value with
/// [`Behold::discard`].
#[must_use]
pub struct Behold<T>(T);
impl<T> Behold<T> {
    /// Creates an unsafe wrapper for a value.
    #[inline(always)]
    pub fn new(value: T) -> Self {
        Self(value)
    }

    /// Accept the consequences and retrieve the wrapped value.
    ///
    /// The accepted values are `#[must_use]`.
    #[inline(always)]
    #[must_use]
    pub fn unwrap(self) -> T {
        self.0
    }

    /// Discards the wrapped value.
    #[inline(always)]
    pub fn discard(self) {}
}
