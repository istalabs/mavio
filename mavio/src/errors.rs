//! # Errors
//!
//! These errors used in `mavio`.
//!
//! The top-level error is [`Error`]. Library API returns versions of this error possibly wrapping
//! other types of errors.
//!
//! We also re-export errors from [`mavspec::rust::spec`](https://docs.rs/mavspec/latest/mavspec/rust/spec/)
//! to provide a full specification of MAVLink-related errors.

use tbytes::errors::TBytesError;

#[cfg(feature = "std")]
use std::sync::Arc;

use crate::protocol::MavLinkVersion;

// Re-export `mavspec::rust::spec` errors.
#[doc(no_inline)]
pub use mavspec::rust::spec::{PayloadError, SpecError};

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

    /// Buffer error.
    ///
    /// This error is internal to Mavio and potentially indicates a bug in implementation. You
    /// should not rely on this error since later versions may fix the implementation in a way that
    /// such error may not occur at all.
    #[deprecated]
    #[cfg_attr(feature = "std", error("MAVLink 2 signature is too small"))]
    Buffer(TBytesError),
}

/// Errors related to MAVLink frame validation.
///
/// This means, that frame is already present but hasn't passed certain criteria.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "std", derive(thiserror::Error))]
pub enum FrameError {
    /// Incorrect MAVLink version.
    #[cfg_attr(
        feature = "std",
        error("invalid MAVLink version, expected: {expected:?}, actual: {actual:?}")
    )]
    InvalidVersion {
        /// Expected protocol version.
        expected: MavLinkVersion,
        /// Actual protocol version.
        actual: MavLinkVersion,
    },
    /// Upon calculation, CRC does not match received [Frame::checksum](crate::Frame::checksum).
    #[cfg_attr(feature = "std", error("checksum validation failed"))]
    InvalidChecksum,
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

#[allow(deprecated)]
impl From<TBytesError> for Error {
    /// Converts [`TBytesError`] into [`Error::Buffer`].
    fn from(value: TBytesError) -> Self {
        Self::Buffer(value)
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
    fn from(value: SpecError) -> Self {
        Self::Spec(value)
    }
}
