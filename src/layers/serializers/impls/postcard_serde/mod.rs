//! Support for [James Munns](https://github.com/jamesmunns)'
//! [postcard](https://crates.io/crates/postcard) crate's [serde](https://serde.rs/) implementation.

mod serializer;
mod ordered_when_serialized;

#[cfg(feature = "serde-safety")]
pub mod serde_safety;