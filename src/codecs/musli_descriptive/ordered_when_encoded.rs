//! Trait implementations that tell the system which keys types are safe to use in "ordered"
//! contexts or in "ranged" queries.

/// Marker trait indicating that when `u8` types are encoded by the `musli-descriptive` codec, they
/// remain in lexographical order.
impl crate::codecs::OrderedWhenEncoded for u8 {}