//! # MAVLink frame writer

use tokio::io::AsyncWrite;

use crate::errors::Result;
use crate::protocol::frame::Frame;

/// Sends MAVLink frames asynchronously.
///
/// Sends MAVLink frames to an instance of [`AsyncWrite`].  
#[derive(Clone, Debug)]
pub struct AsyncSender<W: AsyncWrite + Unpin> {
    writer: W,
}

impl<W: AsyncWrite + Unpin> AsyncSender<W> {
    /// Default constructor.
    pub fn new(writer: W) -> Self {
        Self { writer }
    }

    /// Sends MAVLink [`Frame`] asynchronously.
    ///
    /// Returns the number of bytes sent.
    pub async fn send(&mut self, frame: &Frame) -> Result<usize> {
        frame.send_async(&mut self.writer).await
    }
}
