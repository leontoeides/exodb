//! Support for [James Munns](https://github.com/jamesmunns)'
//! [postcard](https://crates.io/crates/postcard) crate.

mod serializer;
mod ordered_when_serialized;

#[cfg(feature = "serde-safety")]
pub mod serde_safety;