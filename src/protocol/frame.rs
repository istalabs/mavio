//! # MAVLink frame

use crc_any::CRCu16;

use crate::consts::{CHECKSUM_SIZE, SIGNATURE_LENGTH};
use crate::errors::{FrameError, Result};
use crate::io::{Read, Write};
use crate::protocol::header::{Header, HeaderBuilder};
use crate::protocol::signature::{Sign, Signature, SignatureConf};
use crate::protocol::{
    Checksum, CompatFlags, CrcExtra, DialectSpec, IncompatFlags, MavLinkVersion, MavTimestamp,
    MessageId, MessageImpl, Payload, SecretKey, SignatureBytes, SignatureLinkId, SignatureValue,
};

/// MAVLink frame.
///
/// Since MAVLink frames has a complex internal structure depending on [`MavLinkVersion`], encoded [`MessageImpl`]
/// and presence of [`Signature`], there are no constructor for this struct. [`Frame`] can be either received as they
/// were sent by remote or built from [`FrameBuilder`].
///
/// Use [`Frame::builder`] to create a new unsigned message and [`Frame::add_signature`]/[`Frame::replace_signature`]
/// to manage signature of exising frame.  
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Frame {
    header: Header,
    payload: Payload,
    checksum: Checksum,
    signature: Option<Signature>,
}

/// Configuration from which [`Frame`] can be built.
///
/// Implements [builder](https://rust-unofficial.github.io/patterns/patterns/creational/builder.html)
/// pattern for [`Frame`]. Once all configuration parameters are set, the client calls [`FrameBuilder::build`] or
/// [`FrameBuilder::build_for`] to obtain an instance of [`Frame`].
///
/// > **Note!** Frames built by [`FrameBuilder`] are always unsigned and `MAVLINK_IFLAG_SIGNED` flag in
/// > [`Frame::incompat_flags`] is always dropped. Use [`Frame::add_signature`] to sign an existing frame.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FrameBuilder {
    header_conf: HeaderBuilder,
    payload: Option<Payload>,
    crc_extra: Option<CrcExtra>,
}

impl Frame {
    /// Instantiates a [builder](https://rust-unofficial.github.io/patterns/patterns/creational/builder.html) for
    /// [`Frame`].
    ///
    /// An instance of [`FrameBuilder`] returned by this function is initialized with default values. Once desired frame
    /// parameters are set, use [`FrameBuilder::build`] or [`FrameBuilder::build_for`] to obtain a valid
    /// instance of [`Frame`].
    pub fn builder() -> FrameBuilder {
        FrameBuilder::new()
    }

    /// Generic MAVLink header.
    ///
    /// # Links
    ///
    /// * [`Header`] implementation.
    #[inline]
    pub fn header(&self) -> &Header {
        &self.header
    }

    /// MAVLink protocol version defined by [`Header`].
    ///
    /// # Links
    ///
    /// * [MavLinkVersion]
    /// * [Header::mavlink_version]
    #[inline]
    pub fn mavlink_version(&self) -> MavLinkVersion {
        self.header.mavlink_version()
    }

    /// Incompatibility flags for `MAVLink 2` header.
    ///
    /// Flags that must be understood for MAVLink compatibility (implementation discards packet if
    /// it does not understand flag).
    ///
    /// See: [MAVLink 2 incompatibility flags](https://mavlink.io/en/guide/serialization.html#incompat_flags).
    #[inline]
    pub fn incompat_flags(&self) -> Option<u8> {
        self.header.incompat_flags()
    }

    /// Compatibility flags for `MAVLink 2` header.
    ///
    /// Flags that can be ignored if not understood (implementation can still handle packet even if
    /// it does not understand flag).
    ///
    /// See: [MAVLink 2 compatibility flags](https://mavlink.io/en/guide/serialization.html#compat_flags).
    #[inline]
    pub fn compat_flags(&self) -> Option<u8> {
        self.header.compat_flags()
    }

