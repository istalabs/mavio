use crate::protocol::{Checksum, Header, Signature};

use crate::prelude::*;

/// Implementors of this trait can scramble (or unscramble) frame data.
pub trait Scramble<V: MaybeVersioned> {
    /// Scramblers frame payload and signature.
    ///
    /// Used with [`Frame::scramble`] to update frame payload ad signature (for `MAVLink 2` frames).
    /// This trait allows to implement encryption algorithms without breaking a MAVLink protocol.
    ///
    /// If scrambler adds a signature, then `MAVLink 2` frame will be considered signed (in vice
    /// versa). For `MAVLink 1` frames adding signature won't have any effect on a frame.
    fn scramble(
        &mut self,
        header: Header<V>,
        payload: &mut [u8],
        checksum: &mut Checksum,
        signature: &mut Option<Signature>,
    );
}
