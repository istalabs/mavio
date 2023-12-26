//! # MAVLink packet start marker
//!
//! [`MavSTX`] represents a protocol-specific start-of-text (STX) marker used to indicate the
//! beginning of a new packet.
//!
//! Any system that does not understand protocol version will skip the packet.
//!
//! See:
//! * [MAVLink 1 Packet Format](https://mavlink.io/en/guide/serialization.html#v1_packet_format).
//! * [MAVLink 2 Packet Format](https://mavlink.io/en/guide/serialization.html#mavlink2_packet_format).

use core::convert::TryFrom;

use crate::consts::{STX_V1, STX_V2};
use crate::errors::{CoreError, FrameError, Result};
use crate::protocol::MavLinkVersion;

/// Packet start marker.
///
/// Protocol-specific start-of-text (STX) marker used to indicate the beginning of a new packet.
///
/// Any system that does not understand protocol version will skip the packet.
///
/// See:
/// * [MAVLink 1 Packet Format](https://mavlink.io/en/guide/serialization.html#v1_packet_format).
/// * [MAVLink 2 Packet Format](https://mavlink.io/en/guide/serialization.html#mavlink2_packet_format).
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MavSTX {
    /// Designates `MAVLink 1` protocol, equals to [`STX_V1`].
    V1,
    /// Designates `MAVLink 2` protocol, equals to [`STX_V2`].
    V2,
    /// Unknown protocol.
    Unknown(u8),
}

impl Default for MavSTX {
    /// Creates [`MavSTX`] with default value.
    ///
    /// We assume unknown protocol with zero marker.
    fn default() -> Self {
        Self::Unknown(0)
    }
}

impl From<MavSTX> for u8 {
    /// Converts from `u8` into [`MavSTX`].
    fn from(value: MavSTX) -> Self {
        match value {
            MavSTX::V1 => STX_V1,
            MavSTX::V2 => STX_V2,
            MavSTX::Unknown(unknown) => unknown,
        }
    }
}

impl From<u8> for MavSTX {
    /// Converts from `u8` into [`MavSTX`].
    fn from(value: u8) -> Self {
        match value {
            STX_V1 => MavSTX::V1,
            STX_V2 => MavSTX::V2,
            unknown => MavSTX::Unknown(unknown),
        }
    }
}

impl From<MavLinkVersion> for MavSTX {
    /// Creates [`MavSTX`] from [`MavLinkVersion`].
    fn from(value: MavLinkVersion) -> Self {
        match value {
            MavLinkVersion::V1 => MavSTX::V1,
            MavLinkVersion::V2 => MavSTX::V2,
        }
    }
}

impl TryFrom<MavSTX> for MavLinkVersion {
    type Error = CoreError;

    /// Tries to convert [`MavSTX`] into [`MavLinkVersion`].
    ///
    /// # Errors
    ///
    /// Returns [`FrameError::InvalidMavLinkVersion`] if [`MavSTX::Unknown`] provided.
    fn try_from(value: MavSTX) -> Result<Self> {
        Ok(match value {
            MavSTX::V1 => Self::V1,
            MavSTX::V2 => Self::V2,
            MavSTX::Unknown(_) => return Err(FrameError::InvalidMavLinkVersion.into()),
        })
    }
}

impl MavSTX {
    /// Checks that `value` represents MAVLink magic (start-of-text) byte.
    pub fn is_magic_byte(value: u8) -> bool {
        value == STX_V1 || value == STX_V2
    }
}
