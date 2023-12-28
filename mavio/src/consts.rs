//! # Constants
//!
//! We re-export from [`mavspec::rust::spec::consts`] to to provide a full specification of MAVLink-related types.

// Re-export from `mavspec::rust::spec::consts`
#[doc(no_inline)]
pub use mavspec::rust::spec::consts::MESSAGE_ID_V1_MAX;
#[doc(no_inline)]
pub use mavspec::rust::spec::consts::MESSAGE_ID_V2_MAX;
#[doc(no_inline)]
pub use mavspec::rust::spec::consts::PAYLOAD_MAX_SIZE;

/// `MAVLink 1` packet start marker value.
///
/// # Links
///
/// * [`MavSTX::V1`](crate::protocol::MavSTX::V1).
pub const STX_V1: u8 = 0xFE;
/// `MAVLink 2` packet start marker value.
///
/// # Links
///
/// * [`MavSTX::V2`](crate::protocol::MavSTX::V2).
pub const STX_V2: u8 = 0xFD;

/// Minimum size of a MAVLink header (regardless of protocol).
pub const HEADER_MIN_SIZE: usize = HEADER_V1_SIZE;
/// Maximum size of a MAVLink header (regardless of protocol).
pub const HEADER_MAX_SIZE: usize = HEADER_V2_SIZE;
/// Size of the `MAVLink 1` header in bytes.
///
/// `MAVLink 1` header have the following format:
///
/// | Field            | Size in bytes |
/// |------------------|---------------|
/// | `magic` byte     | 1             |
/// | `payload_length` | 1             |
/// | `sequence`       | 1             |
/// | `system_id`      | 1             |
/// | `component_id`   | 1             |
/// | `message_id`     | 1             |
///
/// # Links
///
/// * [MAVLink 1 packet format](https://mavlink.io/en/guide/serialization.html#mavlink2_packet_format).
pub const HEADER_V1_SIZE: usize = 6;
/// Size of the `MAVLink 2` header in bytes.
///
/// `MAVLink 2` header have the following format:
///
/// | Field            | Size in bytes |
/// |------------------|---------------|
/// | `magic` byte     | 1             |
/// | `incompat_flags` | 1             |
/// | `compat_flags`   | 1             |
/// | `payload_length` | 1             |
/// | `sequence`       | 1             |
/// | `system_id`      | 1             |
/// | `component_id`   | 1             |
/// | `message_id`     | 3             |
///
/// # Links
///
/// * [MAVLink 2 packet format](https://mavlink.io/en/guide/serialization.html#v1_packet_format).
pub const HEADER_V2_SIZE: usize = 10;

/// Size of MAVLink checksum in bytes.
pub const CHECKSUM_SIZE: usize = 2;

/// `MAVLink 2` "message is signed" incompatibility flag.
///
/// # Links
///
/// * `MAVLINK_IFLAG_SIGNED` field in [MAVLink 2 incompatibility flags](https://mavlink.io/en/guide/serialization.html#incompat_flags)
pub const MAVLINK_IFLAG_SIGNED: u8 = 0x01;

/// `MAVLink 2` signature link ID length in bytes.
///
/// # Links
///
/// * [`Signature`](crate::protocol::Signature)
/// * `link id` field in [MAVLink 2 message signing](https://mavlink.io/en/guide/message_signing.html)
pub const SIGNATURE_LINK_ID_LENGTH: usize = 1;
/// `MAVLink 2` signature timestamp length in bytes.
///
/// # Links
///
/// * [`Signature`](crate::protocol::Signature)
/// * `tm.timestamp` field in [MAVLink 2 message signing](https://mavlink.io/en/guide/message_signing.html)
/// * [Timestamp handling](https://mavlink.io/en/guide/message_signing.html#timestamp) in MAVLink documentation.
/// * [`SIGNATURE_TIMESTAMP_OFFSET`] for switching between Unix and MAVLink epochs.
pub const SIGNATURE_TIMESTAMP_LENGTH: usize = 6;
/// `MAVLink 2` signature value length in bytes.
///
/// # Links
///
///  * [`Signature`](crate::protocol::Signature)
///  * `signature` field in [MAVLink 2 message signing](https://mavlink.io/en/guide/message_signing.html)
pub const SIGNATURE_VALUE_LENGTH: usize = 6;

/// `MAVLink 2` signature length in bytes.
///
/// # Links
///
///  * [`Signature`](crate::protocol::Signature)
///  * [MAVLink 2 message signing](https://mavlink.io/en/guide/message_signing.html)
pub const SIGNATURE_LENGTH: usize =
    SIGNATURE_LINK_ID_LENGTH + SIGNATURE_TIMESTAMP_LENGTH + SIGNATURE_VALUE_LENGTH;

/// Timestamp offset in seconds from MAVLink to Unix epoch
///
/// Number of seconds between MAVLink epoch (1st January 2015 GMT) and Unix epoch (1st January 1970 GMT)
///
/// # Links
///
/// * [Timestamp handling](https://mavlink.io/en/guide/message_signing.html#timestamp) in MAVLink documentation.
pub const SIGNATURE_TIMESTAMP_OFFSET: u64 = 1420070400;

/// Length of a `MAVLink 2` secret key in bytes.
///
/// # Links
///
/// * [Signature](https://mavlink.io/en/guide/message_signing.html#signature) in MAVLink message signing documentation.
pub const SIGNATURE_SECRET_KEY_LENGTH: usize = 32;