    /// Payload length.
    ///
    /// Indicates length of the following `payload` section. This may be affected by payload truncation.
    ///
    /// # Links
    ///
    /// * [Header::payload_length].
    #[inline]
    pub fn payload_length(&self) -> u8 {
        self.header.payload_length()
    }

    /// Packet sequence number.
    ///
    /// Used to detect packet loss. Components increment value for each message sent.
    ///
    /// # Links
    ///
    /// * [Header::sequence].
    #[inline]
    pub fn sequence(&self) -> u8 {
        self.header.sequence()
    }

    /// System `ID`.
    ///
    /// `ID` of system (vehicle) sending the message. Used to differentiate systems on network.
    ///
    /// > Note that the broadcast address 0 may not be used in this field as it is an invalid source
    /// > address.
    ///
    /// # Links
    ///
    /// * [Header::system_id].
    #[inline]
    pub fn system_id(&self) -> u8 {
        self.header.system_id()
    }

    /// Component `ID`.
    ///
    /// `ID` of component sending the message. Used to differentiate components in a system (e.g.
    /// autopilot and a camera). Use appropriate values in
    /// [MAV_COMPONENT](https://mavlink.io/en/messages/common.html#MAV_COMPONENT).
    ///
    /// > Note that the broadcast address `MAV_COMP_ID_ALL` may not be used in this field as it is
    /// > an invalid source address.
    ///
    /// # Links
    ///
    /// * [Header::component_id].
    #[inline]
    pub fn component_id(&self) -> u8 {
        self.header.component_id()
    }

    /// Message `ID`.
    ///
    /// `ID` of MAVLink message. Defines how payload will be encoded and decoded.
    ///
    /// # Links
    ///
    /// * [Header::message_id].
    #[inline]
    pub fn message_id(&self) -> MessageId {
        self.header.message_id()
    }

    /// Payload data.
    ///
    /// Message data. Content depends on message type (i.e. `message_id`).
    ///
    /// # Links
    ///
    /// * Payload implementation: [`Payload`].
    #[inline]
    pub fn payload(&self) -> &Payload {
        &self.payload
    }

    /// MAVLink packet checksum.
    ///
    /// `CRC-16/MCRF4XX` [checksum](https://mavlink.io/en/guide/serialization.html#checksum) for
    /// message (excluding magic byte).
    ///
    /// Includes [CRC_EXTRA](https://mavlink.io/en/guide/serialization.html#crc_extra) byte.
    ///
    /// Checksum is encoded with little endian (low byte, high byte).
    ///
    /// # Links
    ///
    /// * [`Frame::calculate_crc`] for implementation.
    /// * [MAVLink checksum definition](https://mavlink.io/en/guide/serialization.html#checksum).
    /// * [CRC-16/MCRF4XX](https://ww1.microchip.com/downloads/en/AppNotes/00752a.pdf) (PDF).
    #[inline]
    pub fn checksum(&self) -> Checksum {
        self.checksum
    }

    /// `MAVLink 2` signature.
    ///
    /// Returns signature that ensures the link is tamper-proof.
    ///
    /// Available only for signed `MAVLink 2` frame. For `MAVLink 1` always return `None`.
    ///
    /// # Links
    ///
    /// * [`Frame::is_signed`].
    /// * [`Frame::link_id`] and [`Frame::timestamp`] provide direct access to signature fields.
    /// * [MAVLink 2 message signing](https://mavlink.io/en/guide/message_signing.html).
    #[inline]
    pub fn signature(&self) -> Option<&Signature> {
        self.signature.as_ref()
    }

