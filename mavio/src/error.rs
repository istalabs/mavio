//! # Errors
//!
//! These errors used in `mavio`.
//!
//! The top-level error is [`Error`]. Library API returns versions of this error possibly wrapping
//! other types of errors.
//!
//! We also re-export errors from [`mavspec::rust::spec`](https://docs.rs/mavspec/latest/mavspec/rust/spec/)
//! to provide a full specification of MAVLink-related errors.

#[cfg(feature = "std")]
use std::sync::Arc;

use crate::protocol::{IncompatFlags, MavLinkVersion, MessageId};

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

/// Common result type returned by `mavio` functions and methods.
pub type Result<T> = core::result::Result<T, Error>;

/// `mavio` top-level error.
///
/// Returned by most of the functions and methods across `mavio`. Other errors are either
/// converted to this error or wrapped by its variants.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "std", derive(thiserror::Error))]
pub enum Error {
    /// [`std::io::Error`] wrapper.
    #[cfg(feature = "std")]
    #[cfg_attr(feature = "std", error("I/O error: {0:?}"))]
    Io(Arc<std::io::Error>),

    /// `no_std` I/O error.
    ///
    /// Wraps [`IoError`](crate::io::no_std::IoError).
    #[cfg(not(feature = "std"))]
    Io(crate::io::no_std::IoError),

    /// Frame validation errors.
    #[cfg_attr(feature = "std", error("frame decoding/encoding error: {0:?}"))]
    Frame(FrameError),

    /// MAVLink specification errors. A wrapper for [`SpecError`] re-exported from MAVSpec.
    #[cfg_attr(feature = "std", error("frame decoding/encoding error: {0:?}"))]
    Spec(SpecError),
}

/// Errors related to MAVLink frame validation.
///
/// This means, that frame is already present but hasn't passed certain criteria.
#[derive(Clone, Debug)]
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
pub struct IncompatFlagsError {
    /// Expected flag set.
    pub expected: IncompatFlags,
    /// Actual flag set.
    pub actual: IncompatFlags,
}

impl From<VersionError> for FrameError {
    /// Converts [`VersionError`] into [`FrameError::Version`].
    fn from(value: VersionError) -> Self {
        Self::Version(value)
    }
}

impl From<VersionError> for Error {
    /// Converts [`VersionError`] into [`FrameError::Version`]  variant of [`Error::Frame`].
    fn from(value: VersionError) -> Self {
        FrameError::from(value).into()
    }
}

impl From<ChecksumError> for FrameError {
    /// Converts [`ChecksumError`] into [`FrameError::Checksum`].
    fn from(_: ChecksumError) -> Self {
        Self::Checksum
    }
}

impl From<ChecksumError> for Error {
    /// Converts [`ChecksumError`] into [`FrameError::Checksum`] variant of [`Error::Frame`].
    fn from(value: ChecksumError) -> Self {
        FrameError::from(value).into()
    }
}

impl From<SignatureError> for FrameError {
    /// Converts [`SignatureError`] into [`FrameError::Signature`].
    fn from(_: SignatureError) -> Self {
        Self::Signature
    }
}

impl From<SignatureError> for Error {
    /// Converts [`SignatureError`] into [`FrameError::Signature`] variant of [`Error::Frame`].
    fn from(value: SignatureError) -> Self {
        FrameError::from(value).into()
    }
}

impl From<IncompatFlagsError> for FrameError {
    /// Converts [`IncompatFlagsError`] into [`FrameError::Incompatible`].
    fn from(value: IncompatFlagsError) -> Self {
        Self::Incompatible(value)
    }
}

impl From<IncompatFlagsError> for Error {
    /// Converts [`IncompatFlagsError`] into [`FrameError::Incompatible`] variant of [`Error::Frame`].
    fn from(value: IncompatFlagsError) -> Self {
        FrameError::from(value).into()
    }
}

#[cfg(feature = "std")]
impl From<std::io::Error> for Error {
    /// Convert [`std::io::Error`] into [`Error::Io`].
    ///
    /// Note that [`Error::Io`] wraps IO error with [`Arc`] to make [`Error`] compatible with [`Clone`] trait.
    fn from(value: std::io::Error) -> Self {
        Error::Io(Arc::new(value))
    }
}

impl From<FrameError> for Error {
    /// Converts [`FrameError`] into [`Error::Frame`].
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
