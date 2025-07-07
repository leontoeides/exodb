//! Trait implementations that tell the system which keys types are safe to use in "ordered"
//! contexts or in "ranged" queries.

use crate::layers::serializers::OrderedWhenSerialized;

/// Marker trait indicating that when `u8` types are serialized by `rmp-serde`, they remain in
/// lexographical order.
impl OrderedWhenSerialized for u8 {}

/// Marker trait indicating that when `u16` types are serialized by `rmp-serde`, they remain in
/// lexographical order.
impl OrderedWhenSerialized for u16 {}

/// Marker trait indicating that when `u32` types are serialized by `rmp-serde`, they remain in
/// lexographical order.
impl OrderedWhenSerialized for u32 {}

/// Marker trait indicating that when `u64` types are serialized by `rmp-serde`, they remain in
/// lexographical order.
impl OrderedWhenSerialized for u64 {}