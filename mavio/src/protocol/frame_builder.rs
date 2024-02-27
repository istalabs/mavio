use core::marker::PhantomData;

use crate::protocol::marker::{
    HasCompId, HasCrcExtra, HasMsgId, HasPayload, HasPayloadLen, HasSignature, HasSysId, IsCompId,
    IsCrcExtra, IsMsgId, IsPayload, IsPayloadLen, IsSequenced, IsSigned, IsSysId, NoCompId,
    NoCrcExtra, NoMsgId, NoPayload, NoPayloadLen, NoSysId, NotSequenced, NotSigned, Sequenced,
};
use crate::protocol::{
    CompatFlags, ComponentId, CrcExtra, Endpoint, HeaderBuilder, IncompatFlags, MaybeVersioned,
    Message, MessageId, Payload, Sequence, Signature, SystemId, Unsafe, Versioned, Versionless, V1,
    V2,
};
use crate::Frame;

use crate::prelude::*;

/// Builder for [`Frame`].
///
/// Frame builder is useful, when you want to build a frame manually. In most cases we suggest to
/// use [`Endpoint::next_frame`] of a previously configured endpoint.
///
/// # Examples
///
/// Create a new frame from a heartbeat message using parameters of existing endpoint:
///
/// ```no_run
/// use mavio::dialects::minimal::messages::Heartbeat;
/// use mavio::prelude::*;
///
/// let endpoint = Endpoint::new::<V2>(MavLinkId::new(17, 42));
///
/// let frame = Frame::builder()
///     .endpoint(&endpoint)
///     .message(&Heartbeat::default()).unwrap()
///     .build();
/// ```
///
/// Create a new frame manually:
///
/// ```no_run
/// use mavio::dialects::minimal::messages::Heartbeat;
/// use mavio::protocol::{IntoPayload};
/// use mavio::prelude::*;
///
/// let message = Heartbeat::default();
/// let payload = message.encode(MavLinkVersion::V2).unwrap();
///
/// let frame = Frame::builder()
///     .version(V2)
///     .sequence(11)
///     .system_id(17)
///     .component_id(42)
///     .message_id(Heartbeat::spec().id())
///     .payload(payload.bytes())
///     .crc_extra(Heartbeat::spec().crc_extra())
///     .build();
/// ```
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FrameBuilder<
    V: MaybeVersioned,
    L: IsPayloadLen,
    Seq: IsSequenced,
    S: IsSysId,
    C: IsCompId,
    M: IsMsgId,
    P: IsPayload,
    E: IsCrcExtra,
    Sig: IsSigned,
> {
    pub(super) header_builder: HeaderBuilder<V, L, Seq, S, C, M>,
    pub(super) payload: P,
    pub(super) crc_extra: E,
    pub(super) signature: Sig,
}

impl Default
    for FrameBuilder<
        Versionless,
        NoPayloadLen,
        NotSequenced,
        NoSysId,
        NoCompId,
        NoMsgId,
        NoPayload,
        NoCrcExtra,
        NotSigned,
    >
{
    fn default() -> Self {
        Self::new()
    }
}

impl
    FrameBuilder<
        Versionless,
        NoPayloadLen,
        NotSequenced,
        NoSysId,
        NoCompId,
        NoMsgId,
        NoPayload,
        NoCrcExtra,
        NotSigned,
    >
{
    /// Default constructor.
    pub fn new() -> Self {
        Self {
            header_builder: HeaderBuilder::new(),
            payload: NoPayload,
            crc_extra: NoCrcExtra,
            signature: NotSigned,
        }
    }
}

