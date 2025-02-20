//! # Errors
//!
//! These errors used in `mavio`.
//!
//! The top-level error is [`Error`]. Library API returns versions of this error possibly wrapping
//! other types of errors.
//!
//! We also re-export errors from [`mavspec::rust::spec`](https://docs.rs/mavspec/latest/mavspec/rust/spec/)
//! to provide a full specification of MAVLink-related errors.

use crate::protocol::{IncompatFlags, MavLinkVersion, MessageId};

mod io_error;
pub use io_error::{IoError, IoErrorKind};

/// <sup>[`mavspec`](https://crates.io/crates/mavspec)</sup>
/// Errors related to MAVLink message specification.
///
/// Upon conversion into Mavio [`Error`], this error will be wrapped by [`Error::Spec`], except
/// [`SpecError::NotInDialect`], that will be converted into [`FrameError::NotInDialect`] and
/// wrapped by [`Error::Frame`].
///
/// ---
#[doc(inline)]
pub use mavspec::rust::spec::SpecError;

/// <sup>[`mavspec`](https://crates.io/crates/mavspec)</sup>
#[cfg(feature = "msrv-utils-mission")]
#[doc(inline)]
pub use mavspec::rust::microservices::mission::MissionError;

/// Common result type returned by `mavio` functions and methods.
pub type Result<T> = core::result::Result<T, Error>;

/// `mavio` top-level error.
///
/// Returned by most of the functions and methods across `mavio`. Other errors are either
/// converted to this error or wrapped by its variants.
///
/// ## Caveats
///
/// We provide a limited support for [Serde](https://serde.rs) and
/// [Specta](https://crates.io/crates/specta). At the moment, the goal is simply to show something
/// meaningful. See [`IoError`] for details.
///
/// This part of the API is considered unstable. Breaking changes may be introduced at any time.
///
/// To enable Serde or Specta for [`Error`], turn the `unstable` feature on.
///
/// ### Specta
///
/// The name of the exported type is `MavioError`.
#[derive(Clone, Debug)]
#[cfg_attr(all(feature = "specta", feature = "unstable"), derive(specta::Type))]
#[cfg_attr(
    all(feature = "specta", feature = "unstable"),
    specta(rename = "MavioError")
)]
#[cfg_attr(
    all(feature = "serde", feature = "unstable"),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(feature = "std", derive(thiserror::Error))]
pub enum Error {
    /// I/O error wrapper. Check [`IoError`] for details.
    #[cfg_attr(feature = "std", error("I/O error: {0:?}"))]
    Io(IoError),

    /// Frame validation errors.
    #[cfg_attr(feature = "std", error("frame decoding/encoding error: {0:?}"))]
    Frame(FrameError),

    /// MAVLink specification errors. A wrapper for [`SpecError`] re-exported from MAVSpec.
    #[cfg_attr(feature = "std", error("frame decoding/encoding error: {0:?}"))]
    Spec(SpecError),

    /// <sup>`⍚`</sup> MAVLink mission-related errors.
    #[cfg_attr(feature = "std", error("mission error: {0:?}"))]
    #[cfg(feature = "msrv-utils-mission")]
    Mission(MissionError),
}

/// Errors related to MAVLink frame validation.
///
/// This means, that frame is already present but hasn't passed certain criteria.
#[derive(Clone, Debug)]
#[cfg_attr(all(feature = "specta", feature = "unstable"), derive(specta::Type))]
#[cfg_attr(
    all(feature = "serde", feature = "unstable"),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(feature = "std", derive(thiserror::Error))]
pub enum FrameError {
    /// Incorrect MAVLink version.
    #[cfg_attr(feature = "std", error("invalid MAVLink version: {0:?}"))]
    Version(VersionError),
    /// Upon calculation, CRC does not match received [Frame::checksum](crate::Frame::checksum).
    #[cfg_attr(feature = "std", error("checksum validation failed"))]
    Checksum,
    /// Upon validation, the [Frame::signature](crate::Frame::signature) found to be incorrect.
    #[cfg_attr(feature = "std", error("signature validation failed"))]
    Signature,
    /// Upon validation, the [Frame::incompat_flags](crate::Frame::incompat_flags) do not match the
    /// required flag set.
    #[cfg_attr(feature = "std", error("invalid incompat flags: {0:?}"))]
    Incompatible(IncompatFlagsError),
    /// MAVLink message with specified ID is not in dialect.
    #[cfg_attr(feature = "std", error("message with ID {0:?} is not in dialect"))]
    NotInDialect(MessageId),
}

