//! # MAVLink frame writer

use core::marker::PhantomData;

use crate::io::Read;
use crate::protocol::{Dialectless, Frame, MaybeDialect, MaybeVersioned, Versioned, Versionless};

use crate::prelude::*;

/// Receives MAVLink frames.
///
/// Receives MAVLink frames from an instance of [`Read`].
#[derive(Clone, Debug)]
pub struct Receiver<R: Read, V: MaybeVersioned, D: MaybeDialect> {
    reader: R,
    _marker_version: PhantomData<V>,
    _marker_dialect: D,
}

impl<R: Read> Receiver<R, Versionless, Dialectless> {
    /// Default constructor.
    pub fn new<Version: MaybeVersioned>(reader: R) -> Receiver<R, Version, Dialectless> {
        Receiver {
            reader,
            _marker_version: PhantomData,
            _marker_dialect: Dialectless,
        }
    }

    /// Create a MAVLink version agnostic receiver.
    ///
    /// Creates a protocol-agnostic receiver which will look up for both `MAVLink 1` and `MAVLink 2`
    /// frames.
    ///
    /// If you want a receiver restricted to a specific MAVLink protocol version, use
    /// [`Receiver::versioned`].
    ///
    /// If you want to instantiate a generic receiver, use [`Receiver::new`].
    pub fn versionless(reader: R) -> Receiver<R, Versionless, Dialectless> {
        Receiver::new(reader)
    }

    /// Create a receiver specific to a particular MAVLink protocol version.
    ///
    /// Same as [`Receiver::new::<V1>`] / [`Receiver::new::<V2>`] but with an explicit `version`
    /// marker as a parameter.
    ///
    /// Since versioned receiver will look up for MAVLink frames starting with a specific
    /// [`MavSTX`](crate::protocol::MavSTX) magic byte, it may behave incorrectly, if the incoming
    /// stream contains frames of a different protocol version. If this is the case, it is preferred
    /// to construct versionless receiver by [`Receiver::new`] and then attempt to convert incoming
    /// frames into a specific protocol version with [`Frame::try_versioned`].
    pub fn versioned<Version: Versioned>(
        reader: R,
        #[allow(unused_variables)] version: Version,
    ) -> Receiver<R, Version, Dialectless> {
        Receiver::new(reader)
    }
}

impl<R: Read, V: MaybeVersioned> Receiver<R, V, Dialectless> {
    /// Receives MAVLink [`Frame`].
    ///
    /// Blocks until a valid MAVLink frame received.
    ///
    /// [`Versioned`] receiver accepts only frames of a specific MAVLink protocol version.
    /// Otherwise, returns [`FrameError::InvalidVersion`].
    ///
    /// [`Versionless`] receiver accepts both `MAVLink 1` and `MAVLink 2` frames.
    pub fn recv_frame(&mut self) -> Result<Frame<V, Dialectless>> {
        Frame::<V, Dialectless>::recv(&mut self.reader)
    }
}