impl<
        V: MaybeVersioned,
        L: IsPayloadLen,
        Seq: IsSequenced,
        S: IsSysId,
        C: IsCompId,
        M: IsMsgId,
        P: IsPayload,
        E: IsCrcExtra,
        Sig: IsSigned,
    > FrameBuilder<V, L, Seq, S, C, M, P, E, Sig>
{
    /// Set packet sequence number.
    ///
    /// Drops previously set [`FrameBuilder::signature`].
    ///
    /// See: [`Frame::sequence`].
    pub fn sequence(
        self,
        sequence: Sequence,
    ) -> FrameBuilder<V, L, Sequenced, S, C, M, P, E, NotSigned> {
        FrameBuilder {
            header_builder: self.header_builder.sequence(sequence),
            payload: self.payload,
            crc_extra: self.crc_extra,
            signature: NotSigned,
        }
    }

    /// Set system `ID`.
    ///
    /// Drops previously set [`FrameBuilder::signature`].
    ///
    /// See: [`Frame::system_id`].
    pub fn system_id(
        self,
        system_id: SystemId,
    ) -> FrameBuilder<V, L, Seq, HasSysId, C, M, P, E, NotSigned> {
        FrameBuilder {
            header_builder: self.header_builder.system_id(system_id),
            payload: self.payload,
            crc_extra: self.crc_extra,
            signature: NotSigned,
        }
    }

    /// Set component `ID`.
    ///
    /// See: [`Frame::component_id`].
    pub fn component_id(
        self,
        component_id: ComponentId,
    ) -> FrameBuilder<V, L, Seq, S, HasCompId, M, P, E, Sig> {
        FrameBuilder {
            header_builder: self.header_builder.component_id(component_id),
            payload: self.payload,
            crc_extra: self.crc_extra,
            signature: self.signature,
        }
    }
}

impl<
        V: MaybeVersioned,
        L: IsPayloadLen,
        Seq: IsSequenced,
        S: IsSysId,
        C: IsCompId,
        P: IsPayload,
        E: IsCrcExtra,
        Sig: IsSigned,
    > FrameBuilder<V, L, Seq, S, C, NoMsgId, P, E, Sig>
{
    /// Set message `ID`.
    ///
    /// This method drops previously set [`FrameBuilder::crc_extra`] and [`FrameBuilder::signature`]
    /// and can be called only once.
    ///
    /// See: [`Frame::message_id`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mavio::prelude::*;
    ///
    /// let frame = Frame::builder()
    ///     .message_id(0)
    ///     /* other frame parameters */
    /// # ;
    /// ```
    ///
    /// This won't compile:
    ///
    /// ```ignore
    /// use mavio::prelude::*;
    ///
    /// let frame = Frame::builder()
    ///     .message_id(0)
    ///     .message_id(1)  // won't compile
    ///     /* other frame parameters */
    /// # ;
    /// ```
    #[must_use]
    pub fn message_id(
        self,
        message_id: MessageId,
    ) -> FrameBuilder<V, L, Seq, S, C, HasMsgId, P, NoCrcExtra, NotSigned> {
        FrameBuilder {
            header_builder: self.header_builder.message_id(message_id),
            payload: self.payload,
            crc_extra: NoCrcExtra,
            signature: NotSigned,
        }
    }
}

impl<
        V: Versioned,
        L: IsPayloadLen,
        Seq: IsSequenced,
        S: IsSysId,
        C: IsCompId,
        P: IsPayload,
        E: IsCrcExtra,
        Sig: IsSigned,
    > FrameBuilder<V, L, Seq, S, C, HasMsgId, P, E, Sig>
{
    /// Set payload bytes.
    ///
    /// The size of the provided slice is not checked. Larger slices will be truncated and missing
    /// trailing bytes will be replaced with zeros.
    ///
    /// This method drops previously set [`FrameBuilder::signature`] and available only once
    /// [`FrameBuilder::message_id`] and [`FrameBuilder::version`] are defined.
    ///
    /// See: [`Frame::payload`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mavio::prelude::*;
    ///
    /// let frame = Frame::builder()
    ///     .message_id(0)
    ///     .version(V2)
    ///     .payload(&[0; 10]);
    /// ```
    #[must_use]
    pub fn payload(
        self,
        bytes: &[u8],
    ) -> FrameBuilder<V, HasPayloadLen, Seq, S, C, HasMsgId, HasPayload, E, NotSigned> {
        let payload = Payload::new(self.header_builder.message_id.0, bytes, V::version());

        FrameBuilder {
            header_builder: self
                .header_builder
                .message_id(payload.id())
                .payload_length(payload.length()),
            payload: HasPayload(payload),
            crc_extra: self.crc_extra,
            signature: NotSigned,
        }
    }
}

