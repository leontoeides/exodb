//! Serialization implementations for transforming typed data into byte buffers used for storage.

// -------------------------------------------------------------------------------------------------
//
// Common Assets

mod core;
pub use crate::layers::serializers::core::DeserializeError;
pub use crate::layers::serializers::core::Error;
pub use crate::layers::serializers::core::Method;
pub use crate::layers::serializers::core::Serializable;
pub use crate::layers::serializers::core::SerializeError;
pub use crate::layers::serializers::core::OrderedWhenSerialized;
pub use crate::layers::serializers::core::Serializer;

// -------------------------------------------------------------------------------------------------
//
// Serializer Implementations

pub mod impls;

// -------------------------------------------------------------------------------------------------
//
// Serde Safety

#[cfg(all(
    feature = "serde-safety",
    any(
        feature = "serializer-bincode-serde",
        feature = "serializer-bitcode-serde",
        feature = "serializer-messagepack",
        feature = "serializer-postcard-serde"
    )
))]
pub use crate::layers::serializers::impls::SafeForSerde;