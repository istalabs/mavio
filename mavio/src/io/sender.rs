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
    ///
    /// Creates a protocol-agnostic sender which can send both `MAVLink 1` and `MAVLink 2` frames.
    ///
    /// If you want a sender that sends only frames restricted to a particular MAVLink protocol
    /// version, use [`Sender::versioned`].
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            _marker_version: PhantomData,
        }
    }

    /// Create a [`Sender`] that accepts only frames of a specified MAVLink dialect.
    ///
    /// If you want to send both `MAVLink 1` and `MAVLink 2` frames, use [`Sender::new`].
    pub fn versioned<Version: Versioned>(writer: W) -> Sender<W, Version> {
        Sender {
            writer,
            _marker_version: PhantomData,
        }
    }

    /// Sends MAVLink [`Frame`].
    ///
    /// Blocks until all bytes written and returns the number of bytes sent.
    ///
    /// Accepts both `MAVLink 1` and `MAVLink 2` frames as [`Frame<Versionless>`].
    pub fn send(&mut self, frame: &Frame<Versionless>) -> Result<usize> {
        frame.send(&mut self.writer)
    }
}

impl<W: Write, V: Versioned> Sender<W, V> {
    /// Sends MAVLink [`Frame`].
    ///
    /// Blocks until all bytes written and returns the number of bytes sent.
    ///
    /// Accepts only frames of a specific MAVLink protocol version. Otherwise, returns
    /// [`FrameError::InvalidVersion`].
    pub fn send<Version: Versioned>(&mut self, frame: &Frame<Version>) -> Result<usize> {
        Version::marker().expect(frame.mavlink_version())?;
        frame.send(&mut self.writer)
    }
}