/// Invalid MAVLink version.
///
/// Can be converted to [`FrameError::Version`].
#[derive(Copy, Clone, Debug)]
#[cfg_attr(all(feature = "specta", feature = "unstable"), derive(specta::Type))]
#[cfg_attr(
    all(feature = "serde", feature = "unstable"),
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct VersionError {
    /// Expected protocol version.
    pub expected: MavLinkVersion,
    /// Actual protocol version.
    pub actual: MavLinkVersion,
}

/// Invalid frame checksum.
///
/// Can be converted to [`FrameError::Checksum`].
pub struct ChecksumError;

/// Invalid frame signature.
///
/// Can be converted to [`FrameError::Signature`].
pub struct SignatureError;

/// Invalid incompatibility flags.
///
/// Can be converted to [`FrameError::Incompatible`].
#[derive(Copy, Clone, Debug)]
#[cfg_attr(all(feature = "specta", feature = "unstable"), derive(specta::Type))]
#[cfg_attr(
    all(feature = "serde", feature = "unstable"),
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct IncompatFlagsError {
    /// Expected flag set.
    pub expected: IncompatFlags,
    /// Actual flag set.
    pub actual: IncompatFlags,
}

impl From<VersionError> for FrameError {
    /// Converts [`VersionError`] into [`FrameError::Version`].
    #[inline(always)]
    fn from(value: VersionError) -> Self {
        Self::Version(value)
    }
}

impl From<VersionError> for Error {
    /// Converts [`VersionError`] into [`FrameError::Version`]  variant of [`Error::Frame`].
    #[inline(always)]
    fn from(value: VersionError) -> Self {
        FrameError::from(value).into()
    }
}

impl From<ChecksumError> for FrameError {
    /// Converts [`ChecksumError`] into [`FrameError::Checksum`].
    #[inline(always)]
    fn from(_: ChecksumError) -> Self {
        Self::Checksum
    }
}

impl From<ChecksumError> for Error {
    /// Converts [`ChecksumError`] into [`FrameError::Checksum`] variant of [`Error::Frame`].
    #[inline(always)]
    fn from(value: ChecksumError) -> Self {
        FrameError::from(value).into()
    }
}

impl From<SignatureError> for FrameError {
    /// Converts [`SignatureError`] into [`FrameError::Signature`].
    #[inline(always)]
    fn from(_: SignatureError) -> Self {
        Self::Signature
    }
}

impl From<SignatureError> for Error {
    /// Converts [`SignatureError`] into [`FrameError::Signature`] variant of [`Error::Frame`].
    #[inline(always)]
    fn from(value: SignatureError) -> Self {
        FrameError::from(value).into()
    }
}

impl From<IncompatFlagsError> for FrameError {
    /// Converts [`IncompatFlagsError`] into [`FrameError::Incompatible`].
    #[inline(always)]
    fn from(value: IncompatFlagsError) -> Self {
        Self::Incompatible(value)
    }
}

impl From<IncompatFlagsError> for Error {
    /// Converts [`IncompatFlagsError`] into [`FrameError::Incompatible`] variant of [`Error::Frame`].
    #[inline(always)]
    fn from(value: IncompatFlagsError) -> Self {
        FrameError::from(value).into()
    }
}

impl From<FrameError> for Error {
    /// Converts [`FrameError`] into [`Error::Frame`].
    #[inline(always)]
    fn from(value: FrameError) -> Self {
        Self::Frame(value)
    }
}

impl From<SpecError> for Error {
    /// Converts [`SpecError`] into [`Error::Spec`].
    ///
    /// There is a special case for [`SpecError::NotInDialect`], that will be converted to
    /// [`FrameError::NotInDialect`] variant of [`Error::Frame`].
    fn from(value: SpecError) -> Self {
        if let SpecError::NotInDialect(id) = value {
            Error::Frame(FrameError::NotInDialect(id))
        } else {
            Self::Spec(value)
        }
    }
}

#[cfg(feature = "msrv-utils-mission")]
impl From<MissionError> for Error {
    /// Converts [`MissionError`] into [`Error::Mission`] variant of [`Error`].
    #[inline(always)]
    fn from(value: MissionError) -> Self {
        Error::Mission(value)
    }
}
