//! The `SerializerStandard` trait provides an interface for standard serialization schemes.

// Exports

// mod ordering_discovery_tests; // Used in development only.

pub mod ordered_when_serialized;
pub use crate::layers::serializers::core::traits::standard::ordered_when_serialized::OrderedWhenSerialized;

pub mod serializer;
pub use crate::layers::serializers::core::traits::standard::serializer::StandardSerializer as Serializer;