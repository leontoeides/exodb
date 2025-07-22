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
        feature = "serialize-bincode-serde",
        feature = "serialize-bitcode-serde",
        feature = "serialize-messagepack",
        feature = "serialize-postcard-serde"
    )
))]
pub use crate::layers::serializers::impls::SafeForSerde;