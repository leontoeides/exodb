//! Serialization implementations for transforming typed data into byte buffers used for storage.

mod errors;
pub use crate::layers::serializers::core::errors::DeserializeError;
pub use crate::layers::serializers::core::errors::Error;
pub use crate::layers::serializers::core::errors::SerializeError;

mod method;
pub use crate::layers::serializers::core::method::Method;

mod serializable;
pub use crate::layers::serializers::core::serializable::Serializable;

// -------------------------------------------------------------------------------------------------
//
// Serializer Traits

mod traits;
pub use crate::layers::serializers::core::traits::OrderedWhenSerialized;
pub use crate::layers::serializers::core::traits::Serializer;

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