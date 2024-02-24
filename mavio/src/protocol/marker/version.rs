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
    fn expect(#[allow(unused_variables)] version: MavLinkVersion) -> Result<()> {
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
/// In the context of [`Frame`](Frame) and [`Header`](crate::protocol::Header) this means
/// that although these entities are always belong to some MAVLink protocol version, this
/// information is opaque to the caller. For example, default [`Receiver`](crate::Receiver) will
/// look up for both `MAVLink 1` and `MAVLink 2` packets and return
/// [`Frame<Versionless>`](Frame<Versionless>) which then can be converted to their
/// version-specific form by [`Frame::try_versioned`](Frame::try_versioned).
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
/// provide a static `marker` for themselves. This trait also enables converting [`V1`] / [`V2`]
/// from type parameter to a type by [`Versioned::v`] and to treat them as unit type instances using
/// [`Versioned::ver`].
///
/// For example, [`Receiver::versioned`](crate::Receiver::versioned) constructs a protocol-specific
/// receiver which looks up for frames only of a specific dialect.
///
/// # Examples
///
/// ```rust
/// use mavio::prelude::*;
///
/// fn release_turbofish<V: Versioned>() {
///     pass_argument(V::v());
///     stay_with_enum(V::v().ver());
/// }
///
/// fn pass_argument<V: Versioned>(version: V) {
///     stay_with_enum(version.ver());
/// }
///
/// fn stay_with_enum(version: MavLinkVersion) {
///     match version {
///         MavLinkVersion::V1 => { /* MAVLink1 specific */ }
///         MavLinkVersion::V2 => { /* MAVLink2 specific */ }
///     }
/// }
///
/// release_turbofish::<V2>();
/// pass_argument(V1);
/// stay_with_enum(MavLinkVersion::V2);
/// ```
pub trait Versioned: MaybeVersioned {
    /// MAVLink protocol version of a type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mavio::prelude::*;
    ///
    /// fn feed_with_enum(version: MavLinkVersion) {
    ///     match version {
    ///         MavLinkVersion::V1 => { /* MAVLink1 specific */ }
    ///         MavLinkVersion::V2 => { /* MAVLink2 specific */ }
    ///     }
    /// }
    ///
    /// feed_with_enum(V1::version());
    /// ```
    fn version() -> MavLinkVersion;

    /// MAVLink protocol version of a unit.
    ///
    /// Allows to obtain [`MavLinkVersion`] from [`V1`] / [`V2`] as unit types.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mavio::prelude::*;
    ///
    /// fn feed_with_argument<V: Versioned>(version: V) {
    /// #   return;
    ///     feed_with_enum(version.ver());
    /// }
    ///
    /// fn feed_with_enum(version: MavLinkVersion) {
    ///     match version {
    ///         MavLinkVersion::V1 => { /* MAVLink1 specific */ }
    ///         MavLinkVersion::V2 => { /* MAVLink1 specific */ }
    ///     }
    /// }
    ///
    /// feed_with_argument(V1);
    /// ```
    fn ver(&self) -> MavLinkVersion;

    /// Returns MAVLink version as [`V1`] / [`V2`] for type parameters.
    ///
    /// Useful when [`Versioned`] is provided as a type parameter, but you need a corresponding
    /// marker. This allows to switch between regular arguments and [turbofish](https://turbo.fish/)
    /// syntax.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mavio::prelude::*;
    ///
    /// fn gimme_argument<V: Versioned>(version: V) {
    /// #   return;
    ///     gimme_turbofish::<V>();
    /// }
    ///
    /// fn gimme_turbofish<V: Versioned>() {
    /// #   return;
    ///     // Hard way
    ///     match V::version() {
    ///         MavLinkVersion::V1 => gimme_argument(V1),
    ///         MavLinkVersion::V2 => gimme_argument(V2),
    ///     }
    ///     // Easy way
    ///     gimme_argument(V::v());
    /// }
    ///
    /// gimme_argument(V2);
    /// gimme_turbofish::<V2>()
    /// ```
    fn v() -> Self;
}

/// Marks entities which are strictly `MAVLink 1` protocol compliant.
#[derive(Clone, Copy, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct V1;
impl Sealed for V1 {}
impl IsMagicByte for V1 {
    #[inline(always)]
    fn is_magic_byte(byte: u8) -> bool {
        byte == STX_V1
    }
}
impl MaybeVersioned for V1 {
    #[inline]
    fn expect(version: MavLinkVersion) -> Result<()> {
        match_error(MavLinkVersion::V1, version)
    }
    #[inline(always)]
    fn matches(version: MavLinkVersion) -> bool {
        version == MavLinkVersion::V1
    }
}
impl Versioned for V1 {
    #[inline(always)]
    fn version() -> MavLinkVersion {
        MavLinkVersion::V1
    }

    fn ver(&self) -> MavLinkVersion {
        MavLinkVersion::V1
    }

    #[inline(always)]
    fn v() -> Self {
        V1
    }
}

/// Marks entities which are strictly `MAVLink 2` protocol compliant.
#[derive(Clone, Copy, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct V2;
impl Sealed for V2 {}
impl IsMagicByte for V2 {
    #[inline(always)]
    fn is_magic_byte(byte: u8) -> bool {
        byte == STX_V2
    }
}
impl MaybeVersioned for V2 {
    #[inline]
    fn expect(version: MavLinkVersion) -> Result<()> {
        match_error(MavLinkVersion::V2, version)
    }

    #[inline(always)]
    fn matches(version: MavLinkVersion) -> bool {
        version == MavLinkVersion::V2
    }
}
impl Versioned for V2 {
    #[inline(always)]
    fn version() -> MavLinkVersion {
        MavLinkVersion::V2
    }

    #[inline(always)]
    fn ver(&self) -> MavLinkVersion {
        MavLinkVersion::V2
    }

    #[inline(always)]
    fn v() -> Self {
        V2
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
