//! # Utils
//!
//! Utility functions, structs and traits which does not fall into any category.

pub(crate) mod sealed;
#[cfg(feature = "sha2")]
mod signer;
mod slice_rw;

#[cfg(feature = "sha2")]
pub use signer::MavSha256;
pub use slice_rw::{SliceReader, SliceWriter};
