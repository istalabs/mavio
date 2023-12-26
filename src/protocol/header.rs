//! # MAVLink header
//!
//! This module contains implementation for MAVLink packet header both for `MAVLink 1` and
//! `MAVLink 2`.

use mavspec::rust::spec::consts::{MESSAGE_ID_V1_MAX, MESSAGE_ID_V2_MAX};
use tbytes::{TBytesReader, TBytesReaderFor};

use crate::consts::{
    CHECKSUM_SIZE, HEADER_MAX_SIZE, HEADER_MIN_SIZE, HEADER_V1_SIZE, HEADER_V2_SIZE,
    MAVLINK_IFLAG_SIGNED, SIGNATURE_LENGTH,
};
use crate::errors::{FrameError, Result};
use crate::io::{Read, Write};
use crate::protocol::{CompatFlags, IncompatFlags, MavSTX};
use crate::protocol::{MavLinkVersion, MessageId};

/// MAVLink frame header.
///
/// Header contains information relevant to for `MAVLink 1` and `MAVLink 2` packet formats.
///
/// See:
///  * [MAVLink 1 packet format](https://mavlink.io/en/guide/serialization.html#v1_packet_format).
///  * [MAVLink 2 packet format](https://mavlink.io/en/guide/serialization.html#mavlink2_packet_format).
#[derive(Clone, Copy, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Header {
    mavlink_version: MavLinkVersion,
    payload_length: u8,
    incompat_flags: IncompatFlags,
    compat_flags: CompatFlags,
    sequence: u8,
    system_id: u8,
    component_id: u8,
    message_id: MessageId,
}

/// Represents [`Header`] encoded as a sequence of bytes.
#[derive(Clone, Copy, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HeaderBytes {
    buffer: [u8; HEADER_MAX_SIZE],
    size: usize,
}

/// Configuration builder for [`Header`].
///
/// Implements [builder](https://rust-unofficial.github.io/patterns/patterns/creational/builder.html)
/// pattern for [`Header`].
#[derive(Clone, Copy, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HeaderBuilder {
    payload_length: Option<u8>,
    incompat_flags: Option<u8>,
    compat_flags: Option<u8>,
    sequence: Option<u8>,
    system_id: Option<u8>,
    component_id: Option<u8>,
    message_id: Option<MessageId>,
}

impl HeaderBytes {
    /// Encoded [`Header`] as a slice of bytes.
    ///
    /// The length of a slice matches [`Self::size`] and therefore [`Header::size`].
    pub fn as_slice(&self) -> &[u8] {
        &self.buffer[0..self.size]
    }

    /// Size of the encoded [`Header`] in bytes.
    pub fn size(&self) -> usize {
        self.size
    }

    /// Encoded [`Header`] CRC data.
    ///
    /// Returns all header data excluding `magic` byte.
    ///
    /// See:
    ///  * [`MavLinkFrame::calculate_crc`](crate::protocol::Frame::calculate_crc).
    ///  * [MAVLink checksum](https://mavlink.io/en/guide/serialization.html#checksum) in MAVLink
    ///    protocol documentation.
    pub fn crc_data(&self) -> &[u8] {
        &self.buffer[1..self.size()]
    }
}

impl Header {
    /// Initiates builder for [`Header`].
    ///
    /// Instead of constructor we use
    /// [builder](https://rust-unofficial.github.io/patterns/patterns/creational/builder.html)
    /// pattern. An instance of [`HeaderBuilder`] returned by this function is initialized
    /// with default values. Once desired values are set, you can call [`HeaderBuilder::build`]
    /// to obtain [`Header`].
    pub fn builder() -> HeaderBuilder {
        HeaderBuilder::new()
    }

    /// MAVLink protocol version.
    ///
    /// MAVLink version defined by the magic byte (STX).
    ///
    /// See [`MavSTX`].
    #[inline]
    pub fn mavlink_version(&self) -> MavLinkVersion {
        self.mavlink_version
    }

