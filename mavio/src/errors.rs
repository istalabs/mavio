//! # Errors
//!
//! These errors used in `mavio`.
//!
//! The top-level error is [`Error`]. Library API returns versions of this error possibly wrapping
//! other types of errors.
//!
//! We also re-export errors from [`mavspec::rust::spec`] crate to provide a full specification of
//! MAVLink-related errors.

use tbytes::errors::TBytesError;

#[cfg(feature = "std")]
use std::sync::Arc;

// Re-export `mavspec::rust::spec` errors.
#[doc(no_inline)]
pub use mavspec::rust::spec::MessageError;

/// Common result type returned by `mavio` functions and methods.
pub type Result<T> = core::result::Result<T, Error>;

/// `mavio` top-level error.
///
/// [`Error`] is returned by most of the functions and methods across `mavio`. Other errors are either
/// converted to [`Error`] or wrapped by its variants.
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

    /// Frame encoding/decoding error.
    #[cfg_attr(feature = "std", error("frame decoding/encoding error: {0:?}"))]
    Frame(FrameError),

    /// Message encoding/decoding and specification discovery error.
    #[cfg_attr(feature = "std", error("frame decoding/encoding error: {0:?}"))]
    Message(MessageError),
}

/// Errors related to MAVLink frame encoding/decoding.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "std", derive(thiserror::Error))]
pub enum FrameError {
    /// MAVLink header is too small.
    #[cfg_attr(feature = "std", error("header is too small"))]
    HeaderIsTooSmall,
    /// `MAVLink 1` header is too small.
    #[cfg_attr(feature = "std", error("MAVLink 1 header is too small"))]
    HeaderV1IsTooSmall,
    /// `MAVLink 2` header is too small.
    #[cfg_attr(feature = "std", error("MAVLink 2 header is too small"))]
    HeaderV2IsTooSmall,
    /// Incorrect MAVLink version.
    #[cfg_attr(feature = "std", error("invalid MAVLink version"))]
    InvalidMavLinkVersion,
    /// `MAVLink 1` version is out of bounds.
    #[cfg_attr(feature = "std", error("`MAVLink 1` version is out of bounds"))]
    MessageIdV1OutOfBounds,
    /// `MAVLink 2` version is out of bounds.
    #[cfg_attr(feature = "std", error("`MAVLink 2` message ID is out of bounds"))]
    MessageIdV2OutOfBounds,
    /// Inconsistent `MAVLink 1` header: `MAVLink 2` fields are defined.
    #[cfg_attr(feature = "std", error("inconsistent MAVLink 1 header"))]
    InconsistentV1Header,
    /// Inconsistent `MAVLink 2` header: `MAVLink 2` fields are not defined.
    #[cfg_attr(feature = "std", error("inconsistent MAVLink 2 header"))]
    InconsistentV2Header,
    /// MAVLink packet body is inconsistent with header.
    #[cfg_attr(
        feature = "std",
        error("packet body length is inconsistent with header")
    )]
    InconsistentBodySize,
    /// `MAVLink 2` signature is too small.
    #[cfg_attr(feature = "std", error("MAVLink 2 signature is too small"))]
    SignatureIsTooSmall,
    /// Attempt to calculate `MAVLink 2` signature while necessary fields are missing.
    #[cfg_attr(feature = "std", error("`MAVLink 2` signature fields are missing"))]
    SignatureFieldsAreMissing,
    /// Attempt to calculate `MAVLink 2` signature in non-`MAVLink 2` context.
    #[cfg_attr(
        feature = "std",
        error("attempt to calculate `MAVLink 2` signature in non-`MAVLink 2` context")
    )]
    SigningIsNotSupported,
    /// Buffer error.
    #[cfg_attr(feature = "std", error("MAVLink 2 signature is too small"))]
    Buffer(TBytesError),
    /// Upon calculation CRC does not match received [MavLinkFrame::checksum](crate::Frame::checksum).
    #[cfg_attr(feature = "std", error("checksum validation failed"))]
    InvalidChecksum,

    /// Missing [`HeaderBuilder`](crate::protocol::header::HeaderBuilder) field when building a
    /// [`Header`](crate::protocol::header::Header).
    #[cfg_attr(
        feature = "std",
        error("can't build header since field `{0}` is missing")
    )]
    MissingHeaderField(&'static str),
    /// Missing [`FrameBuilder`](crate::protocol::frame::FrameBuilder) field when building a
    /// [`Frame`](crate::protocol::frame::Frame).
    #[cfg_attr(
        feature = "std",
        error("can't build frame since field `{0}` is missing")
    )]
    MissingFrameField(&'static str),

    /// Actual payload ahd header have inconsistent size.
    #[cfg_attr(
        feature = "std",
        error("actual payload ahd header have inconsistent size")
    )]
    InconsistentPayloadSize,
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

impl From<TBytesError> for FrameError {
    /// Converts [`TBytesError`] into [`FrameError::Buffer`].
    fn from(value: TBytesError) -> Self {
        FrameError::Buffer(value)
    }
}

impl From<TBytesError> for Error {
    /// Converts [`TBytesError`] into [`Error::Frame`].
    ///
    /// [`TBytesError`] be wrapped internally with [`FrameError`] and then passed to
    /// [`Error::Frame`].
    fn from(value: TBytesError) -> Self {
        Self::Frame(value.into())
    }
}

impl From<FrameError> for Error {
    /// Converts [`FrameError`] into [`Error::Frame`].
    fn from(value: FrameError) -> Self {
        Self::Frame(value)
    }
}

impl From<MessageError> for Error {
    /// Converts [`MessageError`] into [`Error::Message`].
    fn from(value: MessageError) -> Self {
        Self::Message(value)
    }
}
