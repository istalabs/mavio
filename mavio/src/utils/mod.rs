//! # Utils
//!
//! Utility functions, structs and traits which does not fall into any category.

mod slice_rw;
pub use slice_rw::{SliceReader, SliceWriter};

#[cfg(feature = "sha2")]
mod signer;
#[cfg(feature = "sha2")]
pub use signer::MavSha256;
