//! Trait implementations that tell the system which keys types are safe to use in "ordered"
//! contexts or in "ranged" queries.

use crate::codecs::OrderedWhenEncoded;

/// Marker trait indicating that when `u8` types are encoded by the `rmp-serde` codec, they remain
/// in lexographical order.
impl OrderedWhenEncoded for u8 {}

/// Marker trait indicating that when `u16` types are encoded by the `rmp-serde` codec, they remain
/// in lexographical order.
impl OrderedWhenEncoded for u16 {}

/// Marker trait indicating that when `u32` types are encoded by the `rmp-serde` codec, they remain
/// in lexographical order.
impl OrderedWhenEncoded for u32 {}

/// Marker trait indicating that when `u64` types are encoded by the `rmp-serde` codec, they remain
/// in lexographical order.
impl OrderedWhenEncoded for u64 {}