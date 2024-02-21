use core::fmt::Debug;

use crate::consts::{STX_V1, STX_V2};
use crate::protocol::MavLinkVersion;
use crate::utils::sealed::Sealed;

use crate::prelude::*;

/// <sup>🔒</sup>
/// Marks structures which may or may not have a specified MAVLink protocol version.
///
/// ⚠ This trait is sealed ⚠
///
/// For all such structures it is possible to call [`MaybeVersioned::expect`] and
/// [`MaybeVersioned::matches`] to compare MAVLink version. The blanket implementation of
/// [`MaybeVersioned`] assumes that everything is compatible by
/// [vacuous truth](https://en.wikipedia.org/wiki/Vacuous_truth).
pub trait MaybeVersioned: IsMagicByte + Clone + Debug + Sync + Send + Sealed {
    /// Validates that provided frame matches MAVLink protocol version.
    ///
    /// The blanket implementation will always return [`Ok`] meaning that everything is compatible.
    #[inline]
    fn expect(#[allow(unused_variables)] version: MavLinkVersion) -> crate::Result<()> {
        Ok(())
    }

    /// Checks that provided version of MAVLink protocol is compatible.
    ///
    /// The blanket implementation will always return `true` meaning that everything is compatible.
    #[inline]
    fn matches(#[allow(unused_variables)] version: MavLinkVersion) -> bool {
        true
    }
}

/// Marker for entities which are not constrained by a specific MAVLink protocol version.
///
/// In the context of [`Frame`](crate::Frame) and [`Header`](crate::protocol::Header) this means
/// that although these entities are always belong to some MAVLink protocol version, this
/// information is opaque to the caller. For example, default [`Receiver`](crate::Receiver) will
/// look up for both `MAVLink 1` and `MAVLink 2` packets and return
/// [`Frame<Versionless>`](crate::Frame<Versionless>) which then can be converted to their
/// version-specific form by [`Frame::try_versioned`](crate::Frame::try_versioned).
#[derive(Clone, Copy, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Versionless;
impl Sealed for Versionless {}
impl IsMagicByte for Versionless {}

impl MaybeVersioned for Versionless {}

/// <sup>🔒</sup>
/// Marks entities which have a specified MAVLink protocol version.
///
/// ⚠ This trait is sealed ⚠
///
/// Such entities allow to discover their protocol version by [`Versioned::version`] and
/// provide a static `marker` for themselves.
///
/// For example, [`Receiver::versioned`](crate::Receiver::versioned) constructs a protocol-specific
/// receiver which looks up for frames only of a specific dialect.
pub trait Versioned: MaybeVersioned {
    /// MAVLink protocol version of an entity.
    fn version() -> MavLinkVersion;
}

/// Marks entities which are strictly `MAVLink 1` protocol compliant.
#[derive(Clone, Copy, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct V1;
impl Sealed for V1 {}
impl IsMagicByte for V1 {
    #[inline]
    fn is_magic_byte(byte: u8) -> bool {
        byte == STX_V1
    }
}
impl MaybeVersioned for V1 {
    #[inline]
    fn expect(version: MavLinkVersion) -> crate::Result<()> {
        match_error(MavLinkVersion::V1, version)
    }
    #[inline]
    fn matches(version: MavLinkVersion) -> bool {
        version == MavLinkVersion::V1
    }
}
impl Versioned for V1 {
    #[inline]
    fn version() -> MavLinkVersion {
        MavLinkVersion::V1
    }
}

/// Marks entities which are strictly `MAVLink 2` protocol compliant.
#[derive(Clone, Copy, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct V2;
impl Sealed for V2 {}
impl IsMagicByte for V2 {
    #[inline]
    fn is_magic_byte(byte: u8) -> bool {
        byte == STX_V2
    }
}
impl MaybeVersioned for V2 {
    #[inline]
    fn expect(version: MavLinkVersion) -> crate::Result<()> {
        match_error(MavLinkVersion::V2, version)
    }

    #[inline]
    fn matches(version: MavLinkVersion) -> bool {
        version == MavLinkVersion::V2
    }
}
impl Versioned for V2 {
    #[inline]
    fn version() -> MavLinkVersion {
        MavLinkVersion::V2
    }
}

#[inline]
fn match_error(expected: MavLinkVersion, actual: MavLinkVersion) -> Result<()> {
    if expected != actual {
        return Err(FrameError::InvalidVersion { expected, actual }.into());
    }
    Ok(())
}

mod is_magic_byte {
    use crate::protocol::MavSTX;

    pub trait IsMagicByte {
        #[inline]
        fn is_magic_byte(byte: u8) -> bool {
            MavSTX::is_magic_byte(byte)
        }
    }
}
pub(crate) use is_magic_byte::IsMagicByte;

#[cfg(test)]
mod version_marker_tests {
    use super::*;

    #[test]
    fn version_matching() {
        V1::expect(MavLinkVersion::V1).unwrap();
        V2::expect(MavLinkVersion::V2).unwrap();

        Versionless::expect(MavLinkVersion::V1).unwrap();
        Versionless::expect(MavLinkVersion::V2).unwrap();
        assert!(Versionless::matches(MavLinkVersion::V1));
        assert!(Versionless::matches(MavLinkVersion::V2));

        assert!(V1::matches(MavLinkVersion::V1));
        assert!(V2::matches(MavLinkVersion::V2));
        assert!(!V1::matches(MavLinkVersion::V2));
        assert!(!V2::matches(MavLinkVersion::V1));

        fn expect_versioned<V: Versioned>(_: V, version: MavLinkVersion) -> Result<()> {
            V::expect(version)
        }

        expect_versioned(V1, MavLinkVersion::V1).unwrap();
        expect_versioned(V2, MavLinkVersion::V2).unwrap();
        assert!(expect_versioned(V1, MavLinkVersion::V2).is_err());
        assert!(expect_versioned(V2, MavLinkVersion::V1).is_err());
    }
}
