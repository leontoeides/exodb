//! Support for [Ty Overby](https://github.com/TyOverby), [Zoey Riordan](https://github.com/ZoeyR)
//! and [Victor Koenders](https://github.com/VictorKoenders)'s
//! [bincode](https://crates.io/crates/bincode) crate's [serde](https://serde.rs/) implementation.

mod serializer;
mod ordered_when_serialized;

#[cfg(feature = "serde-safety")]
pub mod serde_safety;