impl<
        V: MaybeVersioned,
        L: IsPayloadLen,
        Seq: IsSequenced,
        S: IsSysId,
        C: IsCompId,
        M: IsMsgId,
        P: IsPayload,
        Sig: IsSigned,
    > FrameBuilder<V, L, Seq, S, C, M, P, NoCrcExtra, Sig>
{
    /// Set `CRC_EXTRA`.
    ///
    /// # Links
    ///
    /// * [`Frame::checksum`] is calculated using [`CrcExtra`].
    pub fn crc_extra(
        self,
        crc_extra: CrcExtra,
    ) -> FrameBuilder<V, L, Seq, S, C, M, P, HasCrcExtra, Sig> {
        FrameBuilder {
            header_builder: self.header_builder,
            payload: self.payload,
            crc_extra: HasCrcExtra(crc_extra),
            signature: self.signature,
        }
    }
}

impl<
        L: IsPayloadLen,
        Seq: IsSequenced,
        S: IsSysId,
        C: IsCompId,
        M: IsMsgId,
        P: IsPayload,
        E: IsCrcExtra,
        Sig: IsSigned,
    > FrameBuilder<Versionless, L, Seq, S, C, M, P, E, Sig>
{
    /// Set MAVLink protocol version.
    ///
    /// This method can be called only once. When MAVLink protocol version is set, it can't be
    /// changed:
    ///
    /// ```ignore
    /// use mavio::protocol::{FrameBuilder, V1, V2};
    ///
    /// FrameBuilder::new()
    ///     .version(V1)
    ///     .version(V2); // can't set MAVLink version twice!
    /// ```
    pub fn version<Version: Versioned>(
        self,
        version: Version,
    ) -> FrameBuilder<Version, L, Seq, S, C, M, P, E, Sig> {
        FrameBuilder {
            header_builder: self.header_builder.version(version),
            payload: self.payload,
            crc_extra: self.crc_extra,
            signature: self.signature,
        }
    }
}