    /// Payload length.
    ///
    /// Indicates length of the following `payload` section. This may be affected by payload truncation.
    #[inline]
    pub fn payload_length(&self) -> u8 {
        self.payload_length
    }

    /// Incompatibility flags for `MAVLink 2` header.
    ///
    /// Flags that must be understood for MAVLink compatibility (implementation discards packet if
    /// it does not understand flag).
    ///
    /// See: [MAVLink 2 incompatibility flags](https://mavlink.io/en/guide/serialization.html#incompat_flags).
    #[inline]
    pub fn incompat_flags(&self) -> Option<u8> {
        match self.mavlink_version() {
            MavLinkVersion::V1 => None,
            MavLinkVersion::V2 => Some(self.incompat_flags),
        }
    }

    /// Compatibility flags for `MAVLink 2` header.
    ///
    /// Flags that can be ignored if not understood (implementation can still handle packet even if
    /// it does not understand flag).
    ///
    /// See: [MAVLink 2 compatibility flags](https://mavlink.io/en/guide/serialization.html#compat_flags).
    #[inline]
    pub fn compat_flags(&self) -> Option<u8> {
        match self.mavlink_version() {
            MavLinkVersion::V1 => None,
            MavLinkVersion::V2 => Some(self.compat_flags),
        }
    }

    /// Packet sequence number.
    ///
    /// Used to detect packet loss. Components increment value for each message sent.
    #[inline]
    pub fn sequence(&self) -> u8 {
        self.sequence
    }

    /// System `ID`.
    ///
    /// `ID` of system (vehicle) sending the message. Used to differentiate systems on network.
    ///
    /// > Note that the broadcast address 0 may not be used in this field as it is an invalid source
    /// > address.
    #[inline]
    pub fn system_id(&self) -> u8 {
        self.system_id
    }

    /// Component `ID`.
    ///
    /// `ID` of component sending the message. Used to differentiate components in a system (e.g.
    /// autopilot and a camera). Use appropriate values in
    /// [MAV_COMPONENT](https://mavlink.io/en/messages/common.html#MAV_COMPONENT).
    ///
    /// > Note that the broadcast address `MAV_COMP_ID_ALL` may not be used in this field as it is
    /// > an invalid source address.
    #[inline]
    pub fn component_id(&self) -> u8 {
        self.component_id
    }

    /// Message `ID`.
    ///
    /// `ID` of MAVLink message. Defines how payload will be encoded and decoded.
    #[inline]
    pub fn message_id(&self) -> MessageId {
        self.message_id
    }

    /// Size of the header in bytes.
    ///
    /// Depends on the MAVLink protocol version.
    pub fn size(&self) -> usize {
        match self.mavlink_version {
            MavLinkVersion::V1 => HEADER_V1_SIZE,
            MavLinkVersion::V2 => HEADER_V2_SIZE,
        }
    }

    /// Returns `true` if frame body should contain signature.
    ///
    /// For `MAVLink 1` headers always returns `false`.
    ///
    /// For `MAVLink 2` it checks for [`MAVLINK_IFLAG_SIGNED`] (default is `false`).
    /// returned.
    ///
    /// # Links
    ///
    /// * [Frame::signature](crate::protocol::Frame::signature).
    pub fn is_signed(&self) -> bool {
        match self.mavlink_version {
            MavLinkVersion::V1 => false,
            MavLinkVersion::V2 => {
                self.incompat_flags & MAVLINK_IFLAG_SIGNED == MAVLINK_IFLAG_SIGNED
            }
        }
    }

    /// Sets whether `MAVLink 2` frame body should contain signature.
    ///
    /// Sets `MAVLINK_IFLAG_SIGNED` for [`Self::incompat_flags`].
    #[inline]
    pub(super) fn set_is_signed(&mut self, flag: bool) {
        self.incompat_flags =
            self.incompat_flags & !MAVLINK_IFLAG_SIGNED | (MAVLINK_IFLAG_SIGNED & flag as u8);
    }

