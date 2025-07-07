//! Marker trait for types whose serialized form preserves their natural ordering.

// -------------------------------------------------------------------------------------------------
//
/// Marker trait for types whose serialized form preserves their natural ordering.
///
/// For example, `u64` encoded in big-endian byte order will sort correctly lexicographically. Types
/// implementing this trait are safe to use in `redb` range queries.
pub trait OrderedWhenSerialized: for<'b> crate::layers::Serializer<'b, Self> + Sized {}