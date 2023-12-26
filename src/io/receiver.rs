//! # MAVLink frame writer

use crate::errors::Result;
use crate::io::Read;
use crate::protocol::frame::Frame;

/// Receives MAVLink frames.
///
/// Receives MAVLink frames from an instance of [`Read`].
#[derive(Clone, Debug)]
pub struct Receiver<R: Read> {
    reader: R,
}

impl<R: Read> Receiver<R> {
    /// Default constructor.
    pub fn new(reader: R) -> Self {
        Self { reader }
    }

    /// Receives MAVLink [`Frame`].
    ///
    /// Blocks until a valid MAVLink frame received.
    pub fn recv(&mut self) -> Result<Frame> {
        Frame::recv(&mut self.reader)
    }
}