    /// MAVLink frame body length.
    ///
    /// Calculates expected size in bytes for frame body. Depends on MAVLink protocol version and presence of
    /// signature (when [`MAVLINK_IFLAG_SIGNED`] incompatibility flag is set).
    ///
    /// # Links
    /// * [`Frame::signature`](crate::protocol::Frame::signature).
    pub fn body_length(&self) -> usize {
        match self.mavlink_version {
            MavLinkVersion::V1 => self.payload_length as usize + CHECKSUM_SIZE,
            MavLinkVersion::V2 => {
                if self.is_signed() {
                    self.payload_length as usize + CHECKSUM_SIZE + SIGNATURE_LENGTH
                } else {
                    self.payload_length as usize + CHECKSUM_SIZE
                }
            }
        }
    }

    /// Decodes [`Header`] as [`HeaderBytes`].
    ///
    /// Returns header data encoded as a sequence of bytes.
    pub fn decode(&self) -> HeaderBytes {
        let mut header_bytes = HeaderBytes {
            size: self.size(),
            ..Default::default()
        };
        self.dump_bytes(&mut header_bytes);
        header_bytes
    }

    pub(super) fn recv<R: Read>(reader: &mut R) -> Result<Self> {
        loop {
            let mut buffer = [0u8; HEADER_MIN_SIZE];
            reader.read_exact(&mut buffer)?;

            let mut mavlink_version: Option<MavLinkVersion> = None;
            let mut header_start_idx = buffer.len();
            for (i, &byte) in buffer.iter().enumerate() {
                if MavSTX::is_magic_byte(byte) {
                    header_start_idx = i;
                    mavlink_version = MavLinkVersion::try_from(MavSTX::from(byte)).ok();
                }
            }

            match mavlink_version {
                None => continue,
                Some(version) => {
                    let header_size = match version {
                        MavLinkVersion::V1 => HEADER_V1_SIZE,
                        MavLinkVersion::V2 => HEADER_V2_SIZE,
                    };

                    let num_read_bytes = buffer.len() - header_start_idx;
                    let header_start_bytes = &buffer[header_start_idx..buffer.len()];

                    let mut header_bytes = [0u8; HEADER_MAX_SIZE];
                    header_bytes[0..num_read_bytes].copy_from_slice(header_start_bytes);

                    if num_read_bytes < header_size {
                        reader.read_exact(&mut header_bytes[num_read_bytes..header_size])?;
                    }

                    return Self::try_from_slice(&header_bytes);
                }
            }
        }
    }

    pub(super) fn send<W: Write>(&self, writer: &mut W) -> Result<usize> {
        writer.write_all(self.decode().as_slice())?;
        Ok(self.size())
    }

    fn try_from_slice(bytes: &[u8]) -> Result<Self> {
        let reader = TBytesReader::from(bytes);

        let magic: u8 = reader.read()?;
        let mavlink_version: MavLinkVersion = MavLinkVersion::try_from(MavSTX::from(magic))?;
        let payload_length: u8 = reader.read()?;

        let (incompat_flags, compat_flags) = if let MavLinkVersion::V2 = mavlink_version {
            let incompat_flags = reader.read()?;
            let compat_flags = reader.read()?;
            (incompat_flags, compat_flags)
        } else {
            (0, 0)
        };

        let sequence: u8 = reader.read()?;
        let system_id: u8 = reader.read()?;
        let component_id: u8 = reader.read()?;

        let message_id: MessageId = match mavlink_version {
            MavLinkVersion::V1 => {
                let version: u8 = reader.read()?;
                version as MessageId
            }
            MavLinkVersion::V2 => {
                let version_byte: [u8; 4] = [reader.read()?, reader.read()?, reader.read()?, 0];
                MessageId::from_le_bytes(version_byte)
            }
        };

        let mut header_bytes = [0u8; HEADER_MAX_SIZE];
        header_bytes[0..bytes.len()].copy_from_slice(bytes);

        Ok(Self {
            mavlink_version,
            payload_length,
            incompat_flags,
            compat_flags,
            sequence,
            system_id,
            component_id,
            message_id,
        })
    }