    /// `MAVLink 2` signature `link_id`, a 8-bit identifier of a MAVLink channel.
    ///
    /// Peers may have different semantics or rules for different links. For example, some links may have higher
    /// priority over another during routing. Or even different secret keys for authorization.
    ///
    /// Available only for signed `MAVLink 2` frame. For `MAVLink 1` always return `None`.
    ///
    /// # Links
    ///
    /// * [`Self::signature`] from which [`Signature`] can be obtained. The former contains all signature-related fields
    ///   (if applicable).
    /// * [MAVLink 2 message signing](https://mavlink.io/en/guide/message_signing.html).
    pub fn link_id(&self) -> Option<SignatureLinkId> {
        self.signature.map(|sig| sig.link_id)
    }

    /// `MAVLink 2` signature [`MavTimestamp`], a 48-bit value that specifies the moment when message was sent.
    ///
    /// The unit of measurement is the number of millisecond * 10 since MAVLink epoch (1st January 2015 GMT).
    ///
    /// According to MAVLink protocol, the sender must guarantee that the next timestamp is greater than the previous
    /// one.
    ///
    /// Available only for signed `MAVLink 2` frame. For `MAVLink 1` always return `None`.
    ///
    /// # Links
    ///
    /// * [`Self::signature`] from which [`Signature`] can be obtained. The former contains all signature-related fields
    ///   (if applicable).
    /// * [`MavTimestamp`] type which has utility function for converting from and into Unix timestamp.
    /// * [Timestamp handling](https://mavlink.io/en/guide/message_signing.html#timestamp) in MAVLink documentation.
    pub fn timestamp(&self) -> Option<MavTimestamp> {
        self.signature.map(|sig| sig.timestamp)
    }

    /// Whether a [`Frame`] is signed.
    ///
    /// Returns `true` if [`Frame`] contains [`Signature`]. Correctness of signature is not validated.
    ///
    /// For `MAVLink 1` always returns `false`.
    #[inline]
    pub fn is_signed(&self) -> bool {
        self.signature.is_some()
    }

    /// Body length.
    ///
    /// Returns the length of the entire [`Frame`] body. The frame body consist of [`Payload::bytes`], [`Checksum`], and
    /// optional [`Signature`] (for `MAVLink 2` protocol).
    ///
    /// # Links
    ///
    /// * [`Header::body_length`].
    #[inline]
    pub fn body_length(&self) -> usize {
        self.header().body_length()
    }

    /// Calculates CRC for [`Frame`] within `crc_extra`.
    ///
    /// Provided `crc_extra` depends on a dialect and contains a digest of message XML definition.
    ///
    /// # Links
    ///
    /// * [`Frame::checksum`].
    /// * [MAVLink checksum definition](https://mavlink.io/en/guide/serialization.html#checksum).
    /// * [CRC-16/MCRF4XX](https://ww1.microchip.com/downloads/en/AppNotes/00752a.pdf) (PDF).
    pub fn calculate_crc(&self, crc_extra: CrcExtra) -> Checksum {
        let mut crc_calculator = CRCu16::crc16mcrf4cc();

        crc_calculator.digest(self.header.decode().crc_data());
        crc_calculator.digest(self.payload.bytes());

        crc_calculator.digest(&[crc_extra]);

        crc_calculator.get_crc()
    }

    /// Validates frame in the context of specific dialect.
    ///
    /// Receives dialect specification in `dialect_spec`, ensures that message with such ID
    /// exists in this dialect, and compares checksums using `EXTRA_CRC`.
    ///
    /// # Errors
    ///
    /// * Returns [`CoreError::Message`](crate::errors::CoreError::Message) if message discovery failed.  
    /// * Returns [`FrameError::InvalidChecksum`] (wrapped by [`CoreError`](crate::errors::CoreError)) if checksum
    ///   validation failed.
    ///
    /// # Links
    ///
    /// * [`DialectSpec`] for dialect specification.
    /// * [`Frame::calculate_crc`] for CRC implementation details.
    pub fn validate_checksum(&self, dialect_spec: &dyn DialectSpec) -> Result<()> {
        let message_info = dialect_spec.message_info(self.header().message_id())?;
        self.validate_checksum_with_crc_extra(message_info.crc_extra())?;

        Ok(())
    }

