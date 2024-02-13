//! Common generic markers.
//!
//! These markers are used to distinguish different versions of generic entities.

mod dialect;
mod private;
mod version;

pub use dialect::{Dialectless, HasDialect, MaybeDialect};
pub use private::*;
pub use version::{MaybeVersioned, Versioned, Versionless, V1, V2};
