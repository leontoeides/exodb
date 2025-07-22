//! Trait implementations that tell the system which keys types are safe to use in “ordered”
//! contexts or in “ranged” queries.

use crate::layers::serializers::OrderedWhenSerialized;

/// Marker trait indicating that when `u8` values are serialized by `bincode-serde`, they remain in
/// lexographical order.
impl OrderedWhenSerialized<'_> for u8 {}

/// Marker trait indicating that when `u64` values are serialized by `bincode-serde`, they remain in
/// lexographical order.
impl OrderedWhenSerialized<'_> for u64 {}

/// Marker trait indicating that when `u128` values are serialized by `bincode-serde`, they remain
/// in lexographical order.
impl OrderedWhenSerialized<'_> for u128 {}

/// Marker trait indicating that when `usize` values are serialized by `bincode-serde`, they remain
/// in lexographical order.
impl OrderedWhenSerialized<'_> for usize {}