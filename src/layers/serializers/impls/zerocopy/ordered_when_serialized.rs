//! Trait implementations that tell the system which keys types are safe to use in “ordered”
//! contexts or in “ranged” queries.

/// Marker trait indicating that when `u8` values are serialized by `zerocopy`, they remain in
/// lexographical order.
impl crate::layers::serializers::OrderedWhenSerialized<'_> for u8 {}