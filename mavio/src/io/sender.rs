//! # MAVLink frame writer

use core::marker::PhantomData;

use crate::io::Write;
use crate::protocol::{Frame, MaybeVersioned, Versioned, Versionless};

use crate::prelude::*;

/// Sends MAVLink frames.
///
/// Sends MAVLink frames to an instance of [`Write`].
///
/// Instead of relying on a particular definition of write trait, we allow users to use any library
/// with I/O capabilities. See [`Write`] for details.
#[derive(Clone, Debug)]
pub struct Sender<E: Into<Error>, W: Write<E>, V: MaybeVersioned> {
    writer: W,
    _error_marker: PhantomData<E>,
    _marker_version: PhantomData<V>,
}

impl<E: Into<Error>, W: Write<E>> Sender<E, W, Versionless> {
    /// Default constructor.
    pub fn new<V: MaybeVersioned>(writer: W) -> Sender<E, W, V> {
        Sender {
            writer,
            _error_marker: PhantomData,
            _marker_version: PhantomData,
        }
    }

    /// Create a MAVLink version agnostic sender.
    ///
    /// Creates a protocol-agnostic sender which can send both `MAVLink 1` and `MAVLink 2` frames.
    ///
    /// If you want a sender that sends only frames restricted to a particular MAVLink protocol
    /// version, use [`Sender::versioned`].
    ///
    /// If you want to instantiate a generic sender, use [`Sender::new`].
    pub fn versionless(writer: W) -> Self {
        Sender::new(writer)
    }

    /// Create a receiver specific to a particular MAVLink protocol version.
    ///
    /// Same as [`Sender::new::<V1>`] / [`Sender::new::<V2>`] but with an explicit `version`
    /// marker as parameter.
    ///
    /// If you want to send both `MAVLink 1` and `MAVLink 2` frames, use [`Sender::versionless`].
    pub fn versioned<Version: Versioned>(
        writer: W,
        #[allow(unused_variables)] version: Version,
    ) -> Sender<E, W, Version> {
        Sender::new(writer)
    }
}

impl<E: Into<Error>, W: Write<E>, V: MaybeVersioned> Sender<E, W, V> {
    /// Sends MAVLink [`Frame`].
    ///
    /// Blocks until all bytes written and returns the number of bytes sent.
    ///
    /// [`Versioned`] sender accepts only frames of a specific MAVLink protocol version.
    ///
    /// [`Versionless`] sender accepts both `MAVLink 1` and `MAVLink 2` frames as
    /// [`Frame<Versionless>`].
    #[inline(always)]
    pub fn send(&mut self, frame: &Frame<V>) -> Result<usize> {
        V::expect(frame.version())?;
        frame.send(&mut self.writer).map_err(E::into)
    }

    /// Flushes all buffers.
    ///
    /// Certain writers require flush to be called on tear down in order to write all contents.
    #[inline]
    pub fn flush(&mut self) -> Result<()> {
        self.writer.flush().map_err(E::into)
    }
}
