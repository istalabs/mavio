//! # MAVLink frame writer

use core::marker::PhantomData;

use crate::io::AsyncWrite;
use crate::protocol::{Frame, MaybeVersioned, Versioned, Versionless};

use crate::prelude::*;

/// Sends MAVLink frames asynchronously.
///
/// Sends MAVLink frames to an instance of [`AsyncWrite`].  
#[derive(Clone, Debug)]
pub struct AsyncSender<E: Into<Error>, W: AsyncWrite<E> + Unpin, V: MaybeVersioned> {
    writer: W,
    _error_marker: PhantomData<E>,
    _marker_version: PhantomData<V>,
}

impl<E: Into<Error>, W: AsyncWrite<E> + Unpin> AsyncSender<E, W, Versionless> {
    /// Default constructor.
    pub fn new<V: MaybeVersioned>(writer: W) -> AsyncSender<E, W, V> {
        AsyncSender {
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
    ) -> AsyncSender<E, W, Version> {
        AsyncSender::new(writer)
    }
}

impl<E: Into<Error>, W: AsyncWrite<E> + Unpin, V: MaybeVersioned> AsyncSender<E, W, V> {
    /// Send MAVLink [`Frame`] asynchronously.
    ///
    /// [`Versioned`] sender accepts only frames of a specific MAVLink protocol version.
    ///
    /// [`Versionless`] sender accepts both `MAVLink 1` and `MAVLink 2` frames as
    /// [`Frame<Versionless>`].
    ///
    /// Returns the number of bytes sent.
    #[inline(always)]
    pub async fn send(&mut self, frame: &Frame<V>) -> Result<usize> {
        V::expect(frame.version())?;
        frame.send_async(&mut self.writer).await.map_err(E::into)
    }

    /// Flushes all buffers.
    ///
    /// Certain writers require flush to be called on tear down in order to write all contents.
    #[inline]
    pub async fn flush(&mut self) -> Result<()> {
        self.writer.flush().await.map_err(E::into)
    }
}
