use core::marker::PhantomData;

use crate::protocol::header::Header;
use crate::protocol::marker::{
    HasCompId, HasMsgId, HasPayloadLen, HasSysId, IsCompId, IsMsgId, IsPayloadLen, IsSequenced,
    IsSysId, NoCompId, NoMsgId, NoPayloadLen, NoSysId, NotSequenced, Sequenced,
};
use crate::protocol::{
    CompatFlags, IncompatFlags, MavLinkVersion, MaybeVersioned, MessageId, PayloadLength, Sequence,
    SystemId, Versioned, Versionless, V1, V2,
};

/// Builder for [`Header`].
///
/// Implements [builder](https://rust-unofficial.github.io/patterns/patterns/creational/builder.html)
/// pattern for [`Header`].
#[derive(Clone, Copy, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HeaderBuilder<
    V: MaybeVersioned,
    L: IsPayloadLen,
    Seq: IsSequenced,
    S: IsSysId,
    C: IsCompId,
    M: IsMsgId,
> {
    pub(super) mavlink_version: PhantomData<V>,
    pub(super) payload_length: L,
    pub(super) incompat_flags: Option<IncompatFlags>,
    pub(super) compat_flags: Option<CompatFlags>,
    pub(super) sequence: Seq,
    pub(super) system_id: S,
    pub(super) component_id: C,
    pub(super) message_id: M,
}

impl HeaderBuilder<Versionless, NoPayloadLen, NotSequenced, NoSysId, NoCompId, NoMsgId> {
    /// Default constructor.
    pub fn new() -> Self {
        Self {
            mavlink_version: PhantomData,
            payload_length: NoPayloadLen,
            incompat_flags: None,
            compat_flags: None,
            sequence: NotSequenced,
            system_id: NoSysId,
            component_id: NoCompId,
            message_id: NoMsgId,
        }
    }
}

impl<V: MaybeVersioned, L: IsPayloadLen, Seq: IsSequenced, S: IsSysId, C: IsCompId, M: IsMsgId>
    HeaderBuilder<V, L, Seq, S, C, M>
{
    /// Set MAVLink protocol version.
    ///
    /// See: [`Header::payload_length`].
    pub fn version<Version: Versioned>(
        self,
        #[allow(unused_variables)] version: Version,
    ) -> HeaderBuilder<Version, L, Seq, S, C, M> {
        let from_v1_to_v2 =
            Version::matches(MavLinkVersion::V1) && Version::version() == MavLinkVersion::V2;

        let mut header = HeaderBuilder {
            mavlink_version: PhantomData,
            payload_length: self.payload_length,
            incompat_flags: self.incompat_flags,
            compat_flags: self.compat_flags,
            sequence: self.sequence,
            system_id: self.system_id,
            component_id: self.component_id,
            message_id: self.message_id,
        };

        if Version::matches(MavLinkVersion::V1) {
            header.incompat_flags = None;
            header.compat_flags = None;
        }

        if from_v1_to_v2 {
            header.incompat_flags = Some(header.incompat_flags.unwrap_or_default());
            header.compat_flags = Some(header.compat_flags.unwrap_or_default());
        }

        header
    }

    /// Set payload length.
    ///
    /// See: [`Header::payload_length`].
    pub fn payload_length(
        self,
        payload_length: PayloadLength,
    ) -> HeaderBuilder<V, HasPayloadLen, Seq, S, C, M> {
        HeaderBuilder {
            mavlink_version: self.mavlink_version,
            payload_length: HasPayloadLen(payload_length),
            incompat_flags: self.incompat_flags,
            compat_flags: self.compat_flags,
            sequence: self.sequence,
            system_id: self.system_id,
            component_id: self.component_id,
            message_id: self.message_id,
        }
    }

    /// Set incompatibility flags for `MAVLink 2` header.
    ///
    /// Calling this method will force MAVLink protocol version to [`V2`].
    pub fn incompat_flags(
        self,
        incompat_flags: IncompatFlags,
    ) -> HeaderBuilder<V2, L, Seq, S, C, M> {
        let this = self.version(V2);

        HeaderBuilder {
            incompat_flags: Some(incompat_flags),
            ..this
        }
    }

    /// Set compatibility flags for `MAVLink 2` header.
    ///
    /// Calling this method will force MAVLink protocol version to [`V2`].
    pub fn compat_flags(self, compat_flags: CompatFlags) -> HeaderBuilder<V2, L, Seq, S, C, M> {
        let this = self.version(V2);

        HeaderBuilder {
            compat_flags: Some(compat_flags),
            ..this
        }
    }

    /// Set packet sequence number.
    ///
    /// See: [`Header::sequence`].
    pub fn sequence(self, sequencer: Sequence) -> HeaderBuilder<V, L, Sequenced, S, C, M> {
        HeaderBuilder {
            mavlink_version: self.mavlink_version,
            payload_length: self.payload_length,
            incompat_flags: self.incompat_flags,
            compat_flags: self.compat_flags,
            sequence: Sequenced(sequencer),
            system_id: self.system_id,
            component_id: self.component_id,
            message_id: self.message_id,
        }
    }

    /// Sets system `ID`.
    ///
    /// See: [`Header::system_id`].
    pub fn system_id(self, system_id: SystemId) -> HeaderBuilder<V, L, Seq, HasSysId, C, M> {
        HeaderBuilder {
            mavlink_version: self.mavlink_version,
            payload_length: self.payload_length,
            incompat_flags: self.incompat_flags,
            compat_flags: self.compat_flags,
            sequence: self.sequence,
            system_id: HasSysId(system_id),
            component_id: self.component_id,
            message_id: self.message_id,
        }
    }

    /// Set component `ID`.
    ///
    /// See: [`Header::component_id`].
    pub fn component_id(self, component_id: SystemId) -> HeaderBuilder<V, L, Seq, S, HasCompId, M> {
        HeaderBuilder {
            mavlink_version: self.mavlink_version,
            payload_length: self.payload_length,
            incompat_flags: self.incompat_flags,
            compat_flags: self.compat_flags,
            sequence: self.sequence,
            system_id: self.system_id,
            component_id: HasCompId(component_id),
            message_id: self.message_id,
        }
    }

    /// Set message `ID`.
    ///
    /// See: [`Header::message_id`].
    pub fn message_id(self, message_id: MessageId) -> HeaderBuilder<V, L, Seq, S, C, HasMsgId> {
        HeaderBuilder {
            mavlink_version: self.mavlink_version,
            payload_length: self.payload_length,
            incompat_flags: self.incompat_flags,
            compat_flags: self.compat_flags,
            sequence: self.sequence,
            system_id: self.system_id,
            component_id: self.component_id,
            message_id: HasMsgId(message_id),
        }
    }

    fn set_is_signed_no_v2(self, flag: bool) -> HeaderBuilder<V2, L, Seq, S, C, M> {
        let mut flags = IncompatFlags::default();
        flags.set(IncompatFlags::MAVLINK_IFLAG_SIGNED, flag);

        HeaderBuilder {
            incompat_flags: Some(flags),
            compat_flags: Some(CompatFlags::default()),
            ..self.version(V2)
        }
    }
}