    fn dump_bytes(&self, header_bytes: &mut HeaderBytes) {
        match self.mavlink_version {
            MavLinkVersion::V1 => self.dump_v1_bytes(header_bytes),
            MavLinkVersion::V2 => self.dump_v2_bytes(header_bytes),
        };
    }

    fn dump_v1_bytes(&self, header_bytes: &mut HeaderBytes) {
        header_bytes.buffer[0] = MavSTX::V1.into();
        header_bytes.buffer[1] = self.payload_length;
        header_bytes.buffer[2] = self.sequence;
        header_bytes.buffer[3] = self.system_id;
        header_bytes.buffer[4] = self.component_id;
        header_bytes.buffer[5] = self.message_id.to_le_bytes()[0];
    }

    fn dump_v2_bytes(&self, header_bytes: &mut HeaderBytes) {
        let message_id: [u8; 4] = self.message_id.to_le_bytes();

        header_bytes.buffer[0] = MavSTX::V2.into();
        header_bytes.buffer[1] = self.payload_length;
        header_bytes.buffer[2] = self.incompat_flags;
        header_bytes.buffer[3] = self.compat_flags;
        header_bytes.buffer[4] = self.sequence;
        header_bytes.buffer[5] = self.system_id;
        header_bytes.buffer[6] = self.component_id;
        header_bytes.buffer[7..10].copy_from_slice(&message_id[0..3]);
    }
}

impl HeaderBuilder {
    /// Default constructor.
    pub fn new() -> HeaderBuilder {
        Self::default()
    }

    /// Build [`Header`] for specific [`MavLinkVersion`].
    pub fn build(&self, mavlink_version: MavLinkVersion) -> Result<Header> {
        self.validate(mavlink_version)?;

        // Prepare header
        let mut header = Header {
            mavlink_version,
            ..Default::default()
        };

        macro_rules! set_required_field {
            ($field: ident) => {
                match self.$field {
                    Some($field) => header.$field = $field,
                    None => {
                        return Err(FrameError::MissingHeaderField(stringify!($field).into()).into())
                    }
                }
            };
        }
        set_required_field!(payload_length);
        set_required_field!(sequence);
        set_required_field!(system_id);
        set_required_field!(component_id);
        set_required_field!(message_id);

        if let MavLinkVersion::V2 = mavlink_version {
            if let Some(incompat_flags) = self.incompat_flags {
                header.incompat_flags = incompat_flags;
            }
            if let Some(compat_flags) = self.compat_flags {
                header.compat_flags = compat_flags;
            }
        }

        Ok(header)
    }

    /// Sets incompatibility flags for `MAVLink 2` header.
    ///
    /// # Errors
    ///
    /// Does not returns error directly but if both MAVLink version is set to [`MavLinkVersion::V1`] and incompatibility
    /// flags are present, then [`FrameError::InconsistentV1Header`] error will be returned by [`Self::build`].
    pub fn set_incompat_flags(&mut self, incompat_flags: IncompatFlags) -> &mut Self {
        self.incompat_flags = Some(incompat_flags);
        self
    }

    /// Sets compatibility flags for `MAVLink 2` header.
    ///
    /// # Errors
    ///
    /// Does not returns error directly but if both MAVLink version is set to [`MavLinkVersion::V1`] and compatibility
    /// flags are present, then [`FrameError::InconsistentV1Header`] error will be returned by [`Self::build`].
    pub fn set_compat_flags(&mut self, compat_flags: CompatFlags) -> &mut Self {
        self.compat_flags = Some(compat_flags);
        self
    }

    /// Sets payload length.
    ///
    /// See: [`Header::payload_length`].
    pub fn set_payload_length(&mut self, payload_length: u8) -> &mut Self {
        self.payload_length = Some(payload_length);
        self
    }

    /// Sets packet sequence number.
    ///
    /// See: [`Header::sequence`].
    pub fn set_sequence(&mut self, sequence: u8) -> &mut Self {
        self.sequence = Some(sequence);
        self
    }