    /// Validates [`Frame::checksum`] using provided `crc_extra`.
    ///
    /// # Links
    ///
    /// * [`Frame::calculate_crc`] for CRC implementation details.
    pub fn validate_checksum_with_crc_extra(&self, crc_extra: CrcExtra) -> Result<()> {
        if self.calculate_crc(crc_extra) != self.checksum {
            return Err(FrameError::InvalidChecksum.into());
        }

        Ok(())
    }

    /// Adds signature to `MAVLink 2` frame.
    ///
    /// Signs `MAVLink 2` frame with provided instance of `signer` that implements [`Sign`] trait and signature
    /// configuration specified as [`SignatureConf`].
    ///
    /// # Errors
    ///
    /// Returns [`FrameError::SigningIsNotSupported`] when caller attempts to sign `MAVLink 1` frame.
    ///
    /// # Links
    ///
    /// * [`Sign`] trait.
    /// * [`Signature`] struct which contains frame signature.
    /// * [MAVLink 2 message signing](https://mavlink.io/en/guide/message_signing.html).
    pub fn add_signature(
        &mut self,
        signer: &mut dyn Sign,
        conf: SignatureConf,
    ) -> Result<&mut Self> {
        self.validate_can_be_signed()?;
        self.header.set_is_signed(true);

        self.signature = Some(Signature {
            link_id: conf.link_id,
            timestamp: conf.timestamp,
            value: Default::default(),
        });

        let signature_bytes = self.calculate_signature(signer, &conf.secret);
        if let Some(sig) = self.signature.as_mut() {
            sig.value = signature_bytes?
        }

        Ok(self)
    }

    /// Replaces existing signature for `MAVLink 2` frame.
    ///
    /// Re-signs `MAVLink 2` frame with provided instance of `signer` that implements [`Sign`]. An instance of [`Frame`]
    /// should already have a (possibly invalid) signature.
    ///
    /// # Errors
    ///
    /// * Returns [`FrameError::SigningIsNotSupported`] when caller attempts to sign `MAVLink 1` frame.
    /// * Returns [`FrameError::SignatureFieldsAreMissing`] if frame is not already signed.
    ///
    /// # Links
    ///
    /// * [`Sign`] trait.
    /// * [`Signature`] struct which contains frame signature.
    /// * [MAVLink 2 message signing](https://mavlink.io/en/guide/message_signing.html).
    pub fn replace_signature(
        &mut self,
        signer: &mut dyn Sign,
        conf: SignatureConf,
    ) -> Result<&mut Self> {
        self.validate_can_be_signed()?;

        let signature_bytes = self.calculate_signature(signer, &conf.secret);

        if let Some(sig) = self.signature.as_mut() {
            sig.value = signature_bytes?
        }

        Ok(self)
    }

    /// Removes `MAVLink 2` signature from [`Frame`].
    ///
    /// Applicable only for `MAVLink 2` frames.
    pub fn remove_signature(&mut self) -> &mut Self {
        self.signature = None;
        self.header.set_is_signed(false);
        self
    }

    /// Calculates `MAVLink 2` signature.
    ///
    /// Calculates `MAVLink 2` frame signature with provided instance of `signer` that implements [`Sign`] trait and signature
    /// configuration specified as [`SignatureConf`].
    ///
    /// # Errors
    ///
    /// Returns [`FrameError::SigningIsNotSupported`] when caller attempts to sign `MAVLink 1` frame.
    ///
    /// # Links
    ///
    /// * [`Sign`] trait.
    /// * [`Signature`] struct which contains frame signature.
    /// * [MAVLink 2 message signing](https://mavlink.io/en/guide/message_signing.html).
    pub fn calculate_signature(
        &self,
        signer: &mut dyn Sign,
        secret_key: &SecretKey,
    ) -> Result<SignatureValue> {
        if self.signature.is_none() {
            return Err(FrameError::SignatureFieldsAreMissing.into());
        }

        signer.reset();

        signer.digest(secret_key);
        signer.digest(self.header.decode().as_slice());
        signer.digest(self.payload.bytes());
        signer.digest(&self.checksum.to_le_bytes());
        signer.digest(&[self.signature.unwrap().link_id]);
        signer.digest(&self.signature.unwrap().timestamp.to_bytes_array());

        Ok(signer.signature())
    }

