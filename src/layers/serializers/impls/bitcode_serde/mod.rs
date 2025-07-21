//! Support for [Finn Bear](https://github.com/finnbear) and
//! [Cai Bear](https://github.com/caibear)'s [bitcode](https://crates.io/crates/bitcode) crate's
//! [serde](https://serde.rs/) implementation.

mod serializer;
mod ordered_when_serialized;

#[cfg(feature = "serde-safety")]
pub mod serde_safety;