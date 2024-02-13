//! # MAVLink frame writer

use core::marker::PhantomData;

use tokio::io::AsyncWrite;

use crate::protocol::{Dialectless, Frame, MaybeDialect, MaybeVersioned, Versioned, Versionless};

use crate::prelude::*;

/// Sends MAVLink frames asynchronously.
///
/// Sends MAVLink frames to an instance of [`AsyncWrite`].  
#[derive(Clone, Debug)]
pub struct AsyncSender<W: AsyncWrite + Unpin, V: MaybeVersioned, D: MaybeDialect> {
    writer: W,
    _marker_version: PhantomData<V>,
    _marker_dialect: D,
}

impl<W: AsyncWrite + Unpin> AsyncSender<W, Versionless, Dialectless> {
    /// Default constructor.
    pub fn new<Version: MaybeVersioned>(writer: W) -> AsyncSender<W, Version, Dialectless> {
        AsyncSender {
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
    /// version, use [`AsyncSender::versioned`].
    ///
    /// If you want to instantiate a generic sender, use [`AsyncSender::new`].
    pub fn versionless(writer: W) -> Self {
        AsyncSender::new(writer)
    }

    /// Create a receiver specific to a particular MAVLink protocol version.
    ///
    /// Same as [`AsyncSender::new::<V1>`] / [`AsyncSender::new::<V2>`] but with an explicit
    /// `version` marker as parameter.
    ///
    /// If you want to send both `MAVLink 1` and `MAVLink 2` frames, use [`AsyncSender::versionless`].
    pub fn versioned<Version: Versioned>(
        writer: W,
        #[allow(unused_variables)] version: Version,
    ) -> AsyncSender<W, Version, Dialectless> {
        AsyncSender::new(writer)
    }
}

impl<W: AsyncWrite + Unpin, V: MaybeVersioned> AsyncSender<W, V, Dialectless> {
    /// Send MAVLink [`Frame`] asynchronously.
    ///
    /// [`Versioned`] sender accepts only frames of a specific MAVLink protocol version. Otherwise,
    /// returns [`FrameError::InvalidVersion`].
    ///
    /// [`Versionless`] sender accepts both `MAVLink 1` and `MAVLink 2` frames as
    /// [`Frame<Versionless, _>`].
    ///
    /// Returns the number of bytes sent.
    pub async fn send_frame(&mut self, frame: &Frame<V, Dialectless>) -> Result<usize> {
        V::expect(frame.version())?;
        frame.send_async(&mut self.writer).await
    }
}
