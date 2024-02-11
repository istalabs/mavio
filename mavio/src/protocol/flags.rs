mod incompat_flags {
    use bitflags::bitflags;

    use crate::consts::MAVLINK_IFLAG_SIGNED;

    /// `MAVLink 2` incompatibility flags.
    ///
    /// First bit is [`MAVLINK_IFLAG_SIGNED`] as required by
    /// [specification]((https://mavlink.io/en/guide/serialization.html#incompat_flags)).
    ///
    /// Other bit flags are named according to powers of 2 in little endian.
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct IncompatFlags(u8);
    bitflags! {
        impl IncompatFlags: u8 {
            /// Signed message flag.
            const MAVLINK_IFLAG_SIGNED = MAVLINK_IFLAG_SIGNED;
            /// Second bit.
            const BIT_2 = 1 << 1;
            /// Third bit.
            const BIT_3 = 1 << 2;
            /// Fourth bit.
            const BIT_4 = 1 << 3;
            /// Fifth bit.
            const BIT_5 = 1 << 4;
            /// Sixth bit.
            const BIT_6 = 1 << 5;
            /// Seventh bit.
            const BIT_7 = 1 << 6;
            /// Eighth bit.
            const BIT_8 = 1 << 7;
        }
    }
}
pub use incompat_flags::IncompatFlags;

mod compat_flags {
    use bitflags::bitflags;

    /// `MAVLink 2` compatibility flags.
    ///
    /// Bit flags are named according to powers of 2 in little endian.
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct CompatFlags(u8);
    bitflags! {
        impl CompatFlags: u8 {
            /// Fist bit.
            const BIT_1 = 1;
            /// Second bit.
            const BIT_2 = 1 << 1;
            /// Third bit.
            const BIT_3 = 1 << 2;
            /// Fourth bit.
            const BIT_4 = 1 << 3;
            /// Fifth bit.
            const BIT_5 = 1 << 4;
            /// Sixth bit.
            const BIT_6 = 1 << 5;
            /// Seventh bit.
            const BIT_7 = 1 << 6;
            /// Eighth bit.
            const BIT_8 = 1 << 7;
        }
    }
}
pub use compat_flags::CompatFlags;
