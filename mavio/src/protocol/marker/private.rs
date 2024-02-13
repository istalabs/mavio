pub(crate) mod sealed_traits {
    pub trait IsPayloadLen {}
    pub trait IsSequenced {}
    pub trait IsSysId {}
    pub trait IsCompId {}
    pub trait IsMsgId {}
    pub trait IsPayload {}
    pub trait IsCrcExtra {}
    pub trait IsSigned {
        fn is_signed(&self) -> bool {
            false
        }
    }
}
pub(crate) use sealed_traits::*;

mod payload_len {
    use super::IsPayloadLen;
    use crate::protocol::PayloadLength;

    #[derive(Clone, Copy, Debug, Default)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct NoPayloadLen;
    impl IsPayloadLen for NoPayloadLen {}
    #[derive(Clone, Debug, Default)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct HasPayloadLen(pub(crate) PayloadLength);
    impl IsPayloadLen for HasPayloadLen {}
}
pub use payload_len::*;

mod sequenced {
    use super::IsSequenced;
    use crate::protocol::Sequence;

    #[derive(Clone, Copy, Debug, Default)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct NotSequenced;
    impl IsSequenced for NotSequenced {}
    #[derive(Clone, Debug, Default)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct Sequenced(pub(crate) Sequence);
    impl IsSequenced for Sequenced {}
}
pub use sequenced::*;

mod sys_id {
    use super::IsSysId;
    use crate::protocol::SystemId;

    #[derive(Clone, Copy, Debug, Default)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct NoSysId;
    impl IsSysId for NoSysId {}
    #[derive(Clone, Debug, Default)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct HasSysId(pub(crate) SystemId);
    impl IsSysId for HasSysId {}
}
pub use sys_id::*;

mod comp_id {
    use super::IsCompId;
    use crate::protocol::ComponentId;

    #[derive(Clone, Copy, Debug, Default)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct NoCompId;
    impl IsCompId for NoCompId {}
    #[derive(Clone, Debug, Default)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct HasCompId(pub(crate) ComponentId);
    impl IsCompId for HasCompId {}
}
pub use comp_id::*;

mod msg_id {
    use super::IsMsgId;
    use crate::protocol::MessageId;

    #[derive(Clone, Copy, Debug, Default)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct NoMsgId;
    impl IsMsgId for NoMsgId {}
    #[derive(Clone, Debug, Default)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct HasMsgId(pub(crate) MessageId);
    impl IsMsgId for HasMsgId {}
}
pub use msg_id::*;

mod payload {
    use super::IsPayload;
    use crate::protocol::Payload;

    #[derive(Clone, Copy, Debug, Default)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct NoPayload;
    impl IsPayload for NoPayload {}
    #[derive(Clone, Debug, Default)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct HasPayload(pub(crate) Payload);
    impl IsPayload for HasPayload {}
}
pub use payload::*;

mod crc {
    use super::IsCrcExtra;
    use crate::protocol::CrcExtra;

    #[derive(Clone, Copy, Debug, Default)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct NoCrcExtra;
    impl IsCrcExtra for NoCrcExtra {}
    #[derive(Clone, Debug, Default)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct HasCrcExtra(pub(crate) CrcExtra);
    impl IsCrcExtra for HasCrcExtra {}
}
pub use crc::*;

mod signed {
    use super::IsSigned;
    use crate::protocol::Signature;

    #[derive(Clone, Copy, Debug, Default)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct NotSigned;
    impl IsSigned for NotSigned {
        fn is_signed(&self) -> bool {
            false
        }
    }
    #[derive(Clone, Debug)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct HasSignature(pub(crate) Signature);
    impl IsSigned for HasSignature {
        fn is_signed(&self) -> bool {
            true
        }
    }
}
pub use signed::*;