    pub(crate) fn recv<R: Read>(reader: &mut R) -> Result<Self> {
        // Retrieve header
        let header = Header::recv(reader)?;

        let body_length = header.body_length();

        // Prepare buffer that will contain the entire message body (with signature if expected)
        #[cfg(feature = "std")]
        let mut body_buf = vec![0u8; body_length];
        #[cfg(not(feature = "std"))]
        let mut body_buf = [0u8; crate::consts::PAYLOAD_MAX_SIZE + SIGNATURE_LENGTH];
        let body_bytes = &mut body_buf[0..body_length];

        // Read and decode
        reader.read_exact(body_bytes)?;
        let frame = Self::try_from_raw_body(&header, body_bytes)?;

        Ok(frame)
    }

    pub(crate) fn send<W: Write>(&self, writer: &mut W) -> Result<usize> {
        // Validate payload length consistency
        if self.payload_length() != self.payload.length() {
            return Err(FrameError::InconsistentPayloadSize.into());
        }
        let payload_length = self.payload_length() as usize;

        // Send header
        let header_bytes_sent = self.header.send(writer)?;

        // Prepare a buffer
        #[cfg(not(feature = "alloc"))]
        let mut buf = [0u8; crate::consts::PAYLOAD_MAX_SIZE + SIGNATURE_LENGTH];
        #[cfg(feature = "alloc")]
        let mut buf = vec![0u8; self.body_length()];

        // Put payload into buffer
        buf[0..payload_length].copy_from_slice(self.payload.bytes());

        // Put checksum into buffer
        let checksum_bytes: [u8; 2] = self.checksum.to_le_bytes();
        buf[payload_length..payload_length + 2].copy_from_slice(&checksum_bytes);

        // Put signature if required
        if let Some(signature) = self.signature {
            let signature_bytes: SignatureBytes = signature.to_byte_array();
            let sig_start_idx = payload_length + 2;
            buf[sig_start_idx..self.body_length()].copy_from_slice(&signature_bytes);
        }

        writer.write_all(buf.as_slice())?;

        Ok(header_bytes_sent + self.body_length())
    }

    fn validate_can_be_signed(&self) -> Result<()> {
        if let MavLinkVersion::V1 = self.mavlink_version() {
            return Err(FrameError::SigningIsNotSupported.into());
        }

        Ok(())
    }

    fn try_from_raw_body(header: &Header, body_bytes: &[u8]) -> Result<Self> {
        if body_bytes.len() != header.body_length() {
            return Err(FrameError::InconsistentBodySize.into());
        }

        let payload_bytes = &body_bytes[0..header.payload_length() as usize];
        let payload = Payload::new(header.message_id(), payload_bytes, header.mavlink_version());

        let checksum_start = header.payload_length() as usize;
        let checksum_bytes = [body_bytes[checksum_start], body_bytes[checksum_start + 1]];
        let checksum: Checksum = Checksum::from_le_bytes(checksum_bytes);

        let signature: Option<Signature> = if header.is_signed() {
            let signature_start = checksum_start + CHECKSUM_SIZE;
            let signature_bytes = &body_bytes[signature_start..signature_start + SIGNATURE_LENGTH];
            Some(Signature::try_from(signature_bytes)?)
        } else {
            None
        };

        Ok(Self {
            header: *header,
            payload,
            checksum,
            signature,
        })
    }
}

