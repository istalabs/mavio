//! Reader and writer for byte slices.

use crate::io::{Read, Write};

/// <sup>`extras`</sup>
/// Reads the contents of a predefined slice.
///
/// <sup>Available with `extras` Cargo feature</sup>
///
/// Receives a pre-defined slice and reads its contents while moving internal
/// cursor position.
///
/// Works both for `std` and `no_std` targets.
///
/// [`SliceReader`] created mainly for testing purposes. In most use cases there will be a better
/// alternative. However, since it may have a limited potential use, we've decided to include this
/// struct into `mavio` API.
#[derive(Debug, Default)]
pub struct SliceReader<'a> {
    content: &'a [u8],
    pos: usize,
}

impl<'a> SliceReader<'a> {
    /// Creates [`SliceReader`] from slice.
    pub fn new(content: &'a [u8]) -> Self {
        Self { content, pos: 0 }
    }

    /// Slice content.
    pub fn content(&self) -> &[u8] {
        self.content
    }

    /// Cursor position.
    pub fn pos(&self) -> usize {
        self.pos
    }

    /// Number of remaining bytes.
    pub fn num_remaining_bytes(&self) -> usize {
        self.content.len() - self.pos
    }

    fn read_internal(&mut self, buf: &mut [u8]) -> usize {
        let num_bytes_requested = buf.len();
        let num_bytes = core::cmp::min(self.content.len() - self.pos, num_bytes_requested);

        buf.copy_from_slice(&self.content[self.pos..self.pos + num_bytes]);
        self.pos += num_bytes;

        num_bytes
    }
}

#[cfg(not(feature = "std"))]
impl<'a> Read for SliceReader<'a> {
    /// Read the exact number of bytes required to fill `buf`.
    ///
    /// # Errors
    ///
    /// Returns [`IoError::UnexpectedEof`](no_std::IoError::UnexpectedEof) if buffer does not ave
    /// enough content.
    fn read_exact(&mut self, buf: &mut [u8]) -> crate::error::Result<()> {
        // Return error if the remaining data in internal buffer are not enough to fill the provided one
        if self.num_remaining_bytes() < buf.len() {
            return Err(crate::io::no_std::IoError::UnexpectedEof.into());
        }

        self.read_internal(buf);
        Ok(())
    }
}

#[cfg(feature = "std")]
impl<'a> Read<std::io::Error> for SliceReader<'a> {
    /// Read the exact number of bytes required to fill `buf`.
    ///
    /// # Errors
    ///
    /// Returns [`ErrorKind::UnexpectedEof`](std::io::ErrorKind::UnexpectedEof) if internal slice does not
    /// have enough content.
    fn read_exact(&mut self, buf: &mut [u8]) -> std::io::Result<()> {
        // Return error if the remaining data in internal buffer are not enough to fill the provided one
        if self.num_remaining_bytes() < buf.len() {
            return Err(make_err_eof(self.content.len() - self.pos, buf.len()));
        }

        self.read_internal(buf);
        Ok(())
    }
}

/// <sup>`extras`</sup>
/// Writes the contents to a predefined slice.
///
/// <sup>Available with `extras` Cargo feature</sup>
///
/// Receives a pre-defined slice and reads its contents while moving internal
/// cursor position.
///
/// Works both for `std` and `no_std` targets.
///
/// [`SliceWriter`] created mainly for testing purposes. In most use cases there will be a better alternative. However,
/// since it may have a limited potential use, we've decided to include this struct into `mavio` API.
#[derive(Debug, Default)]
pub struct SliceWriter<'a> {
    content: &'a mut [u8],
    pos: usize,
}

impl<'a> SliceWriter<'a> {
    /// Creates [`SliceReader`] from slice.
    pub fn new(content: &'a mut [u8]) -> Self {
        Self { content, pos: 0 }
    }

    /// Slice content.
    pub fn content(&self) -> &[u8] {
        self.content
    }

    /// Cursor position.
    pub fn pos(&self) -> usize {
        self.pos
    }

    /// Number of remaining bytes.
    pub fn num_remaining_bytes(&self) -> usize {
        self.content.len() - self.pos
    }

    fn write_internal(&mut self, buf: &[u8]) -> usize {
        let num_bytes_requested = buf.len();
        let num_bytes = core::cmp::min(self.content.len() - self.pos, num_bytes_requested);

        self.content[self.pos..self.pos + num_bytes].copy_from_slice(buf);
        self.pos += num_bytes;

        num_bytes
    }
}

#[cfg(not(feature = "std"))]
impl<'a> Write for SliceWriter<'a> {
    /// Attempts to write an entire buffer into this writer.
    ///
    /// # Errors
    ///
    /// Returns [`ErrorKind::UnexpectedEof`](std::io::ErrorKind::UnexpectedEof) if internal slice does not
    /// have enough content.
    fn write_all(&mut self, buf: &[u8]) -> crate::Result<()> {
        // Return error if internal buffer has insufficient size
        if self.num_remaining_bytes() < buf.len() {
            return Err(crate::io::no_std::IoError::UnexpectedEof.into());
        }

        self.write_internal(buf);
        Ok(())
    }
}

#[cfg(feature = "std")]
impl<'a> Write<std::io::Error> for SliceWriter<'a> {
    /// Write a buffer into this writer, returning how many bytes were written.
    ///
    /// # Errors
    ///
    /// Returns [`ErrorKind::UnexpectedEof`](std::io::ErrorKind::UnexpectedEof) if internal slice does not
    /// have enough content.
    fn write_all(&mut self, buf: &[u8]) -> Result<(), std::io::Error> {
        // Return error if internal buffer has insufficient size
        if self.num_remaining_bytes() < buf.len() {
            return Err(make_err_eof(self.content.len() - self.pos, buf.len()));
        }

        self.write_internal(buf);
        Ok(())
    }

    fn flush(&mut self) -> Result<(), std::io::Error> {
        Ok(())
    }
}

#[cfg(feature = "std")]
fn make_err_eof(internal_len: usize, requested_len: usize) -> std::io::Error {
    std::io::Error::new(
        std::io::ErrorKind::UnexpectedEof,
        format!(
            "buffer contains only {} bytes but {} requested",
            internal_len, requested_len
        ),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reader_reads() {
        let content = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9u8];
        let mut buffer = [0u8; 5];

        let mut reader = SliceReader::new(&content);
        reader.read_exact(buffer.as_mut_slice()).unwrap();

        assert_eq!(&content[0..5], &buffer[0..5]);
        assert_eq!(reader.pos(), 5);
    }

    #[test]
    fn reader_reads_it_all() {
        let content = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9u8];
        let mut buffer = [0u8; 10];

        let mut reader = SliceReader::new(&content);
        reader.read_exact(buffer.as_mut_slice()).unwrap();

        assert_eq!(content, buffer);
    }

    #[test]
    fn writer_writes() {
        let content = [0, 1, 2, 3, 4u8];
        let mut buffer = [0u8; 10];

        let mut writer = SliceWriter::new(&mut buffer);
        writer.write_all(&content).unwrap();

        assert_eq!(&content[0..5], &writer.content()[0..5]);
        assert_eq!(writer.pos(), 5);
    }

    #[test]
    fn writer_writes_it_all() {
        let content = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9u8];
        let mut buffer = [0u8; 10];

        let mut writer = SliceWriter::new(&mut buffer);
        writer.write_all(&content).unwrap();

        assert_eq!(content, buffer);
    }
}
