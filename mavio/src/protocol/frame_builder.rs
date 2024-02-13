use core::marker::PhantomData;

use crate::protocol::marker::{
    HasCompId, HasCrcExtra, HasMsgId, HasPayload, HasPayloadLen, HasSignature, HasSysId, IsCompId,
    IsCrcExtra, IsMsgId, IsPayload, IsPayloadLen, IsSequenced, IsSigned, IsSysId, NoCompId,
    NoCrcExtra, NoMsgId, NoPayload, NoPayloadLen, NoSysId, NotSequenced, NotSigned, Sequenced,
};
use crate::protocol::{
    CompatFlags, ComponentId, CrcExtra, DialectImpl, DialectMessage, Dialectless, HasDialect,
    HeaderBuilder, IncompatFlags, MaybeDialect, MaybeVersioned, MessageId, MessageImpl, Payload,
    Sequence, Signature, SystemId, Versioned, Versionless, V1, V2,
};
use crate::Frame;

use crate::prelude::*;

/// Builder for [`Frame`].
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FrameBuilder<
    V: MaybeVersioned,
    D: MaybeDialect,
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
    pub(super) dialect: D,
}

impl Default
    for FrameBuilder<
        Versionless,
        Dialectless,
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
        Dialectless,
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
            dialect: Dialectless,
        }
    }
}

impl<
        V: MaybeVersioned,
        D: MaybeDialect,
        L: IsPayloadLen,
        Seq: IsSequenced,
        S: IsSysId,
        C: IsCompId,
        M: IsMsgId,
        P: IsPayload,
        E: IsCrcExtra,
        Sig: IsSigned,
    > FrameBuilder<V, D, L, Seq, S, C, M, P, E, Sig>
{
    /// Set packet sequence number.
    ///
    /// See: [`Frame::sequence`].
    pub fn sequence(
        self,
        sequence: Sequence,
    ) -> FrameBuilder<V, D, L, Sequenced, S, C, M, P, E, Sig> {
        FrameBuilder {
            header_builder: self.header_builder.sequence(sequence),
            payload: self.payload,
            crc_extra: self.crc_extra,
            signature: self.signature,
            dialect: self.dialect,
        }
    }

    /// Set system `ID`.
    ///
    /// See: [`Frame::system_id`].
    pub fn system_id(
        self,
        system_id: SystemId,
    ) -> FrameBuilder<V, D, L, Seq, HasSysId, C, M, P, E, Sig> {
        FrameBuilder {
            header_builder: self.header_builder.system_id(system_id),
            payload: self.payload,
            crc_extra: self.crc_extra,
            signature: self.signature,
            dialect: self.dialect,
        }
    }

    /// Set component `ID`.
    ///
    /// See: [`Frame::component_id`].
    pub fn component_id(
        self,
        component_id: ComponentId,
    ) -> FrameBuilder<V, D, L, Seq, S, HasCompId, M, P, E, Sig> {
        FrameBuilder {
            header_builder: self.header_builder.component_id(component_id),
            payload: self.payload,
            crc_extra: self.crc_extra,
            signature: self.signature,
            dialect: self.dialect,
        }
    }

    /// Set payload data.
    ///
    /// Also sets [`Frame::message_id`] from [`Payload::id`] and [`Frame::payload_length`] .
    ///
    /// # Links
    ///
    /// * [`Frame::payload`]
    /// * [`Frame::message_id`]
    pub fn payload(
        self,
        payload: Payload,
    ) -> FrameBuilder<V, D, HasPayloadLen, Seq, S, C, HasMsgId, HasPayload, E, Sig> {
        FrameBuilder {
            header_builder: self
                .header_builder
                .message_id(payload.id())
                .payload_length(payload.length()),
            payload: HasPayload(payload),
            crc_extra: self.crc_extra,
            signature: self.signature,
            dialect: self.dialect,
        }
    }

    /// Set `CRC_EXTRA`.
    ///
    /// # Links
    ///
    /// * [`Frame::checksum`] is calculated using [`CrcExtra`].
    pub fn crc_extra(
        self,
        crc_extra: CrcExtra,
    ) -> FrameBuilder<V, D, L, Seq, S, C, M, P, HasCrcExtra, Sig> {
        FrameBuilder {
            header_builder: self.header_builder,
            payload: self.payload,
            crc_extra: HasCrcExtra(crc_extra),
            signature: self.signature,
            dialect: self.dialect,
        }
    }

    /// Set dialect.
    ///
    /// This dialect can be used with [`Frame::decode`].
    pub fn dialect<Message: DialectMessage + 'static>(
        self,
        dialect: &'static dyn DialectImpl<Message = Message>,
    ) -> FrameBuilder<V, HasDialect<Message>, L, Seq, S, C, M, P, E, Sig> {
        FrameBuilder {
            header_builder: self.header_builder,
            payload: self.payload,
            crc_extra: self.crc_extra,
            signature: self.signature,
            dialect: HasDialect(dialect),
        }
    }

    fn message_id(
        self,
        message_id: MessageId,
    ) -> FrameBuilder<V, D, L, Seq, S, C, HasMsgId, P, E, Sig> {
        FrameBuilder {
            header_builder: self.header_builder.message_id(message_id),
            payload: self.payload,
            crc_extra: self.crc_extra,
            signature: self.signature,
            dialect: self.dialect,
        }
    }
}