    /// Sets system `ID`.
    ///
    /// See: [`Header::system_id`].
    pub fn set_system_id(&mut self, system_id: u8) -> &mut Self {
        self.system_id = Some(system_id);
        self
    }

    /// Sets component `ID`.
    ///
    /// See: [`Header::component_id`].
    pub fn set_component_id(&mut self, component_id: u8) -> &mut Self {
        self.component_id = Some(component_id);
        self
    }

    /// Sets message `ID`.
    ///
    /// See: [`Header::message_id`].
    pub fn set_message_id(&mut self, message_id: u32) -> &mut Self {
        self.message_id = Some(message_id);
        self
    }

    /// Sets whether `MAVLink 2` frame body should contain signature.
    ///
    /// Sets [`MAVLINK_IFLAG_SIGNED`] flag for `incompat_flags`.
    ///
    /// # Links
    ///
    /// * [`Frame::signature`](crate::protocol::Frame::signature).
    pub fn set_is_signed(&mut self, flag: bool) -> &mut Self {
        match self.incompat_flags {
            None => {
                self.incompat_flags = Some(MAVLINK_IFLAG_SIGNED & flag as u8);
            }
            Some(incompat_flags) => {
                self.incompat_flags = Some(
                    incompat_flags & !MAVLINK_IFLAG_SIGNED | (MAVLINK_IFLAG_SIGNED & flag as u8),
                );
            }
        }
        self
    }

    /// Validates header builder data for consistency.
    ///
    /// # Errors
    ///
    /// * Returns [`FrameError::InconsistentV1Header`] if MAVLink version is set to [`MavLinkVersion::V1`] but either
    /// [Self::set_incompat_flags] or [`Self::set_compat_flags`] were set. These fields are not allowed for `MAVLink 1`
    /// headers.
    /// * Returns [`FrameError::MissingHeaderField`] if required fields are missing.
    pub fn validate(&self, mavlink_version: MavLinkVersion) -> Result<()> {
        self.validate_required_fields()?;
        match mavlink_version {
            MavLinkVersion::V1 => self.validate_v1_fields()?,
            MavLinkVersion::V2 => self.validate_v2_fields()?,
        }
        Ok(())
    }

    fn validate_required_fields(&self) -> Result<()> {
        macro_rules! required_field {
            ($field: ident) => {
                if self.$field.is_none() {
                    return Err(FrameError::MissingHeaderField(stringify!($field).into()).into());
                }
            };
        }
        required_field!(payload_length);
        required_field!(sequence);
        required_field!(system_id);
        required_field!(component_id);
        required_field!(message_id);
        Ok(())
    }

    fn validate_v1_fields(&self) -> Result<()> {
        if self.incompat_flags.is_some() || self.compat_flags.is_some() {
            return Err(FrameError::InconsistentV1Header.into());
        }
        match self.message_id {
            Some(message_id) if message_id > MESSAGE_ID_V1_MAX => {
                return Err(FrameError::MessageIdV1OutOfBounds.into());
            }
            _ => {}
        }
        Ok(())
    }

    fn validate_v2_fields(&self) -> Result<()> {
        match self.message_id {
            Some(message_id) if message_id > MESSAGE_ID_V2_MAX => {
                return Err(FrameError::MessageIdV2OutOfBounds.into());
            }
            _ => {}
        }
        Ok(())
    }
}

#[cfg(test)]
#[cfg(feature = "std")]
mod tests {
    use super::*;
    use crate::consts::{STX_V1, STX_V2};
    use crate::errors::CoreError;
    use std::io::Cursor;

