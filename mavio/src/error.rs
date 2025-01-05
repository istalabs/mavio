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
    /// [`std::io::Error`] wrapper.
    #[cfg(feature = "std")]
    #[cfg_attr(feature = "std", error("I/O error: {0:?}"))]
    Io(IoError),

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

/// I/O errors.
///
/// Errors returned by [`mavio`](crate) I/O.
///
/// These errors will be wrapped with [`Error::Io`] upon
/// returning to client.
///
/// See:
///  * [`Error::Io`].
///  * [`std::result::Result`].
///
/// ## Caveats
///
/// We provide a limited support for [Serde](https://serde.rs) and
/// [Specta](https://crates.io/crates/specta). At the moment, the goal is simply to show something
/// meaningful.
///
/// This part of the API is considered unstable. Breaking changes may be introduced at any time.
///
/// ### Serde
///
/// We provide a simplified serialization sufficient to display an error. The deserialized error
/// always has a kind of [`std::io::ErrorKind::Other`]. In future version the behavior may change.
///
/// ### Specta
///
/// We provide a simplified type definition:
///
/// ```rust,no_run
/// struct IoError {
///     kind: String,
///     error: String,
/// }
/// ```
#[cfg(feature = "std")]
#[derive(Clone)]
pub struct IoError {
    inner: Arc<std::io::Error>,
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

#[cfg(feature = "std")]
impl AsRef<std::io::Error> for IoError {
    fn as_ref(&self) -> &std::io::Error {
        self.inner.as_ref()
    }
}

#[cfg(feature = "std")]
impl core::fmt::Debug for IoError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.as_ref().fmt(f)
    }
}

#[cfg(all(feature = "serde", feature = "unstable"))]
#[cfg(feature = "std")]
impl serde::Serialize for IoError {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut io_error = serializer.serialize_struct("IoError", 2)?;
        io_error.serialize_field("kind", &format!("{:?}", self.as_ref().kind()))?;
        io_error.serialize_field("error", &self.as_ref().to_string())?;
        io_error.end()
    }
}

#[cfg(all(feature = "serde", feature = "unstable"))]
#[cfg(feature = "std")]
impl<'de> serde::Deserialize<'de> for IoError {
    fn deserialize<D>(_: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(IoError {
            inner: Arc::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Foreign I/O error",
            )),
        })
    }
}

#[cfg(all(feature = "specta", feature = "unstable"))]
#[cfg(feature = "std")]
#[derive(specta::Type)]
#[allow(dead_code)]
struct IoErrorStub {
    kind: String,
    error: String,
}

#[cfg(all(feature = "specta", feature = "unstable"))]
#[cfg(feature = "std")]
impl specta::Type for IoError {
    fn inline(type_map: &mut specta::TypeMap, generics: specta::Generics) -> specta::DataType {
        specta::DataType::from(IoErrorStub::inline(type_map, generics))
    }
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
impl From<std::io::Error> for IoError {
    fn from(value: std::io::Error) -> Self {
        Self {
            inner: Arc::new(value),
        }
    }
}

#[cfg(feature = "std")]
impl From<IoError> for Error {
    fn from(value: IoError) -> Self {
        Error::Io(value)
    }
}

#[cfg(feature = "std")]
impl From<std::io::Error> for Error {
    /// Convert [`std::io::Error`] into [`Error::Io`].
    ///
    /// Note that [`Error::Io`] wraps IO error with [`Arc`] to make [`Error`] compatible with [`Clone`] trait.
    fn from(value: std::io::Error) -> Self {
        Error::Io(IoError {
            inner: Arc::new(value),
        })
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