impl FrameBuilder {
    /// Default constructor.
    pub fn new() -> Self {
        Self::default()
    }

    /// Builds [`Frame`] from configuration.
    ///
    /// Validates frame configuration and creates an instance of [`Frame`].
    ///
    /// # Errors
    ///
    /// * Returns various variants of [`FrameError`] (wrapped by [`CoreError`](crate::errors::CoreError)) if validation
    /// fails.
    pub fn build(&self, mavlink_version: MavLinkVersion) -> Result<Frame> {
        let payload = match &self.payload {
            Some(payload) => payload.clone(),
            None => {
                return Err(FrameError::MissingFrameField("payload").into());
            }
        };

        let header = self
            .header_conf
            .clone()
            .set_payload_length(payload.length())
            .build(mavlink_version)?;

        let mut frame = Frame {
            header,
            payload,
            checksum: 0,
            signature: None,
        };

        frame.checksum = match self.crc_extra {
            Some(crc_extra) => frame.calculate_crc(crc_extra),
            None => {
                return Err(FrameError::MissingFrameField("crc_extra").into());
            }
        };

        Ok(frame)
    }

    /// Builds and instance of [`Frame`] for a message specified by [`MessageImpl`] and requested MAVLink protocol
    /// version.
    ///
    /// Imports and encodes MAVLink message. Uses `crc_extra` from [`MessageImpl`] to create a checksum.
    ///
    /// Uses [`MessageImpl`] to define:
    ///
    /// * [`Frame::message_id`]
    /// * [`Frame::payload_length`]
    /// * [`Frame::payload`]
    /// * [`Frame::checksum`]
    ///
    /// # Errors
    ///
    /// In addition to errors returned by [`Self::build`] it may return [`MessageError`](crate::errors::MessageError) if
    /// message is misconfigured or does not supports provided `mavlink_version`.
    pub fn build_for(
        &mut self,
        message: &dyn MessageImpl,
        mavlink_version: MavLinkVersion,
    ) -> Result<Frame> {
        let payload = message.encode(mavlink_version)?;

        self.set_message_id(message.id());
        self.set_payload(payload);
        self.set_crc_extra(message.crc_extra());

        self.build(mavlink_version)
    }

    /// Sets incompatibility flags for `MAVLink 2` header.
    ///
    /// # Errors
    ///
    /// Does not returns error directly but if both MAVLink version is set to [`MavLinkVersion::V1`] and incompatibility
    /// flags are present, then [`FrameError::InconsistentV1Header`] error will be returned by [`Self::build`].
    ///
    /// Ignores `MAVLINK_IFLAG_SIGNED` incompatibility `MAVLink 2` flag since frames build by [`FrameBuilder`] are always
    /// unsigned. Use [`Frame::add_signature`] to sign an existing frame build frame.
    pub fn set_incompat_flags(&mut self, incompat_flags: IncompatFlags) -> &mut Self {
        self.header_conf.set_incompat_flags(incompat_flags);
        // Force `MAVLINK_IFLAG_SIGNED` to be dropped.
        self.header_conf.set_is_signed(false);
        self
    }

    /// Sets compatibility flags for `MAVLink 2` header.
    ///
    /// # Errors
    ///
    /// Does not returns error directly but if both MAVLink version is set to [`MavLinkVersion::V1`] and compatibility
    /// flags are present, then [`FrameError::InconsistentV1Header`] error will be returned by [`Self::build`].
    pub fn set_compat_flags(&mut self, compat_flags: CompatFlags) -> &mut Self {
        self.header_conf.set_compat_flags(compat_flags);
        self
    }

    /// Sets packet sequence number.
    ///
    /// # Links
    ///
    /// * [`Frame::sequence`].
    pub fn set_sequence(&mut self, sequence: u8) -> &mut Self {
        self.header_conf.set_sequence(sequence);
        self
    }

