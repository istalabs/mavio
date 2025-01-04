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
    use crate::protocol::{PayloadLength, Unset};

    impl IsPayloadLen for Unset {}
    #[derive(Clone, Debug, Default)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct HasPayloadLen(pub(crate) PayloadLength);
    impl IsPayloadLen for HasPayloadLen {}
}
pub use payload_len::*;

mod sequenced {
    use super::IsSequenced;
    use crate::protocol::{Sequence, Unset};

    impl IsSequenced for Unset {}
    #[derive(Clone, Debug, Default)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct Sequenced(pub(crate) Sequence);
    impl IsSequenced for Sequenced {}
}
pub use sequenced::*;

mod sys_id {
    use super::IsSysId;
    use crate::protocol::{SystemId, Unset};

    impl IsSysId for Unset {}
    #[derive(Clone, Debug, Default)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct HasSysId(pub(crate) SystemId);
    impl IsSysId for HasSysId {}
}
pub use sys_id::*;

mod comp_id {
    use super::IsCompId;
    use crate::protocol::{ComponentId, Unset};

    impl IsCompId for Unset {}
    #[derive(Clone, Debug, Default)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct HasCompId(pub(crate) ComponentId);
    impl IsCompId for HasCompId {}
}
pub use comp_id::*;

mod msg_id {
    use super::IsMsgId;
    use crate::protocol::{MessageId, Unset};

    impl IsMsgId for Unset {}
    #[derive(Clone, Debug, Default)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct HasMsgId(pub(crate) MessageId);
    impl IsMsgId for HasMsgId {}
}
pub use msg_id::*;

mod payload {
    use super::IsPayload;
    use crate::protocol::{Payload, Unset};

    impl IsPayload for Unset {}
    #[derive(Clone, Debug, Default)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct HasPayload(pub(crate) Payload);
    impl IsPayload for HasPayload {}
}
pub use payload::*;

mod crc {
    use super::IsCrcExtra;
    use crate::protocol::{CrcExtra, Unset};

    impl IsCrcExtra for Unset {}
    #[derive(Clone, Debug, Default)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct HasCrcExtra(pub(crate) CrcExtra);
    impl IsCrcExtra for HasCrcExtra {}
}
pub use crc::*;

mod signed {
    use super::IsSigned;
    use crate::protocol::{Signature, Unset};

    impl IsSigned for Unset {
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
