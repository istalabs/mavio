//! # `MAVLink 2` packet signature
//!
//! Implements [`Signature`].
//!
//! # Links
//!
//! * [MAVLink 2 message signing](https://mavlink.io/en/guide/message_signing.html).

use core::fmt::{Debug, Formatter};

use crate::consts::{
    SIGNATURE_LENGTH, SIGNATURE_LINK_ID_LENGTH, SIGNATURE_SECRET_KEY_LENGTH,
    SIGNATURE_TIMESTAMP_LENGTH, SIGNATURE_TIMESTAMP_OFFSET, SIGNATURE_VALUE_LENGTH,
};
use crate::errors::Result;
use crate::errors::{CoreError, FrameError};
use crate::protocol::{SignatureBytes, SignatureLinkId, SignatureTimestampBytes, SignatureValue};

/// `MAVLink 2` packet signature.
///
/// # Links
///
/// * [MAVLink 2 message signing](https://mavlink.io/en/guide/message_signing.html).
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Signature {
    pub(super) link_id: SignatureLinkId,
    pub(super) timestamp: MavTimestamp,
    pub(super) value: SignatureValue,
}

/// A [`MavTimestamp`] is a 48-bit timestamp used for `MAVLink 2` packet signing.
///
/// Contains `MAVLink 2` timestamp, a 48-bit value that equals to the number of milliseconds * 10 since the start of the
/// MAVLink epoch (1st January 2015 GMT).
///
/// # Links
///
/// * [Timestamp handling](https://mavlink.io/en/guide/message_signing.html#timestamp) in MAVLink documentation.
/// * [`Signature`] is a section of MAVLink packet where timestamp is stored.
#[derive(Clone, Copy, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MavTimestamp {
    raw: u64,
}

/// The [`Sign`] trait allows signing `MAVLink 2` frames.
///
/// Implements `sha256_48`, a `MAVLink 2` specific hashing algorithm similar to regular `sha256` except that only first
/// 48 bits are considered.
///
/// # Links
///
/// * Values calculated by implementors of [`Sign`] are stored in [`Signature`] struct as `value`.
/// * [Signature specification](https://mavlink.io/en/guide/message_signing.html#signature) format in MAVLink docs.
pub trait Sign {
    /// Reset inner state of a signer.
    ///
    /// Used by caller to ensure that signer's inner state does not have any digested data.
    fn reset(&mut self);
    /// Adds value to digest.
    ///
    /// Caller can invoke [`Sign::digest`] multiple times. Passing data as several sequential chunks is the same as
    /// calling `digest` with the whole data at once.
    fn digest(&mut self, bytes: &[u8]);
    /// Calculates `MAVLink 2` signature.
    fn signature(&self) -> SignatureValue;
}

/// A struct to set specify [`Signature`] configuration for [`Frame`](crate::protocol::Frame).
///
/// # Links
///
/// * [`Sign`] trait defines signing algorithm protocol.
/// * [Signature specification](https://mavlink.io/en/guide/message_signing.html#signature) in MAVLink docs.
#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SignatureConf {
    /// Defines [`Signature::link_id`] that will be appended to MAVLink packet upon signing.
    pub link_id: SignatureLinkId,
    /// Defines [`Signature::timestamp`] that will be appended to MAVLink packet upon signing.
    pub timestamp: MavTimestamp,
    /// Secret key is used to calculate [`Signature::value`] value.
    ///
    /// > **Note!** Since `secret` contains sensitive value it will be excluded from serialization. In addition,
    /// > [`SignatureConf::fmt`] used by [`Debug`] trait will mask `secret` value preventing it from being accidental
    /// printed to logs.
    #[cfg_attr(feature = "serde", serde(skip_serializing))]
    pub secret: SecretKey,
}

/// `MAVLink 2` signature secret key.
///
/// A 32-byte secret key for `MAVLink 2` message signing.
///
/// # Links
///
///  * [`Signature`] is a container for storing `MAVLink 2` signature.
///  * [`Sign`] is a trait which `sha256_48` algorithms should implement.
///  * `signature` field in [MAVLink 2 message signing](https://mavlink.io/en/guide/message_signing.html).
pub type SecretKey = [u8; SIGNATURE_SECRET_KEY_LENGTH];

impl TryFrom<&[u8]> for Signature {
    type Error = CoreError;

    /// Decodes slice of bytes into [`Signature`].
    ///
    /// See [`Signature::try_from_slice`].
    #[inline]
    fn try_from(value: &[u8]) -> Result<Self> {
        Self::try_from_slice(value)
    }
}

impl From<Signature> for SignatureBytes {
    /// Encodes [`Signature`] into [`SignatureBytes`] byte array.
    ///
    /// #Links
    ///
    /// * [`Signature::to_byte_array`].
    fn from(value: Signature) -> Self {
        value.to_byte_array()
    }
}

