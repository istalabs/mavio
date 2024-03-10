use crate::prelude::*;

use crate::protocol::{
    Checksum, CrcExtra, MessageId, Payload, PayloadLength, Sequence, Signature, SystemId,
};

/// Version-agnostic MAVLink frame, that can be matched according to its protocol version.
///
/// [`MavFrame`] allows to work with MAVLink frames dynamically, but still type-safely, where
/// generic [`Frame`] is suboptimal. It has almost the same protocol-agnostic methods as [`Frame`],
/// to access protocol-specific functionality, you have to  convert or pattern match [`MavFrame`]
/// back into a [`Frame`].
///
/// It is not possible to create [`MavFrame`] directly. An instance of [`Frame`] should be obtained
/// first.
///
/// # Examples
///
/// ```rust
/// # use mavio::dialects::minimal::messages::Heartbeat;
/// use mavio::prelude::*;
///
/// let frame: Frame<_> = // ... obtain a frame
/// # Frame::builder().version(V2).sequence(1).system_id(1).component_id(1).message(&Heartbeat::default()).unwrap().build();
///
/// let mav_frame = MavFrame::new(frame);
///
/// match mav_frame {
///     MavFrame::V1(_) => { /* `MAVLink 1` specific logic */ }
///     MavFrame::V2(_) => { /* `MAVLink 2` specific logic */ }
/// }
///
/// let frame: Frame<Versionless> = mav_frame.into_versionless();
/// ```
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MavFrame {
    /// `MAVLink 1` frame.
    V1(Frame<V1>),
    /// `MAVLink 2` frame.
    V2(Frame<V2>),
}

impl MavFrame {
    /// Creates [`MavFrame`] from the provided [`Frame`].
    pub fn new<V: MaybeVersioned>(frame: Frame<V>) -> Self {
        match frame.version() {
            MavLinkVersion::V1 => MavFrame::V1(frame.try_into_versioned::<V1>().unwrap()),
            MavLinkVersion::V2 => MavFrame::V2(frame.try_into_versioned::<V2>().unwrap()),
        }
    }

    /// `MAVLink` version.
    ///
    /// See: [`Frame::version`].
    #[inline]
    pub fn version(&self) -> MavLinkVersion {
        match self {
            MavFrame::V1(_) => MavLinkVersion::V1,
            MavFrame::V2(_) => MavLinkVersion::V2,
        }
    }

    /// Payload length.
    ///
    /// See: [`Frame::payload_length`].
    #[inline]
    pub fn payload_length(&self) -> PayloadLength {
        match self {
            MavFrame::V1(frame) => frame.payload_length(),
            MavFrame::V2(frame) => frame.payload_length(),
        }
    }

    /// Frame sequence.
    ///
    /// See: [`Frame::sequence`].
    #[inline]
    pub fn sequence(&self) -> Sequence {
        match self {
            MavFrame::V1(frame) => frame.sequence(),
            MavFrame::V2(frame) => frame.sequence(),
        }
    }

    /// System `ID`.
    ///
    /// See: [`Frame::system_id`].
    #[inline]
    pub fn system_id(&self) -> SystemId {
        match self {
            MavFrame::V1(frame) => frame.system_id(),
            MavFrame::V2(frame) => frame.system_id(),
        }
    }

    /// Component `ID`.
    ///
    /// See: [`Frame::component_id`].
    #[inline]
    pub fn component_id(&self) -> SystemId {
        match self {
            MavFrame::V1(frame) => frame.component_id(),
            MavFrame::V2(frame) => frame.component_id(),
        }
    }

    /// Message `ID`.
    ///
    /// See: [`Frame::message_id`].
    #[inline]
    pub fn message_id(&self) -> MessageId {
        match self {
            MavFrame::V1(frame) => frame.message_id(),
            MavFrame::V2(frame) => frame.message_id(),
        }
    }

    /// Payload data.
    ///
    /// See: [`Frame::payload`].
    #[inline]
    pub fn payload(&self) -> &Payload {
        match self {
            MavFrame::V1(frame) => frame.payload(),
            MavFrame::V2(frame) => frame.payload(),
        }
    }

    /// MAVLink packet checksum.
    ///
    /// See: [`Frame::checksum`].
    #[inline]
    pub fn checksum(&self) -> Checksum {
        match self {
            MavFrame::V1(frame) => frame.checksum(),
            MavFrame::V2(frame) => frame.checksum(),
        }
    }

