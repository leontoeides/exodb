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
        feature = "serialize-bincode-serde",
        feature = "serialize-bitcode-serde",
        feature = "serialize-messagepack",
        feature = "serialize-postcard-serde"
    )
))]
pub use crate::layers::serializers::impls::SafeForSerde;