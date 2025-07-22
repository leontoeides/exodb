//! Trait implementations that tell the system which keys types are safe to use in “ordered”
//! contexts or in “ranged” queries.

use crate::layers::serializers::OrderedWhenSerialized;

/// Marker trait indicating that when `u8` values are serialized by `bitcode-native`, they remain in
/// lexographical order.
impl OrderedWhenSerialized<'_> for u8 {}

/// Marker trait indicating that when `NonZeroU8` values are serialized by `bitcode-native`, they
/// remain in lexographical order.
impl OrderedWhenSerialized<'_> for std::num::NonZeroU8 {}