    /// Returns `true` if frame is signed.
    ///
    /// For `MAVLink 1` frames always returns `false`.
    ///
    /// See: [`Frame::is_signed`].
    #[inline]
    pub fn is_signed(&self) -> bool {
        match self {
            MavFrame::V1(_) => false,
            MavFrame::V2(frame) => frame.is_signed(),
        }
    }

    /// `MAVLink 2` signature.
    ///
    /// See: [`Frame::signature`].
    #[inline]
    pub fn signature(&self) -> Option<&Signature> {
        match self {
            MavFrame::V1(_) => None,
            MavFrame::V2(frame) => frame.signature(),
        }
    }

    /// Removes `MAVLink 2` signature from frame.
    ///
    /// Applicable only for `MAVLink 2` frames. `MAVLink 1` frames will be kept untouched.
    ///
    /// See: [`Frame::remove_signature`].
    #[inline]
    pub fn remove_signature(&mut self) {
        match self {
            MavFrame::V1(_) => (),
            MavFrame::V2(frame) => frame.remove_signature(),
        }
    }

    /// Body length.
    ///
    /// See: [`Frame::body_length`].
    #[inline]
    pub fn body_length(&self) -> usize {
        match self {
            MavFrame::V1(frame) => frame.body_length(),
            MavFrame::V2(frame) => frame.body_length(),
        }
    }

    /// Calculates CRC for frame within `crc_extra`.
    ///
    /// See: [`Frame::calculate_crc`].
    #[inline]
    pub fn calculate_crc(&self, crc_extra: CrcExtra) -> Checksum {
        match self {
            MavFrame::V1(frame) => frame.calculate_crc(crc_extra),
            MavFrame::V2(frame) => frame.calculate_crc(crc_extra),
        }
    }

    /// Validates frame in the context of specific dialect.
    ///
    /// See: [`Frame::validate_checksum`].
    #[inline]
    pub fn validate_checksum<D: Dialect>(&self) -> Result<()> {
        match self {
            MavFrame::V1(frame) => frame.validate_checksum::<D>(),
            MavFrame::V2(frame) => frame.validate_checksum::<D>(),
        }
    }

    /// Validates frame's checksum using provided `crc_extra`.
    ///
    /// See: [`Frame::validate_checksum_with_crc_extra`].
    #[inline]
    pub fn validate_checksum_with_crc_extra(&self, crc_extra: CrcExtra) -> Result<()> {
        match self {
            MavFrame::V1(frame) => frame.validate_checksum_with_crc_extra(crc_extra),
            MavFrame::V2(frame) => frame.validate_checksum_with_crc_extra(crc_extra),
        }
    }

    /// Checks that frame has MAVLink version equal to the provided one.
    ///
    /// See: [`Frame::matches_version`]
    #[inline]
    pub fn matches_version<Version: Versioned>(
        &self,
        #[allow(unused_variables)] version: Version,
    ) -> bool {
        Version::matches(self.version())
    }

    /// Decodes frame into a message of particular MAVLink dialect.
    ///
    /// See: [`Frame::decode`].
    #[inline]
    pub fn decode<D: Dialect>(&self) -> Result<D> {
        match self {
            MavFrame::V1(frame) => frame.decode(),
            MavFrame::V2(frame) => frame.decode(),
        }
    }

    /// Converts [`MavFrame`] into a versionless [`Frame`].
    pub fn into_versionless(self) -> Frame<Versionless> {
        match self {
            MavFrame::V1(frame) => frame.into_versionless(),
            MavFrame::V2(frame) => frame.into_versionless(),
        }
    }

    /// Attempts to convert into a versioned form of a [`Frame`].
    ///
    /// Returns [`FrameError::InvalidVersion`] variant of [`Error::Frame`] if conversion is
    /// impossible.
    pub fn try_into_versioned<V: MaybeVersioned>(self) -> Result<Frame<V>> {
        match self {
            MavFrame::V1(frame) => frame.try_into_versioned::<V>(),
            MavFrame::V2(frame) => frame.try_into_versioned::<V>(),
        }
    }
}

impl<V: MaybeVersioned> TryFrom<MavFrame> for Frame<V> {
    type Error = Error;

    fn try_from(value: MavFrame) -> Result<Self> {
        value.try_into_versioned()
    }
}

impl<V: MaybeVersioned> From<Frame<V>> for MavFrame {
    fn from(value: Frame<V>) -> Self {
        Self::new(value)
    }
}
