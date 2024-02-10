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

    /// Returns [`FrameIterator`] which iterates over [`Frame`]s.
    ///
    /// *&#9888; unstable feature*
    ///
    /// Note that [`FrameIterator`] accepts a mutable reference to [`Receiver`]. This
    #[cfg(feature = "unstable")]
    pub fn iter_mut(&mut self) -> FrameIterator<R> {
        FrameIterator::new(self)
    }
}

#[cfg(feature = "unstable")]
mod iterator {
    use super::Receiver;
    use crate::errors::{Error, Result};
    use crate::io::Read;
    use crate::protocol::frame::Frame;

    /// Iterates over [`Frame`]s.
    ///
    /// *&#9888; unstable feature*
    ///
    /// Use [`Receiver::iter_mut`] to construct an instance of [`FrameIterator`].
    #[derive(Debug)]
    pub struct FrameIterator<'a, R: Read> {
        reader: &'a mut Receiver<R>,
        err: Option<Error>,
    }

    impl<'a, R: Read> FrameIterator<'a, R> {
        pub(super) fn new(reader: &'a mut Receiver<R>) -> Self {
            Self { reader, err: None }
        }

        /// Error that caused iteration halt.
        ///
        /// *&#9888; unstable feature*
        pub fn err(&self) -> Option<&Error> {
            self.err.as_ref()
        }
    }

    impl<'a, R: Read> Iterator for FrameIterator<'a, R> {
        type Item = Result<Frame>;

        /// Advances iterator to the next MAVLink [`Frame`].
        ///
        /// *&#9888; unstable feature*
        ///
        /// Returns [`None`] when receives an error incompatible with further iteration. In such case you can check the
        /// cause of iteration halt through [`FrameIterator::err`].
        fn next(&mut self) -> Option<Self::Item> {
            let frame = self.reader.recv();

            match frame {
                Ok(_) => {
                    self.err = None;
                    Some(frame)
                }
                Err(err) => {
                    if let Error::Io(_) = err {
                        self.err = Some(err);
                        None
                    } else {
                        Some(Err(err))
                    }
                }
            }
        }
    }
}
#[cfg(feature = "unstable")]
pub use iterator::FrameIterator;