impl Debug for SignatureConf {
    /// Formats debug string for [`SignatureConf`] masking `secret` value.
    ///
    /// Replaces secret value with bytes with `[0xff; SIGNATURE_SECRET_KEY_LENGTH]` as recommended by
    /// [MAVLink documentation](https://mavlink.io/en/guide/message_signing.html#logging).
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "SignatureConf {{ link_id: {}, timestamp: {:?}, secret: [0xff; {}] }}",
            self.link_id, self.timestamp, SIGNATURE_SECRET_KEY_LENGTH
        )
    }
}

impl Default for SignatureConf {
    /// Instantiates [`SignatureConf`] with default values.
    ///
    /// Sets `secret` bytes to `0xff` which is recommended as a masked value by
    /// [MAVLink documentation](https://mavlink.io/en/guide/message_signing.html#logging).
    fn default() -> Self {
        Self {
            link_id: 0,
            timestamp: Default::default(),
            secret: [0xff; SIGNATURE_SECRET_KEY_LENGTH],
        }
    }
}

impl Signature {
    /// Signature `link_id` is a 8-bit identifier of a MAVLink channel.
    ///
    /// Peers may have different semantics or rules for different links. For example, some links may have higher
    /// priority over another during routing. Or even different secret keys for authorization.
    #[inline(always)]
    pub fn link_id(&self) -> SignatureLinkId {
        self.link_id
    }

    /// Signature [`MavTimestamp`] is a 48-bit value that specifies the moment when message was sent.
    ///
    /// The unit of measurement is the number of millisecond * 10 since MAVLink epoch (1st January 2015 GMT).
    ///
    /// According to MAVLink protocol, the sender must guarantee that the next timestamp is greater than the previous
    /// one.
    ///
    /// # Links
    ///
    /// * [`MavTimestamp`] struct.
    /// * [Timestamp handling](https://mavlink.io/en/guide/message_signing.html#timestamp) in MAVLink documentation.
    #[inline]
    pub fn timestamp(&self) -> MavTimestamp {
        self.timestamp
    }

    /// Signature `value` is cryptographic 48-bit hash that depends on the entire frame content.
    ///
    /// # Links
    ///
    /// * [Signature specification](https://mavlink.io/en/guide/message_signing.html#signature) in MAVLink docs.
    #[inline]
    pub fn value(&self) -> SignatureValue {
        self.value
    }

    /// Decodes slice of bytes into [`Signature`].
    ///
    /// Used in [`TryFrom<&[u8]>`](TryFrom) trait implementation for [`Signature`].
    ///
    /// # Errors
    ///
    /// Returns [`FrameError::SignatureIsTooSmall`] if slice is too small to
    /// contain a valid signature.
    pub fn try_from_slice(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < SIGNATURE_LENGTH {
            return Err(FrameError::SignatureIsTooSmall.into());
        }

        let link_id = bytes[0];
        let mut timestamp_bytes: SignatureTimestampBytes = Default::default();
        let mut signature: SignatureValue = Default::default();

        let timestamp_start = SIGNATURE_LINK_ID_LENGTH;
        let timestamp_end = timestamp_start + SIGNATURE_TIMESTAMP_LENGTH;
        timestamp_bytes.copy_from_slice(&bytes[timestamp_start..timestamp_end]);
        let timestamp: MavTimestamp = timestamp_bytes.into();

        let value_start = timestamp_end;
        let value_end = value_start + SIGNATURE_VALUE_LENGTH;
        signature.copy_from_slice(&bytes[timestamp_end..value_end]);

        Ok(Self {
            link_id,
            timestamp,
            value: signature,
        })
    }

    /// Encodes [`Signature`] into [`SignatureBytes`] byte array.
    ///
    /// Used in [`From<MavLinkV2Signature>`](From) trait implementation for [`SignatureBytes`].
    pub fn to_byte_array(&self) -> SignatureBytes {
        let mut bytes: SignatureBytes = Default::default();

        let timestamp_offset = SIGNATURE_LINK_ID_LENGTH;
        let signature_value_offset = SIGNATURE_LINK_ID_LENGTH + SIGNATURE_TIMESTAMP_LENGTH;

        bytes[0] = self.link_id;
        bytes[timestamp_offset..signature_value_offset]
            .copy_from_slice(&self.timestamp.to_bytes_array());
        bytes[signature_value_offset..SIGNATURE_LENGTH].copy_from_slice(&self.value);

        bytes
    }
}

impl From<u64> for MavTimestamp {
    /// Creates [`MavTimestamp`] from [`u64`] raw value discarding two higher bytes.
    ///
    /// Uses [Self::from_raw_u64] internally.
    fn from(value: u64) -> Self {
        Self::from_raw_u64(value)
    }
}

impl From<SignatureTimestampBytes> for MavTimestamp {
    /// Decodes [`MavTimestamp`] from bytes.
    ///
    /// Uses [`MavTimestamp::from_bytes`].
    #[inline(always)]
    fn from(bytes: SignatureTimestampBytes) -> Self {
        Self::from_bytes(&bytes)
    }
}