impl<
        V: Versioned,
        D: MaybeDialect,
        L: IsPayloadLen,
        Seq: IsSequenced,
        S: IsSysId,
        C: IsCompId,
        M: IsMsgId,
        P: IsPayload,
        E: IsCrcExtra,
        Sig: IsSigned,
    > FrameBuilder<V, D, L, Seq, S, C, M, P, E, Sig>
{
    /// Set MAVLink message.
    ///
    /// Imports and encodes MAVLink message. Uses `crc_extra` from [`MessageImpl`] to create a
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
    /// Returns [`MessageError`] if
    /// message is misconfigured or does not support provided `mavlink_version`.
    #[allow(clippy::type_complexity)]
    pub fn message(
        self,
        message: &dyn MessageImpl,
    ) -> Result<FrameBuilder<V, D, HasPayloadLen, Seq, S, C, HasMsgId, HasPayload, HasCrcExtra, Sig>>
    {
        let payload = message.encode(V::version())?;

        Ok(self
            .message_id(message.id())
            .payload(payload)
            .crc_extra(message.crc_extra()))
    }
}

impl<
        D: MaybeDialect,
        L: IsPayloadLen,
        Seq: IsSequenced,
        S: IsSysId,
        C: IsCompId,
        M: IsMsgId,
        P: IsPayload,
        E: IsCrcExtra,
        Sig: IsSigned,
    > FrameBuilder<Versionless, D, L, Seq, S, C, M, P, E, Sig>
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
    ///     .mavlink_version(V1)
    ///     .mavlink_version(V2); // can't set MAVLink version twice!
    /// ```
    pub fn version<Version: Versioned>(
        self,
        version: Version,
    ) -> FrameBuilder<Version, D, L, Seq, S, C, M, P, E, Sig> {
        FrameBuilder {
            header_builder: self.header_builder.version(version),
            payload: self.payload,
            crc_extra: self.crc_extra,
            signature: self.signature,
            dialect: self.dialect,
        }
    }
}

impl<
        D: MaybeDialect,
        L: IsPayloadLen,
        Seq: IsSequenced,
        S: IsSysId,
        C: IsCompId,
        M: IsMsgId,
        P: IsPayload,
        E: IsCrcExtra,
        Sig: IsSigned,
    > FrameBuilder<V2, D, L, Seq, S, C, M, P, E, Sig>
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
    ) -> FrameBuilder<V2, D, L, Seq, S, C, M, P, E, Sig> {
        FrameBuilder {
            header_builder: self
                .header_builder
                .incompat_flags(incompat_flags)
                .signed(self.signature.is_signed()),
            payload: self.payload,
            crc_extra: self.crc_extra,
            signature: self.signature,
            dialect: self.dialect,
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
    ) -> FrameBuilder<V2, D, L, Seq, S, C, M, P, E, Sig> {
        FrameBuilder {
            header_builder: self.header_builder.compat_flags(compat_flags),
            payload: self.payload,
            crc_extra: self.crc_extra,
            signature: self.signature,
            dialect: self.dialect,
        }
    }

    /// Set packet signature for `MAVLink 2` header.
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
    ///     });
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
    pub fn signature(
        self,
        signature: Signature,
    ) -> FrameBuilder<V2, D, L, Seq, S, C, M, P, E, HasSignature> {
        FrameBuilder {
            header_builder: self.header_builder.signed(true),
            payload: self.payload,
            crc_extra: self.crc_extra,
            signature: HasSignature(signature),
            dialect: self.dialect,
        }
    }
}

impl<D: MaybeDialect>
    FrameBuilder<
        V1,
        D,
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
        D,
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
            dialect: self.dialect,
        }
    }
}

impl<V: Versioned, D: MaybeDialect, Sig: IsSigned>
    FrameBuilder<
        V,
        D,
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
    pub fn build(self) -> Frame<V, D> {
        let mut frame = Frame {
            header: self.header_builder.build(),
            payload: self.payload.0,
            checksum: 0,
            signature: None,
            dialect: self.dialect,
        };

        frame.checksum = frame.calculate_crc(self.crc_extra.0);

        frame
    }

    /// Build a versionless [`Frame`].
    ///
    /// Versionless frames can be exchanged between protocol-agnostic channels. Internally, frames
    /// still possess a capability to encode and decode themselves.
    pub fn versionless(self) -> Frame<Versionless, D> {
        self.build().versionless()
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

    #[test]
    #[cfg(feature = "minimal")]
    fn build_frame_v2_with_dialect() {
        use crate::dialects::minimal as dialect;
        use crate::protocol::V2;
        use crate::Frame;
        use dialect::messages::Heartbeat;
        use dialect::Minimal;

        let message = Heartbeat::default();
        let frame = Frame::builder()
            .sequence(17)
            .system_id(22)
            .component_id(17)
            .version(V2)
            .message(&message)
            .unwrap()
            .dialect(dialect::dialect())
            .build();

        let message = frame.to_message().unwrap();
        assert!(matches!(message, Minimal::Heartbeat(_)));
    }
}