    #[test]
    fn set_get_is_signed() {
        let mut header = Header::builder()
            .set_payload_length(10)
            .set_sequence(5)
            .set_system_id(10)
            .set_component_id(240)
            .set_message_id(42)
            .build(MavLinkVersion::V2)
            .unwrap();

        assert!(!header.is_signed());

        header.set_is_signed(true);
        assert!(header.is_signed());

        header.incompat_flags = 0b01010100;
        assert!(!header.is_signed());

        header.set_is_signed(true);
        assert!(header.is_signed());
        assert_eq!(header.incompat_flags, 0b01010101);

        header.set_is_signed(false);
        assert_eq!(header.incompat_flags, 0b01010100);
    }

    #[test]
    fn read_v1_header() {
        let mut buffer = Cursor::new(vec![
            12,     // \
            24,     //  | Junk bytes
            240,    // /
            STX_V1, // magic byte
            8,      // payload_length
            1,      // sequence
            10,     // system ID
            255,    // component ID
            0,      // message ID
        ]);

        let header = Header::recv(&mut buffer).unwrap();

        assert!(matches!(header.mavlink_version(), MavLinkVersion::V1));
        assert_eq!(header.payload_length(), 8u8);
        assert_eq!(header.sequence(), 1u8);
        assert_eq!(header.system_id(), 10u8);
        assert_eq!(header.component_id(), 255u8);
        assert_eq!(header.message_id(), 0u32);
    }

    #[test]
    fn read_v2_header() {
        let mut reader = Cursor::new(vec![
            12,     // \
            24,     //  |Junk bytes
            240,    // /
            STX_V2, // magic byte
            8,      // payload_length
            1,      // incompatibility flags
            0,      // compatibility flags
            1,      // sequence
            10,     // system ID
            255,    // component ID
            0,      // \
            0,      //  | message ID
            0,      // /
        ]);

        let header = Header::recv(&mut reader).unwrap();

        assert!(matches!(header.mavlink_version(), MavLinkVersion::V2));
        assert_eq!(header.payload_length(), 8u8);
        assert_eq!(header.incompat_flags().unwrap(), 1u8);
        assert_eq!(header.compat_flags().unwrap(), 0u8);
        assert_eq!(header.sequence(), 1u8);
        assert_eq!(header.system_id(), 10u8);
        assert_eq!(header.component_id(), 255u8);
        assert_eq!(header.message_id(), 0u32);
    }

    #[test]
    fn build_v1_header() {
        let header = Header::builder()
            .set_payload_length(10)
            .set_sequence(5)
            .set_system_id(10)
            .set_component_id(240)
            .set_message_id(42)
            .build(MavLinkVersion::V1);

        assert!(header.is_ok());
        let header = header.unwrap();

        assert!(matches!(header.mavlink_version(), MavLinkVersion::V1));
        assert_eq!(header.payload_length(), 10);
        assert_eq!(header.sequence(), 5);
        assert_eq!(header.system_id(), 10);
        assert_eq!(header.component_id(), 240);
        assert_eq!(header.message_id(), 42);
    }

    #[test]
    fn build_v2_header() {
        let header = Header::builder()
            .set_incompat_flags(MAVLINK_IFLAG_SIGNED)
            .set_compat_flags(8)
            .set_payload_length(10)
            .set_sequence(5)
            .set_system_id(10)
            .set_component_id(240)
            .set_message_id(42)
            .set_is_signed(true)
            .build(MavLinkVersion::V2);

        assert!(header.is_ok());
        let header = header.unwrap();

        assert!(matches!(header.mavlink_version(), MavLinkVersion::V2));
        assert_eq!(header.incompat_flags().unwrap(), MAVLINK_IFLAG_SIGNED);
        assert_eq!(header.compat_flags().unwrap(), 8);
        assert_eq!(header.payload_length(), 10);
        assert_eq!(header.sequence(), 5);
        assert_eq!(header.system_id(), 10);
        assert_eq!(header.component_id(), 240);
        assert_eq!(header.message_id(), 42);
    }

    #[test]
    fn builder_validates_required_fields() {
        let header = Header::builder().build(MavLinkVersion::V2);

        assert!(header.is_err());
        assert!(matches!(
            header,
            Err(CoreError::Frame(FrameError::MissingHeaderField(_)))
        ));
    }
}
