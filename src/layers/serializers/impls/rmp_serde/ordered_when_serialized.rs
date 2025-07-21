//! Trait implementations that tell the system which keys types are safe to use in “ordered”
//! contexts or in “ranged” queries.

use crate::layers::serializers::OrderedWhenSerialized;

/// Marker trait indicating that when `bool` values are serialized by `rmp-serde`, they remain in
/// lexographical order.
impl OrderedWhenSerialized<'_> for bool {}

/// Marker trait indicating that when `char` values are serialized by `rmp-serde`, they remain in
/// lexographical order.
impl OrderedWhenSerialized<'_> for char {}

/// Marker trait indicating that when `u8` values are serialized by `rmp-serde`, they remain in
/// lexographical order.
impl OrderedWhenSerialized<'_> for u8 {}

/// Marker trait indicating that when `u16` values are serialized by `rmp-serde`, they remain in
/// lexographical order.
impl OrderedWhenSerialized<'_> for u16 {}

/// Marker trait indicating that when `u32` values are serialized by `rmp-serde`, they remain in
/// lexographical order.
impl OrderedWhenSerialized<'_> for u32 {}

/// Marker trait indicating that when `u64` values are serialized by `rmp-serde`, they remain in
/// lexographical order.
impl OrderedWhenSerialized<'_> for u64 {}

/// Marker trait indicating that when `u128` values are serialized by `rmp-serde`, they remain in
/// lexographical order.
impl OrderedWhenSerialized<'_> for u128 {}

/// Marker trait indicating that when `usize` values are serialized by `rmp-serde`, they remain in
/// lexographical order.
impl OrderedWhenSerialized<'_> for usize {}

/// Marker trait indicating that when `NonZeroU8` values are serialized by `rmp-serde`, they remain
/// in lexographical order.
impl OrderedWhenSerialized<'_> for std::num::NonZeroU8 {}

/// Marker trait indicating that when `NonZeroU16` values are serialized by `rmp-serde`, they remain
/// in lexographical order.
impl OrderedWhenSerialized<'_> for std::num::NonZeroU16 {}

/// Marker trait indicating that when `NonZeroU32` values are serialized by `rmp-serde`, they remain
/// in lexographical order.
impl OrderedWhenSerialized<'_> for std::num::NonZeroU32 {}

/// Marker trait indicating that when `NonZeroU64` values are serialized by `rmp-serde`, they remain
/// in lexographical order.
impl OrderedWhenSerialized<'_> for std::num::NonZeroU64 {}

/// Marker trait indicating that when `NonZeroU128` values are serialized by `rmp-serde`, they
/// remain in lexographical order.
impl OrderedWhenSerialized<'_> for std::num::NonZeroU128 {}

/// Marker trait indicating that when `NonZeroUsize` values are serialized by `rmp-serde`, they
/// remain in lexographical order.
impl OrderedWhenSerialized<'_> for std::num::NonZeroUsize {}