impl<L: IsPayloadLen, Seq: IsSequenced, S: IsSysId, C: IsCompId, M: IsMsgId>
    HeaderBuilder<Versionless, L, Seq, S, C, M>
{
    /// Set whether `MAVLink 2` frame body should contain signature.
    ///
    /// Sets or drops [`IncompatFlags::MAVLINK_IFLAG_SIGNED`] flag for
    /// [`incompat_flags`](Header::incompat_flags).
    ///
    /// Sets MAVLink protocol version to [`V2`].
    pub fn signed(self, flag: bool) -> HeaderBuilder<V2, L, Seq, S, C, M> {
        self.set_is_signed_no_v2(flag)
    }
}

impl<L: IsPayloadLen, Seq: IsSequenced, S: IsSysId, C: IsCompId, M: IsMsgId>
    HeaderBuilder<V1, L, Seq, S, C, M>
{
    /// Set whether `MAVLink 2` frame body should contain signature.
    ///
    /// Sets or drops [`IncompatFlags::MAVLINK_IFLAG_SIGNED`] flag for
    /// [`incompat_flags`](Header::incompat_flags).
    ///
    /// Sets MAVLink protocol version to [`V2`].
    pub fn signed(self, flag: bool) -> HeaderBuilder<V2, L, Seq, S, C, M> {
        self.set_is_signed_no_v2(flag)
    }
}

impl<L: IsPayloadLen, Seq: IsSequenced, S: IsSysId, C: IsCompId, M: IsMsgId>
    HeaderBuilder<V2, L, Seq, S, C, M>
{
    /// Sets whether `MAVLink 2` frame body should contain signature.
    ///
    /// Sets [`IncompatFlags::MAVLINK_IFLAG_SIGNED`] flag for
    /// [`incompat_flags`](Header::incompat_flags).
    ///
    /// Sets MAVLink protocol version to [`V2`].
    pub fn signed(self, flag: bool) -> HeaderBuilder<V2, L, Seq, S, C, M> {
        let this = self.version(V2);
        HeaderBuilder {
            incompat_flags: this.incompat_flags.map(|flags| {
                flags.clone().set(IncompatFlags::MAVLINK_IFLAG_SIGNED, flag);
                flags
            }),
            ..this
        }
    }
}

impl<V: Versioned> HeaderBuilder<V, HasPayloadLen, Sequenced, HasSysId, HasCompId, HasMsgId> {
    /// Build [`Header`] for a specific MAVLink protocol version.
    pub fn build(&self) -> Header<V> {
        Header {
            version: V::version(),
            payload_length: self.payload_length.0,
            incompat_flags: self.incompat_flags.unwrap_or_default(),
            compat_flags: self.compat_flags.unwrap_or_default(),
            sequence: self.sequence.0,
            system_id: self.system_id.0,
            component_id: self.component_id.0,
            message_id: self.message_id.0,
            _marker_version: PhantomData,
        }
    }
}
