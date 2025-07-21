//! Trait implementations that tell the system which keys types are safe to use in “ordered”
//! contexts or in “ranged” queries.

/// Marker trait indicating that when `bool` values are serialized by `musli-zerocopy` serializer,
/// they remain in lexographical order.
impl crate::layers::serializers::OrderedWhenSerialized<'_> for bool {}

/// Marker trait indicating that when `char` values are serialized by `musli-zerocopy` serializer,
/// they remain in lexographical order.
impl crate::layers::serializers::OrderedWhenSerialized<'_> for char {}

/// Marker trait indicating that when `u8` values are serialized by `musli-zerocopy` serializer,
/// they remain in lexographical order.
impl crate::layers::serializers::OrderedWhenSerialized<'_> for u8 {}

/// Marker trait indicating that when `NonZeroU8` values are serialized by `musli-zerocopy`
/// serializer, they remain in lexographical order.
impl crate::layers::serializers::OrderedWhenSerialized<'_> for std::num::NonZeroU8 {}