    /// Sets system `ID`.
    ///
    /// # Links
    ///
    /// * [`Frame::system_id`].
    pub fn set_system_id(&mut self, system_id: u8) -> &mut Self {
        self.header_conf.set_system_id(system_id);
        self
    }

    /// Sets component `ID`.
    ///
    /// # Links
    ///
    /// * [`Frame::component_id`].
    pub fn set_component_id(&mut self, component_id: u8) -> &mut Self {
        self.header_conf.set_component_id(component_id);
        self
    }

    /// Sets message `ID`.
    ///
    /// # Links
    ///
    /// * [`Frame::message_id`].
    pub fn set_message_id(&mut self, message_id: MessageId) -> &mut Self {
        self.header_conf.set_message_id(message_id);
        self
    }

    /// Sets `CRC_EXTRA`.
    ///
    /// # Links
    ///
    /// * [`Frame::checksum`] is calculated using [`CrcExtra`].
    pub fn set_crc_extra(&mut self, crc_extra: CrcExtra) -> &mut Self {
        self.crc_extra = Some(crc_extra);
        self
    }

    /// Sets payload data.
    ///
    /// # Links
    ///
    /// * [`Frame::payload`]
    pub fn set_payload(&mut self, payload: Payload) -> &mut Self {
        self.payload = Some(payload);
        self
    }
}

#[cfg(test)]
mod tests {
    use crc_any::CRCu16;

    #[test]
    fn crc_calculation_algorithm_accepts_sequential_digests() {
        // We just want to test that CRC algorithm is invariant in respect to the way we feed it
        // data.

        let data = [124, 12, 22, 34, 2, 148, 82, 201, 72, 0, 18, 215, 37, 63u8];
        let split_at: usize = data.len() / 2;

        // Get all data as one slice
        let mut crc_calculator_bulk = CRCu16::crc16mcrf4cc();
        crc_calculator_bulk.digest(&data);

        // Get data as two chunks sequentially
        let mut crc_calculator_seq = CRCu16::crc16mcrf4cc();
        crc_calculator_seq.digest(&data[0..split_at]);
        crc_calculator_seq.digest(&data[split_at..data.len()]);

        assert_eq!(crc_calculator_bulk.get_crc(), crc_calculator_seq.get_crc());
    }

    #[test]
    #[cfg(feature = "minimal")]
    fn test_builder() {
        use crate::dialects::minimal::messages::Heartbeat;
        use crate::protocol::MavLinkVersion;
        use crate::Frame;

        let message = Heartbeat::default();
        let frame = Frame::builder()
            .set_sequence(17)
            .set_system_id(22)
            .set_component_id(17)
            .build_for(&message, MavLinkVersion::V2)
            .unwrap();

        assert!(matches!(frame.mavlink_version(), MavLinkVersion::V2));
        assert_eq!(frame.sequence(), 17);
        assert_eq!(frame.system_id(), 22);
        assert_eq!(frame.component_id(), 17);
        assert_eq!(frame.message_id(), 0);
    }

    #[test]
    #[cfg(feature = "minimal")]
    #[cfg(feature = "std")]
    fn test_signing() {
        use crate::consts::SIGNATURE_SECRET_KEY_LENGTH;
        use crate::dialects::minimal::messages::Heartbeat;
        use crate::protocol::{MavLinkVersion, SignatureConf};
        use crate::utils::MavSha256;
        use crate::Frame;

        let message = Heartbeat::default();
        let mut frame = Frame::builder()
            .set_sequence(17)
            .set_system_id(22)
            .set_component_id(17)
            .build_for(&message, MavLinkVersion::V2)
            .unwrap();

        let frame = frame.add_signature(
            &mut MavSha256::default(),
            SignatureConf {
                link_id: 0,
                timestamp: Default::default(),
                secret: [0u8; SIGNATURE_SECRET_KEY_LENGTH],
            },
        );

        let frame = frame.unwrap();
        assert!(frame.is_signed());
    }
}
