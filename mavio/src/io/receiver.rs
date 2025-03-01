//! # MAVLink frame writer

use core::marker::PhantomData;

use crate::io::Read;
use crate::protocol::{Frame, MaybeVersioned, Versioned, Versionless};

use crate::prelude::*;

/// Receives MAVLink frames.
///
/// Receives MAVLink frames from an instance of [`Read`].
///
/// Instead of relying on a particular definition of read trait, we allow users to use any library
/// with I/O capabilities. See [`Read`] for details.
#[derive(Clone, Debug)]
pub struct Receiver<E: Into<Error>, R: Read<E>, V: MaybeVersioned> {
    reader: R,
    _error_marker: PhantomData<E>,
    _marker_version: PhantomData<V>,
}

impl<E: Into<Error>, R: Read<E>> Receiver<E, R, Versionless> {
    /// Default constructor.
    pub fn new<V: MaybeVersioned>(reader: R) -> Receiver<E, R, V> {
        Receiver {
            reader,
            _error_marker: PhantomData,
            _marker_version: PhantomData,
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
    pub fn versionless(reader: R) -> Self {
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
    /// frames into a specific protocol version with [`Frame::try_into_versioned`].
    pub fn versioned<Version: Versioned>(
        reader: R,
        #[allow(unused_variables)] version: Version,
    ) -> Receiver<E, R, Version> {
        Receiver::new(reader)
    }
}

impl<E: Into<Error>, R: Read<E>, V: MaybeVersioned> Receiver<E, R, V> {
    /// Receives MAVLink [`Frame`].
    ///
    /// Blocks until a valid MAVLink frame received.
    ///
    /// [`Versioned`] receiver accepts only frames of a specific MAVLink protocol version.
    ///
    /// [`Versionless`] receiver accepts both `MAVLink 1` and `MAVLink 2` frames.
    #[inline(always)]
    pub fn recv(&mut self) -> Result<Frame<V>> {
        Frame::<V>::recv(&mut self.reader).map_err(E::into)
    }
}
