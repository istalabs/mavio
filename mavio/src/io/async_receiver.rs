//! # MAVLink frame writer

use tokio::io::AsyncRead;

use crate::errors::Result;
use crate::protocol::frame::Frame;

/// Receives MAVLink frames asynchronously.
///
/// Receives MAVLink frames from an instance of [`AsyncRead`].
#[derive(Clone, Debug)]
pub struct AsyncReceiver<R: AsyncRead + Unpin> {
    reader: R,
}

impl<R: AsyncRead + Unpin> AsyncReceiver<R> {
    /// Default constructor.
    pub fn new(reader: R) -> Self {
        Self { reader }
    }

    /// Receives MAVLink [`Frame`].
    ///
    /// Blocks until a valid MAVLink frame received.
    pub async fn recv(&mut self) -> Result<Frame> {
        Frame::recv_async(&mut self.reader).await
    }
}