impl From<MavTimestamp> for SignatureTimestampBytes {
    /// Encodes [`MavTimestamp`] into bytes.
    ///
    /// Uses [`MavTimestamp::to_bytes_array`].
    #[inline(always)]
    fn from(timestamp: MavTimestamp) -> Self {
        timestamp.to_bytes_array()
    }
}

impl MavTimestamp {
    /// Creates [`MavTimestamp`] from [`u64`] raw value discarding two higher bytes.
    ///
    /// Provided `value` should represent [`Self::raw`] `MAVLink 2` signature timestamp.
    pub fn from_raw_u64(value: u64) -> Self {
        // Discard two higher bytes.
        let raw = value & 0xffffffffffff;
        Self { raw }
    }

    /// Decodes timestamp from bytes.
    ///
    /// # Links
    ///
    /// * [`MavTimestamp`]
    /// * [`SignatureTimestampBytes`]
    /// * [`Signature`]
    pub fn from_bytes(bytes: &SignatureTimestampBytes) -> Self {
        let mut bytes_u64 = [0u8; 8];
        bytes_u64[0..SIGNATURE_TIMESTAMP_LENGTH].copy_from_slice(bytes);
        Self {
            raw: u64::from_le_bytes(bytes_u64),
        }
    }

    /// Raw MAVLink timestamp value.
    ///
    /// Returns number of milliseconds * 10 since the start of MAVLink epoch (1st January 2015 GMT).
    ///
    /// Use [`Self::raw`] to set this value.
    ///
    /// # Links
    ///
    /// * [Timestamp handling](https://mavlink.io/en/guide/message_signing.html#timestamp) in MAVLink documentation.
    #[inline(always)]
    pub fn raw(&self) -> u64 {
        self.raw
    }

    /// Sets raw MAVLink timestamp value.
    ///
    /// Use [`Self::raw`] to get this value.
    ///
    /// # Links
    ///
    /// * [Timestamp handling](https://mavlink.io/en/guide/message_signing.html#timestamp) in MAVLink documentation.
    #[inline(always)]
    pub fn set_raw(&mut self, raw: u64) -> &mut Self {
        self.raw = raw;
        self
    }

    /// MAVLink timestamp in milliseconds.
    ///
    /// Returns value as number of milliseconds since the start of MAVLink epoch (1st January 2015 GMT).
    ///
    /// Use [`Self::millis_mavlink`] to set this value.
    ///
    /// # Links
    ///
    /// * [Timestamp handling](https://mavlink.io/en/guide/message_signing.html#timestamp) in MAVLink documentation.
    #[inline(always)]
    pub fn millis_mavlink(&self) -> u64 {
        self.raw * 10
    }

    /// Sets MAVLink timestamp in milliseconds.
    ///
    /// Use [`Self::millis_mavlink`] to get this value.
    ///
    /// # Links
    ///
    /// * [Timestamp handling](https://mavlink.io/en/guide/message_signing.html#timestamp) in MAVLink documentation.
    #[inline]
    pub fn set_millis_mavlink(&mut self, millis_mavlink: u64) -> &mut Self {
        self.raw = millis_mavlink / 10;
        self
    }

    /// Unix timestamp in milliseconds.
    ///
    /// Returns value as number of milliseconds since the start of Unix epoch (1st January 1970 GMT).
    ///
    /// # Links
    ///
    /// * [Timestamp handling](https://mavlink.io/en/guide/message_signing.html#timestamp) in MAVLink documentation.
    /// * [`SIGNATURE_TIMESTAMP_OFFSET`] defines offset between epochs.
    #[inline]
    pub fn millis(&self) -> u64 {
        self.raw * 10 + SIGNATURE_TIMESTAMP_OFFSET * 10u64.pow(6)
    }

    /// Sets Unix timestamp in milliseconds.
    ///
    /// Use [`Self::millis`] to get this value.
    ///
    /// # Links
    ///
    /// * [Timestamp handling](https://mavlink.io/en/guide/message_signing.html#timestamp) in MAVLink documentation.
    #[inline]
    pub fn set_millis(&mut self, millis: u64) -> &mut Self {
        self.set_millis_mavlink(millis - SIGNATURE_TIMESTAMP_OFFSET * 10u64.pow(6));
        self
    }

    /// Encodes timestamp into bytes.
    ///
    /// # Links
    ///
    /// * [`MavTimestamp`]
    /// * [`SignatureTimestampBytes`]
    /// * [`Signature`]
    pub fn to_bytes_array(&self) -> SignatureTimestampBytes {
        let bytes_u64: [u8; 8] = self.raw.to_le_bytes();
        let mut bytes = [0u8; SIGNATURE_TIMESTAMP_LENGTH];
        bytes.copy_from_slice(&bytes_u64[0..SIGNATURE_TIMESTAMP_LENGTH]);
        bytes
    }
}
