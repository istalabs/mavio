//! # MAVLink frame writer

use core::marker::PhantomData;

use crate::io::Read;
use crate::protocol::{Frame, MaybeVersioned, Versioned, Versionless};

use crate::prelude::*;

/// Receives MAVLink frames.
///
/// Receives MAVLink frames from an instance of [`Read`].
#[derive(Clone, Debug)]
pub struct Receiver<R: Read, V: MaybeVersioned> {
    reader: R,
    _marker_version: PhantomData<V>,
}

impl<R: Read> Receiver<R, Versionless> {
    /// Default constructor.
    ///
    /// Creates a protocol-agnostic receiver which will look up for both `MAVLink 1` and `MAVLink 2`
    /// frames.
    ///
    /// If you want a receiver restricted to a specific MAVLink protocol version, use
    /// [`Receiver::versioned`].
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            _marker_version: PhantomData,
        }
    }

    /// Create a [`Receiver`] that accepts only messages of a specified MAVLink dialect.
    ///
    /// Since versioned receiver will look up for MAVLink frames starting with a specific
    /// [`MavSTX`](crate::protocol::MavSTX) magic byte, it may behave incorrectly, if the incoming
    /// stream contains frames of a different protocol version. If this is the case, it is preferred
    /// to construct versionless receiver by [`Receiver::new`] and then attempt to convert incoming
    /// frames into a specific protocol version with [`Frame::try_versioned`].
    pub fn versioned<Version: Versioned>(reader: R) -> Receiver<R, Version> {
        Receiver {
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
    pub fn recv(&mut self) -> Result<Frame<Versionless>> {
        Frame::<Versionless>::recv(&mut self.reader)
    }
}

impl<R: Read, V: Versioned> Receiver<R, V> {
    /// Receives MAVLink [`Frame`].
    ///
    /// Blocks until a valid MAVLink frame received.
    ///
    /// Accepts only frames of a specific MAVLink protocol version. Otherwise, returns
    /// [`FrameError::InvalidVersion`].
    pub fn recv(&mut self) -> Result<Frame<V>> {
        Frame::<V>::recv(&mut self.reader)
    }
}
