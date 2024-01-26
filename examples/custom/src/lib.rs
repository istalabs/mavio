//! Mavio custom dialect generation & filtering examples library.
//!
//! This library imports custom-built dialects.
//!
//! These dialects are filtered according to settings defined in `[package.metadata.mavspec]` of `Cargo.toml`.

mod mavlink {
    include!(concat!(env!("OUT_DIR"), "/mavlink/mod.rs"));
}
pub use mavlink::dialects;