impl<
        L: IsPayloadLen,
        Seq: IsSequenced,
        S: IsSysId,
        C: IsCompId,
        M: IsMsgId,
        P: IsPayload,
        E: IsCrcExtra,
        Sig: IsSigned,
    > FrameBuilder<V2, L, Seq, S, C, M, P, E, Sig>
{
    /// Sets incompatibility flags for `MAVLink 2` header.
    ///
    /// Drops or sets `MAVLINK_IFLAG_SIGNED` incompatibility flag based on the presence of
    /// [`FrameBuilder::signature`].
    ///
    /// This method becomes available only once [`FrameBuilder::version`] is set to [`V2`].
    /// So, the following is okay:
    ///
    /// ```
    /// # use mavio::protocol::{FrameBuilder, IncompatFlags, V2};
    /// FrameBuilder::new()
    ///     .version(V2)
    ///     .incompat_flags(IncompatFlags::MAVLINK_IFLAG_SIGNED);
    /// ```
    ///
    /// While this won't compile:
    ///
    /// ```ignore
    /// # use mavio::protocol::{FrameBuilder, IncompatFlags};
    /// FrameBuilder::new()
    ///     .system_id(10)
    ///     .incompat_flags(IncompatFlags::MAVLINK_IFLAG_SIGNED);  // Won't compile
    /// ```
    pub fn incompat_flags(
        self,
        incompat_flags: IncompatFlags,
    ) -> FrameBuilder<V2, L, Seq, S, C, M, P, E, Sig> {
        FrameBuilder {
            header_builder: self
                .header_builder
                .incompat_flags(incompat_flags)
                .signed(self.signature.is_signed()),
            payload: self.payload,
            crc_extra: self.crc_extra,
            signature: self.signature,
        }
    }

    /// Set compatibility flags for `MAVLink 2` header.
    ///
    /// This method becomes available only once [`FrameBuilder::version`] is set to [`V2`].
    /// So, the following is okay:
    ///
    /// ```
    /// # use mavio::protocol::{CompatFlags, FrameBuilder, V2};
    /// FrameBuilder::new()
    ///     .version(V2)
    ///     .compat_flags(CompatFlags::BIT_1);
    /// ```
    ///
    /// While this won't compile:
    ///
    /// ```ignore
    /// # use mavio::protocol::{CompatFlags, FrameBuilder};
    /// FrameBuilder::new()
    ///     .system_id(10)
    ///     .compat_flags(CompatFlags::BIT_1);  // Won't compile
    /// ```
    pub fn compat_flags(
        self,
        compat_flags: CompatFlags,
    ) -> FrameBuilder<V2, L, Seq, S, C, M, P, E, Sig> {
        FrameBuilder {
            header_builder: self.header_builder.compat_flags(compat_flags),
            payload: self.payload,
            crc_extra: self.crc_extra,
            signature: self.signature,
        }
    }

    /// Set packet signature for `MAVLink 2` header.
    ///
    /// Setting signature manually is dangerous and may lead to creation of invalid frame. Use
    /// [`Frame::add_signature`] whenever possible. Still, we provide this method to use on your own
    /// discretion. The result is wrapped with [`Unsafe`] and marked as `#[must_use]` to give caller
    /// a hint.
    ///
    /// This method becomes available only once [`FrameBuilder::version`] is set to [`V2`].
    /// So, the following is okay:
    ///
    /// ```
    /// # use mavio::protocol::{FrameBuilder, V2, Signature};
    /// FrameBuilder::new()
    ///     .version(V2)
    ///     .signature(Signature{
    ///          // ...  
    /// #        link_id: 0,
    /// #        timestamp: Default::default(),
    /// #        value: Default::default(),
    ///     }).discard();
    /// ```
    ///
    /// While this won't compile:
    ///
    /// ```ignore
    /// # use mavio::protocol::{FrameBuilder, V2};
    /// FrameBuilder::new()
    ///     .system_id(10)
    ///     .signature(Signature{
    ///          // ...  
    /// #        link_id: 0,
    /// #        timestamp: Default::default(),
    /// #        value: Default::default(),
    ///     }); // Won't compile
    /// ```
    #[must_use]
    #[allow(clippy::type_complexity)]
    pub fn signature(
        self,
        signature: Signature,
    ) -> Unsafe<FrameBuilder<V2, L, Seq, S, C, M, P, E, HasSignature>> {
        Unsafe::new(FrameBuilder {
            header_builder: self.header_builder.signed(true),
            payload: self.payload,
            crc_extra: self.crc_extra,
            signature: HasSignature(signature),
        })
    }
}

impl<
        L: IsPayloadLen,
        Seq: IsSequenced,
        S: IsSysId,
        C: IsCompId,
        M: IsMsgId,
        P: IsPayload,
        E: IsCrcExtra,
        Sig: IsSigned,
    > FrameBuilder<Versionless, L, Seq, S, C, M, P, E, Sig>
{
    /// Updates frame builder with parameters of a MAVlink [`Endpoint`].
    ///
    /// Defines the following fields:
    ///
    /// * [`Frame::sequence`]
    /// * [`Frame::system_id`]
    /// * [`Frame::component_id`]
    /// * [`Frame::version`]
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mavio::prelude::*;
    /// use mavio::protocol::{Endpoint, MavLinkId, FrameBuilder};
    ///
    /// let device = Endpoint::new::<V2>(MavLinkId::new(1, 1));
    ///
    /// FrameBuilder::new().endpoint(&device);
    /// ```
    pub fn endpoint<V: Versioned>(
        self,
        endpoint: &Endpoint<V>,
    ) -> FrameBuilder<V, L, Sequenced, HasSysId, HasCompId, M, P, E, Sig> {
        FrameBuilder {
            header_builder: self
                .header_builder
                .version(V::v())
                .sequence(endpoint.next_sequence())
                .system_id(endpoint.system_id())
                .component_id(endpoint.component_id()),
            payload: self.payload,
            crc_extra: self.crc_extra,
            signature: self.signature,
        }
    }
}

