//! # MAVLink frame writer

use core::marker::PhantomData;

use crate::io::Write;
use crate::protocol::{Dialectless, Frame, MaybeDialect, MaybeVersioned, Versioned, Versionless};

use crate::prelude::*;

/// Sends MAVLink frames.
///
/// Sends MAVLink frames to an instance of [`Write`].  
#[derive(Clone, Debug)]
pub struct Sender<W: Write, V: MaybeVersioned, D: MaybeDialect> {
    writer: W,
    _marker_version: PhantomData<V>,
    _marker_dialect: D,
}

impl<W: Write> Sender<W, Versionless, Dialectless> {
    /// Default constructor.
    pub fn new<Version: MaybeVersioned>(writer: W) -> Sender<W, Version, Dialectless> {
        Sender {
            writer,
            _marker_version: PhantomData,
            _marker_dialect: Dialectless,
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
    ) -> Sender<W, Version, Dialectless> {
        Sender::new(writer)
    }
}

impl<W: Write, V: MaybeVersioned> Sender<W, V, Dialectless> {
    /// Sends MAVLink [`Frame`].
    ///
    /// Blocks until all bytes written and returns the number of bytes sent.
    ///
    /// [`Versioned`] sender accepts only frames of a specific MAVLink protocol version. Otherwise,
    /// returns [`FrameError::InvalidVersion`].
    ///
    /// [`Versionless`] sender accepts both `MAVLink 1` and `MAVLink 2` frames as
    /// [`Frame<Versionless, _>`].
    pub fn send_frame(&mut self, frame: &Frame<V, Dialectless>) -> Result<usize> {
        V::expect(frame.version())?;
        frame.send(&mut self.writer)
    }
}
