//! # MAVLink frame writer

use core::marker::PhantomData;

use crate::io::Write;
use crate::protocol::{Frame, MaybeVersioned, Versioned, Versionless};

use crate::prelude::*;

/// Sends MAVLink frames.
///
/// Sends MAVLink frames to an instance of [`Write`].  
#[derive(Clone, Debug)]
pub struct Sender<W: Write, V: MaybeVersioned> {
    writer: W,
    _marker_version: PhantomData<V>,
}

impl<W: Write> Sender<W, Versionless> {
    /// Default constructor.
    pub fn new<V: MaybeVersioned>(writer: W) -> Sender<W, V> {
        Sender {
            writer,
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
    ) -> Sender<W, Version> {
        Sender::new(writer)
    }
}

impl<W: Write, V: MaybeVersioned> Sender<W, V> {
    /// Sends MAVLink [`Frame`].
    ///
    /// Blocks until all bytes written and returns the number of bytes sent.
    ///
    /// [`Versioned`] sender accepts only frames of a specific MAVLink protocol version. Otherwise,
    /// returns [`FrameError::InvalidVersion`].
    ///
    /// [`Versionless`] sender accepts both `MAVLink 1` and `MAVLink 2` frames as
    /// [`Frame<Versionless>`].
    pub fn send(&mut self, frame: &Frame<V>) -> Result<usize> {
        V::expect(frame.version())?;
        frame.send(&mut self.writer)
    }

    /// Flushes all buffers.
    ///
    /// Certain writers require flush to be called on tear down in order to write all contents.
    pub fn flush(&mut self) -> Result<()> {
        self.writer.flush().map_err(Error::from)
    }
}
