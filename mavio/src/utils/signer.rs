use crate::consts::SIGNATURE_VALUE_LENGTH;
use sha2::{Digest, Sha256};

use crate::protocol::signature::Sign;
use crate::protocol::SignatureValue;

/// Signs MAVLink packages with [`sha2`](https://crates.io/crates/sha2) library.
///
/// Implements `sha256_48`, a `MAVLink 2` specific hashing algorithm similar to regular `sha256` except that only first
/// 48 bits are considered.
///
/// # Links
///
/// * [Signature specification](https://mavlink.io/en/guide/message_signing.html#signature) format in MAVLink docs.
#[derive(Debug, Default)]
pub struct MavSha256 {
    hasher: Sha256,
}

impl Sign for MavSha256 {
    fn reset(&mut self) {
        self.hasher.reset();
    }

    /// Consume data as a slice of bytes.
    fn digest(&mut self, bytes: &[u8]) {
        self.hasher.update(bytes)
    }

    /// Calculates signature from digested data.
    fn signature(&self) -> SignatureValue {
        let sha256 = self.hasher.clone().finalize();
        let mut bytes = [0; SIGNATURE_VALUE_LENGTH];
        bytes.copy_from_slice(&sha256[0..SIGNATURE_VALUE_LENGTH]);
        bytes
    }
}