impl<
        V: Versioned,
        L: IsPayloadLen,
        Seq: IsSequenced,
        S: IsSysId,
        C: IsCompId,
        M: IsMsgId,
        P: IsPayload,
        E: IsCrcExtra,
        Sig: IsSigned,
    > FrameBuilder<V, L, Seq, S, C, M, P, E, Sig>
{
    /// Set MAVLink message.
    ///
    /// Imports and encodes MAVLink message. Uses `crc_extra` from [`Message`] to create a
    /// checksum.
    ///
    /// Uses `message` to define:
    ///
    /// * [`Frame::message_id`]
    /// * [`Frame::payload_length`]
    /// * [`Frame::payload`]
    /// * [`Frame::checksum`]
    ///
    /// # Errors
    ///
    /// Returns [`SpecError`] if message is misconfigured or does not support previously specified
    /// [`FrameBuilder::version`].
    #[allow(clippy::type_complexity)]
    pub fn message(
        self,
        message: &dyn Message,
    ) -> Result<FrameBuilder<V, HasPayloadLen, Seq, S, C, HasMsgId, HasPayload, HasCrcExtra, Sig>>
    {
        let payload = message.encode(V::version())?;
        let crc_extra = HasCrcExtra(message.crc_extra());

        Ok(FrameBuilder {
            header_builder: self
                .header_builder
                .message_id(payload.id())
                .payload_length(payload.length()),
            payload: HasPayload(payload),
            crc_extra,
            signature: self.signature,
        })
    }
}

impl
    FrameBuilder<
        V1,
        HasPayloadLen,
        Sequenced,
        HasSysId,
        HasCompId,
        HasMsgId,
        HasPayload,
        HasCrcExtra,
        NotSigned,
    >
{
    /// Upgrades from `MAVlink 1` to `MAVLink 2` protocol version.
    ///
    /// Can be used in tandem with [`Frame::to_builder`] as a way to upgrade frames.
    pub fn upgrade(
        self,
    ) -> FrameBuilder<
        V2,
        HasPayloadLen,
        Sequenced,
        HasSysId,
        HasCompId,
        HasMsgId,
        HasPayload,
        HasCrcExtra,
        NotSigned,
    > {
        let payload = self.payload.0.upgraded();
        FrameBuilder {
            header_builder: HeaderBuilder {
                mavlink_version: PhantomData,
                payload_length: HasPayloadLen(payload.length()),
                incompat_flags: Some(IncompatFlags::default()),
                compat_flags: Some(CompatFlags::default()),
                sequence: self.header_builder.sequence,
                system_id: self.header_builder.system_id,
                component_id: self.header_builder.component_id,
                message_id: self.header_builder.message_id,
            },
            payload: HasPayload(payload),
            crc_extra: self.crc_extra,
            signature: NotSigned,
        }
    }
}

impl<V: Versioned, Sig: IsSigned>
    FrameBuilder<
        V,
        HasPayloadLen,
        Sequenced,
        HasSysId,
        HasCompId,
        HasMsgId,
        HasPayload,
        HasCrcExtra,
        Sig,
    >
{
    /// Build [`Frame`] for a specific MAVLink protocol version.
    ///
    /// If you want a frame with opaque version, use [`Frame::versionless`] from the obtained frame.
    pub fn build(self) -> Frame<V> {
        let mut frame = Frame {
            header: self.header_builder.build(),
            payload: self.payload.0,
            checksum: 0,
            signature: None,
        };

        frame.checksum = frame.calculate_crc(self.crc_extra.0);

        frame
    }
}

#[cfg(test)]
mod frame_builder_tests {
    #[test]
    #[cfg(feature = "minimal")]
    fn build_frame_v2() {
        use crate::dialects::minimal::messages::Heartbeat;
        use crate::protocol::{MavLinkVersion, V2};
        use crate::Frame;

        let message = Heartbeat::default();
        let frame = Frame::builder()
            .sequence(17)
            .system_id(22)
            .component_id(17)
            .version(V2)
            .message(&message)
            .unwrap()
            .build();

        assert!(matches!(frame.version(), MavLinkVersion::V2));
        assert_eq!(frame.sequence(), 17);
        assert_eq!(frame.system_id(), 22);
        assert_eq!(frame.component_id(), 17);
        assert_eq!(frame.message_id(), 0);
    }
}
