//! The `Serializer` traits provide a set of common interfaces for serializing and deserializing
//! data.

// Standard Serializer

#[cfg(not(feature = "serialize-rkyv"))]
pub mod standard;

#[cfg(not(feature = "serialize-rkyv"))]
pub use crate::layers::serializers::core::traits::standard::OrderedWhenSerialized;

#[cfg(not(feature = "serialize-rkyv"))]
pub use crate::layers::serializers::core::traits::standard::Serializer;

// Rkyv Serializer

#[cfg(feature = "serialize-rkyv")]
pub mod rkyv;

#[cfg(feature = "serialize-rkyv")]
pub use crate::layers::serializers::core::traits::rkyv::OrderedWhenSerialized;

#[cfg(feature = "serialize-rkyv")]
pub use crate::layers::serializers::core::traits::rkyv::Serializer;