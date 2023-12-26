//! # MAVLink frame writer

use crate::errors::Result;
use crate::io::Write;
use crate::protocol::frame::Frame;

/// Sends MAVLink frames.
///
/// Sends MAVLink frames to an instance of [`Write`].  
#[derive(Clone, Debug)]
pub struct Sender<W: Write> {
    writer: W,
}

impl<W: Write> Sender<W> {
    /// Default constructor.
    pub fn new(writer: W) -> Self {
        Self { writer }
    }

    /// Sends MAVLink [`Frame`].
    ///
    /// Blocks until all bytes written and returns the number of bytes sent.
    pub fn send(&mut self, frame: &Frame) -> Result<usize> {
        frame.send(&mut self.writer)
    }
}
