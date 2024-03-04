//! # MAVLink frame writer

use core::marker::PhantomData;

use tokio::io::{AsyncWrite, AsyncWriteExt};

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
    pub fn new<V: MaybeVersioned>(writer: W) -> AsyncSender<W, V> {
        AsyncSender {
            writer,
            _marker_version: PhantomData,
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
    ) -> AsyncSender<W, Version> {
        AsyncSender::new(writer)
    }
}

impl<W: AsyncWrite + Unpin, V: MaybeVersioned> AsyncSender<W, V> {
    /// Send MAVLink [`Frame`] asynchronously.
    ///
    /// [`Versioned`] sender accepts only frames of a specific MAVLink protocol version. Otherwise,
    /// returns [`FrameError::InvalidVersion`].
    ///
    /// [`Versionless`] sender accepts both `MAVLink 1` and `MAVLink 2` frames as
    /// [`Frame<Versionless>`].
    ///
    /// Returns the number of bytes sent.
    pub async fn send(&mut self, frame: &Frame<V>) -> Result<usize> {
        V::expect(frame.version())?;
        frame.send_async(&mut self.writer).await
    }

    /// Flushes all buffers.
    ///
    /// Certain writers require flush to be called on tear down in order to write all contents.
    pub async fn flush(&mut self) -> Result<()> {
        self.writer.flush().await.map_err(Error::from)
    }
}
