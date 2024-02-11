//! # MAVLink frame writer

use core::marker::PhantomData;

use tokio::io::AsyncWrite;

use crate::protocol::{Frame, MaybeVersioned, Versioned, Versionless};

use crate::prelude::*;

/// Sends MAVLink frames asynchronously.
///
/// Sends MAVLink frames to an instance of [`AsyncWrite`].  
#[derive(Clone, Debug)]
pub struct AsyncSender<W: AsyncWrite + Unpin, V: MaybeVersioned> {
    writer: W,
    _marker_version: PhantomData<V>,
}

impl<W: AsyncWrite + Unpin> AsyncSender<W, Versionless> {
    /// Default constructor.
    ///
    /// Creates a protocol-agnostic sender which can send both `MAVLink 1` and `MAVLink 2` frames.
    ///
    /// If you want a sender that sends only frames restricted to a particular MAVLink protocol
    /// version, use [`AsyncSender::versioned`].
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            _marker_version: PhantomData,
        }
    }

    /// Create an [`AsyncSender`] that accepts only frames of a specified MAVLink dialect.
    ///
    /// If you want to send both `MAVLink 1` and `MAVLink 2` frames, use [`AsyncSender::new`].
    pub fn versioned<Version: Versioned>(writer: W) -> AsyncSender<W, Version> {
        AsyncSender {
            writer,
            _marker_version: PhantomData,
        }
    }

    /// Send MAVLink [`Frame`] asynchronously.
    ///
    /// Accepts both `MAVLink 1` and `MAVLink 2` frames as [`Frame<Versionless>`].
    ///
    /// Returns the number of bytes sent.
    pub async fn send(&mut self, frame: &Frame<Versionless>) -> Result<usize> {
        frame.send_async(&mut self.writer).await
    }
}

impl<W: AsyncWrite + Unpin, V: Versioned> AsyncSender<W, V> {
    /// Send MAVLink [`Frame`] asynchronously.
    ///
    /// Accepts only frames of a specific MAVLink protocol version. Otherwise, returns
    /// [`FrameError::InvalidVersion`].
    ///
    /// Returns the number of bytes sent.
    pub async fn send<Version: Versioned>(&mut self, frame: &Frame<Version>) -> Result<usize> {
        Version::marker().expect(frame.mavlink_version())?;
        frame.send_async(&mut self.writer).await
    }
}
