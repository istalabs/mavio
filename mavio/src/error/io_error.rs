use crate::error::Error;
#[cfg(feature = "std")]
use std::sync::Arc;

/// I/O errors.
///
/// Errors returned by [`mavio`](crate) I/O.
///
/// These errors will be wrapped with [`Error::Io`] upon
/// returning to client.
///
/// ## Caveats
///
/// In order to provide transport-agnostic API, we face the necessity to pass errors from different
/// I/O implementations. Check [`IoError::kind`] to analyse the error.
///
/// For now, we support the following I/O errors:
///
/// - [`std::io::Error`] → [`IoErrorKind::Std`]
/// - [`tokio::io::Error`] → [`IoErrorKind::Std`]
/// - [`embedded_io::Error`] → [`IoErrorKind::Embedded`]
///
/// ## Caveats
///
/// ### [`IoErrorKind::Std`]
///
/// For the standard I/O errors you can obtain an optional reference to the original error by using
/// [`IoError::error`].
///
/// ### [`IoErrorKind::Embedded`]
///
/// In the case of [`embedded_io::Error`] we erase all information but the
/// [`embedded_io::ErrorKind`], the [`IoError::error`] will always return [`None`]. Additionally,
/// [`embedded_io::ReadExactError::UnexpectedEof`] is translated to [`IoErrorKind::UnexpectedEof`]
/// when the unexpected end of a read stream is reached.
///
/// ## Unstable Features
///
/// We provide a limited support for [Serde](https://serde.rs) and
/// [Specta](https://crates.io/crates/specta). At the moment, the goal is simply to show something
/// meaningful.
///
/// This part of the API is considered unstable. Breaking changes may be introduced at any time.
///
/// To enable Serde and Specta support for errors use `unstable` Cargo feature.
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
#[derive(Clone)]
pub struct IoError {
    kind: IoErrorKind,
    #[cfg(feature = "std")]
    error: Option<Arc<std::io::Error>>,
}

/// Type of I/O error.
///
/// Allows to match errors for different I/O implementations.
#[cfg_attr(
    all(feature = "serde", feature = "unstable"),
    derive(serde::Serialize, serde::Deserialize)
)]
#[derive(Debug, Clone)]
pub enum IoErrorKind {
    /// Unexpected end of a stream or file.
    ///
    /// All [`embedded_io::ReadExactError::UnexpectedEof`] errors will be translated to this
    /// variant.
    UnexpectedEof,

    /// The kind of [`std::io::Error`].
    #[cfg(feature = "std")]
    #[cfg_attr(
        all(feature = "serde", feature = "unstable"),
        serde(skip_deserializing)
    )]
    #[cfg_attr(
        all(feature = "serde", feature = "unstable"),
        serde(serialize_with = "std_io_err_kind_serializer")
    )]
    Std(std::io::ErrorKind),

    /// The kind of [`embedded_io::Error`].
    #[cfg(any(feature = "embedded-io", feature = "embedded-io-async"))]
    #[cfg_attr(
        all(feature = "serde", feature = "unstable"),
        serde(skip_deserializing)
    )]
    #[cfg_attr(
        all(feature = "serde", feature = "unstable"),
        serde(serialize_with = "embedded_io_err_kind_serializer")
    )]
    Embedded(embedded_io::ErrorKind),

    /// Generic error.
    Generic,
}

impl IoError {
    /// Returns the kind of the error.
    #[inline]
    pub fn kind(&self) -> &IoErrorKind {
        &self.kind
    }

    /// Returns optional error.
    #[inline]
    #[cfg(feature = "std")]
    pub fn error(&self) -> Option<&std::io::Error> {
        self.error.as_ref().map(AsRef::as_ref)
    }

    #[cfg(any(feature = "embedded-io", feature = "embedded-io-async"))]
    pub(crate) fn from_embedded_io_error(value: impl embedded_io::Error) -> Self {
        #[cfg(feature = "std")]
        return Self {
            kind: IoErrorKind::Embedded(value.kind()),
            error: None,
        };
        #[cfg(not(feature = "std"))]
        return Self {
            kind: IoErrorKind::Embedded(value.kind()),
        };
    }
}

impl core::fmt::Debug for IoError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut builder = f.debug_struct("IoError");
        #[allow(unused_mut)]
        let mut builder_ref = builder.field("kind", &self.kind);
        #[cfg(feature = "std")]
        if let Some(err) = self.error.as_ref() {
            builder_ref = builder_ref.field("error", &err);
        }
        builder_ref.finish()
    }
}

