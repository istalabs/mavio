//! Common generic markers.
//!
//! These markers are used to distinguish different versions of generic entities.

mod private;
mod unset;
mod version;

pub use private::*;
pub use unset::Unset;
pub use version::{MaybeVersioned, Versioned, Versionless, V1, V2};
