#![allow(clippy::doc_markdown)]

//! MessagePack serialization using [Kornel Lesiński](https://github.com/kornelski) and
//! [Evgeny Safronov](https://github.com/3Hren)'s
//! [rmp-serde](https://crates.io/crates/rmp-serde) crate.

mod ordered_when_serialized;
mod serializer;

#[cfg(feature = "serde-safety")]
pub mod serde_safety;

#[cfg(feature = "serde-safety")]
pub use crate::layers::serializers::impls::rmp_serde::serde_safety::SafeForMessagePack;
pub use crate::layers::serializers::impls::rmp_serde::serde_safety::SafeForMessagePack as SafeForSerde;