//! Common generic markers.
//!
//! These markers are used to distinguish different versions of generic entities.

use crate::protocol::MavLinkVersion;

use crate::prelude::*;

/// Marks structures which may or may not have MAVLink protocol version.
///
/// For all such structures it is possible to call [`IsVersioned::matches`] to compare MAVLink
/// version. A blanket implementation always returns [`Ok`].
pub trait IsVersioned: Clone {
    /// Validates that provided frame matches MAVLink protocol version.
    ///
    /// A blanket implementation will always return [`Ok`].
    #[inline]
    #[allow(unused_variables)]
    fn matches(&self, version: MavLinkVersion) -> Result<()> {
        Ok(())
    }
}

/// Marker for entities which do not have a specific MAVLink protocol version.
#[derive(Clone, Debug, Default)]
pub struct NotVersioned();
impl IsVersioned for NotVersioned {}

/// Marks entities which have a specified MAVLink protocol version.
///
/// Such entities allow to discover their protocol version by [`Versioned::mavlink_version`].
///
/// > ⚠ Note!
/// >
/// > Since [`IsVersioned`] provides a blanket implementation for [`IsVersioned::matches`],
/// > implementor should always define such method.  
pub trait Versioned: IsVersioned {
    /// MAVLink protocol version of an entity.
    fn mavlink_version(&self) -> MavLinkVersion;
}

/// Marks entities which are strictly `MAVLink 1` protocol compliant.
#[derive(Clone, Copy, Debug, Default)]
pub struct V1();
impl IsVersioned for V1 {
    #[inline]
    fn matches(&self, version: MavLinkVersion) -> Result<()> {
        match_version(MavLinkVersion::V1, version)
    }
}
impl Versioned for V1 {
    #[inline]
    fn mavlink_version(&self) -> MavLinkVersion {
        MavLinkVersion::V1
    }
}

/// Marks entities which are strictly `MAVLink 2` protocol compliant.
#[derive(Clone, Copy, Debug, Default)]
pub struct V2();
impl IsVersioned for V2 {
    #[inline]
    fn matches(&self, version: MavLinkVersion) -> Result<()> {
        match_version(MavLinkVersion::V2, version)
    }
}
impl Versioned for V2 {
    #[inline]
    fn mavlink_version(&self) -> MavLinkVersion {
        MavLinkVersion::V2
    }
}

impl IsVersioned for MavLinkVersion {
    #[inline]
    fn matches(&self, version: MavLinkVersion) -> Result<()> {
        match_version(*self, version)
    }
}
impl Versioned for MavLinkVersion {
    #[inline]
    fn mavlink_version(&self) -> MavLinkVersion {
        *self
    }
}

#[inline]
fn match_version(expected: MavLinkVersion, actual: MavLinkVersion) -> Result<()> {
    if expected != actual {
        return Err(FrameError::InvalidVersion { expected, actual }.into());
    }
    Ok(())
}

#[cfg(test)]
mod marker_tests {
    use super::*;

    #[test]
    fn mavlink_version_is_versioned() {
        assert!(matches!(
            MavLinkVersion::V1.mavlink_version(),
            MavLinkVersion::V1
        ));
        assert!(matches!(
            MavLinkVersion::V2.mavlink_version(),
            MavLinkVersion::V2
        ));

        MavLinkVersion::V1.matches(MavLinkVersion::V1).unwrap();
        MavLinkVersion::V2.matches(MavLinkVersion::V2).unwrap();
        assert!(MavLinkVersion::V1.matches(MavLinkVersion::V2).is_err());
        assert!(MavLinkVersion::V2.matches(MavLinkVersion::V1).is_err());
    }

    #[test]
    fn version_matching() {
        V1().matches(MavLinkVersion::V1).unwrap();
        V2().matches(MavLinkVersion::V2).unwrap();

        NotVersioned().matches(MavLinkVersion::V1).unwrap();
        NotVersioned().matches(MavLinkVersion::V2).unwrap();

        fn match_version(versioned: impl Versioned, version: MavLinkVersion) -> Result<()> {
            versioned.matches(version)
        }

        match_version(V1(), MavLinkVersion::V1).unwrap();
        match_version(V2(), MavLinkVersion::V2).unwrap();
        assert!(match_version(V1(), MavLinkVersion::V2).is_err());
        assert!(match_version(V2(), MavLinkVersion::V1).is_err());
    }
}
