//! # MAVLink frame writer

use core::marker::PhantomData;

use tokio::io::AsyncRead;

use crate::protocol::{Frame, MaybeVersioned, Versioned, Versionless};

use crate::prelude::*;

/// Receives MAVLink frames asynchronously.
///
/// Receives MAVLink frames from an instance of [`AsyncRead`].
#[derive(Clone, Debug)]
pub struct AsyncReceiver<R: AsyncRead + Unpin, V: MaybeVersioned> {
    reader: R,
    _marker_version: PhantomData<V>,
}

impl<R: AsyncRead + Unpin> AsyncReceiver<R, Versionless> {
    /// Default constructor.
    ///
    /// Creates a protocol-agnostic receiver which will look up for both `MAVLink 1` and `MAVLink 2`
    /// frames.
    ///
    /// If you want a receiver restricted to a specific MAVLink protocol version, use
    /// [`AsyncReceiver::versioned`].
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            _marker_version: PhantomData,
        }
    }

    /// Create a [`AsyncReceiver`] that accepts only messages of a specified MAVLink dialect.
    ///
    /// Since versioned receiver will look up for MAVLink frames starting with a specific
    /// [`MavSTX`](crate::protocol::MavSTX) magic byte, it may behave incorrectly, if the incoming
    /// stream contains frames of a different protocol version. If this is the case, it is preferred
    /// to construct versionless receiver by [`AsyncReceiver::new`] and then attempt to convert incoming
    /// frames into a specific protocol version with [`Frame::try_versioned`].
    pub fn versioned<Version: Versioned>(reader: R) -> AsyncReceiver<R, Version> {
        AsyncReceiver {
            reader,
            _marker_version: PhantomData,
        }
    }

    /// Receives MAVLink [`Frame`].
    ///
    /// Blocks until a valid MAVLink frame received.
    ///
    /// Returns a [`Frame<Versionless>`] which then can be cast into a protocol-specific form by
    /// [`Frame::try_versioned`].
    pub async fn recv(&mut self) -> Result<Frame<Versionless>> {
        Frame::<Versionless>::recv_async(&mut self.reader).await
    }
}

impl<R: AsyncRead + Unpin, V: Versioned> AsyncReceiver<R, V> {
    /// Receives MAVLink [`Frame`].
    ///
    /// Blocks until a valid MAVLink frame received.
    ///
    /// Accepts only frames of a specific MAVLink protocol version. Otherwise, returns
    /// [`FrameError::InvalidVersion`].
    pub async fn recv(&mut self) -> Result<Frame<V>> {
        Frame::<V>::recv_async(&mut self.reader).await
    }
}