#[cfg(all(feature = "serde", feature = "unstable"))]
impl serde::Serialize for IoError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut io_error = serializer.serialize_struct("IoError", 2)?;
        io_error.serialize_field("kind", self.kind())?;
        #[cfg(feature = "std")]
        if let Some(err) = self.error.as_ref() {
            io_error.serialize_field("error", &err.to_string())?;
        }
        io_error.end()
    }
}

#[cfg(feature = "std")]
#[cfg(all(feature = "serde", feature = "unstable"))]
fn std_io_err_kind_serializer<S>(
    value: &std::io::ErrorKind,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    use serde::ser::SerializeTuple;
    let mut seq = serializer.serialize_tuple(1)?;
    seq.serialize_element(&format!("{:?}", value))?;
    seq.end()
}

#[cfg(any(feature = "embedded-io", feature = "embedded-io-async"))]
#[cfg(all(feature = "serde", feature = "unstable"))]
fn embedded_io_err_kind_serializer<S>(
    value: &embedded_io::ErrorKind,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    use embedded_io::ErrorKind;
    match value {
        ErrorKind::NotFound => serializer.serialize_unit_struct("NotFound"),
        ErrorKind::PermissionDenied => serializer.serialize_unit_struct("PermissionDenied"),
        ErrorKind::ConnectionRefused => serializer.serialize_unit_struct("ConnectionRefused"),
        ErrorKind::ConnectionReset => serializer.serialize_unit_struct("ConnectionReset"),
        ErrorKind::ConnectionAborted => serializer.serialize_unit_struct("ConnectionAborted"),
        ErrorKind::NotConnected => serializer.serialize_unit_struct("NotConnected"),
        ErrorKind::AddrInUse => serializer.serialize_unit_struct("AddrInUse"),
        ErrorKind::AddrNotAvailable => serializer.serialize_unit_struct("AddrNotAvailable"),
        ErrorKind::BrokenPipe => serializer.serialize_unit_struct("BrokenPipe"),
        ErrorKind::AlreadyExists => serializer.serialize_unit_struct("AlreadyExists"),
        ErrorKind::InvalidInput => serializer.serialize_unit_struct("InvalidInput"),
        ErrorKind::InvalidData => serializer.serialize_unit_struct("InvalidData"),
        ErrorKind::TimedOut => serializer.serialize_unit_struct("Timeout"),
        ErrorKind::Interrupted => serializer.serialize_unit_struct("Interrupted"),
        ErrorKind::Unsupported => serializer.serialize_unit_struct("Unsupported"),
        ErrorKind::OutOfMemory => serializer.serialize_unit_struct("OutOfMemory"),
        ErrorKind::WriteZero => serializer.serialize_unit_struct("WriteZero"),
        _ => serializer.serialize_unit_struct("Other"),
    }
}

#[cfg(all(feature = "serde", feature = "unstable"))]
impl<'de> serde::Deserialize<'de> for IoError {
    fn deserialize<D>(_: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[cfg(feature = "std")]
        return Ok(IoError {
            kind: IoErrorKind::Std(std::io::ErrorKind::Other),
            error: Some(Arc::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Foreign I/O error",
            ))),
        });
        #[cfg(not(feature = "std"))]
        return Ok(IoError {
            kind: IoErrorKind::Generic,
        });
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

#[cfg(feature = "std")]
impl From<std::io::Error> for IoError {
    #[inline(always)]
    fn from(value: std::io::Error) -> Self {
        Self {
            kind: IoErrorKind::Std(value.kind()),
            error: Some(Arc::new(value)),
        }
    }
}

#[cfg(any(feature = "embedded-io", feature = "embedded-io-async"))]
impl From<embedded_io::ErrorKind> for IoError {
    #[inline(always)]
    fn from(value: embedded_io::ErrorKind) -> Self {
        #[cfg(feature = "std")]
        return Self {
            kind: IoErrorKind::Embedded(value),
            error: None,
        };
        #[cfg(not(feature = "std"))]
        return Self {
            kind: IoErrorKind::Embedded(value),
        };
    }
}

impl From<IoError> for Error {
    #[inline(always)]
    fn from(value: IoError) -> Self {
        Error::Io(value)
    }
}

impl From<IoErrorKind> for IoError {
    fn from(kind: IoErrorKind) -> Self {
        #[cfg(feature = "std")]
        return Self { kind, error: None };
        #[cfg(not(feature = "std"))]
        return Self { kind };
    }
}

#[cfg(feature = "std")]
impl From<std::io::Error> for Error {
    /// Convert [`std::io::Error`] into [`Error::Io`].
    ///
    /// Note that [`Error::Io`] wraps IO error with [`Arc`] to make [`Error`] compatible with [`Clone`] trait.
    fn from(value: std::io::Error) -> Self {
        Error::Io(IoError {
            kind: IoErrorKind::Std(value.kind()),
            error: Some(Arc::new(value)),
        })
    }
}
