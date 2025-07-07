//! Serialization implementations for transforming typed data into byte buffers used for storage.

mod deserialize_error;
pub use crate::layers::serializers::deserialize_error::DeserializeError;

mod error;
pub use crate::layers::serializers::error::Error;

mod method;
pub use crate::layers::serializers::method::Method;

mod ordered_when_serialized;
pub use crate::layers::serializers::ordered_when_serialized::OrderedWhenSerialized;

mod serializable;
pub use crate::layers::serializers::serializable::Serializable;

mod serialize_error;
pub use crate::layers::serializers::serialize_error::SerializeError;

mod serializer;
pub use crate::layers::serializers::serializer::Serializer;

// -------------------------------------------------------------------------------------------------
//
// Serializer Feature Guard

/// Helper macro: counts how many of the listed features are turned on.
macro_rules! count_features {
    ($($feat:literal),* $(,)?) => {
        0_usize $(+ cfg!(feature = $feat) as usize)*
    };
}

const _SERIALIZER_FEATURE_COUNT: usize = count_features!(
    "serializer-bincode-native",
    "serializer-bincode-serde",
    "serializer-bitcode-native",
    "serializer-bitcode-serde",
    "serializer-borsh",
    "serializer-musli-descriptive",
    "serializer-musli-storage",
    "serializer-musli-wire",
    "serializer-postcard-serde",
    "serializer-rkyv",
    "serializer-rmp-serde",
    "serializer-zerocopy"
);

const _: () = {
    assert!(
        // Only one serializer feature can be enabled. To fix: 1. open the `Cargo.toml` file, 2. find the
        // `[dependencies]` section where `atlatl` is declared, 3. ensure only one serializer is enabled.
        !(_SERIALIZER_FEATURE_COUNT > 1),
        "Multiple serializer features enabled! Enable only one of: \
        `serializer-bincode-native`, \
        `serializer-bincode-serde`, \
        `serializer-bitcode-native`, \
        `serializer-bitcode-serde`, \
        `serializer-borsh`, \
        `serializer-musli-descriptive`, \
        `serializer-musli-storage`, \
        `serializer-musli-wire`, \
        `serializer-postcard-serde`, \
        `serializer-rkyv`, \
        `serializer-rmp-serde`, or \
        `serializer-zerocopy`",
    );
};

// -------------------------------------------------------------------------------------------------
//
// Serializer Implementations

#[cfg(feature = "serializer-bincode-native")]
mod bincode_native;

#[cfg(feature = "serializer-bincode-serde")]
pub mod bincode_serde;

#[cfg(feature = "serializer-bitcode-native")]
mod bitcode_native;

#[cfg(feature = "serializer-bitcode-serde")]
pub mod bitcode_serde;

#[cfg(feature = "serializer-borsh")]
mod borsh;

#[cfg(feature = "serializer-musli-descriptive")]
mod musli_descriptive;

#[cfg(feature = "serializer-musli-storage")]
mod musli_storage;

#[cfg(feature = "serializer-musli-wire")]
mod musli_wire;

#[cfg(feature = "serializer-postcard-serde")]
pub mod postcard_serde;

#[cfg(feature = "serializer-rkyv")]
mod rkyv;

#[cfg(feature = "serializer-rmp-serde")]
pub mod rmp_serde;

#[cfg(feature = "serializer-zerocopy")]
mod zerocopy;

// -------------------------------------------------------------------------------------------------
//
// Serde Safety

#[cfg(all(feature = "serializer-bincode-serde", feature = "serde-safety"))]
pub use crate::layers::serializers::bincode_serde::serde_safety::SafeForBincodeSerde;

#[cfg(all(feature = "serializer-bitcode-serde", feature = "serde-safety"))]
pub use crate::layers::serializers::bitcode_serde::serde_safety::SafeForBitcodeSerde;

#[cfg(all(feature = "serializer-postcard-serde", feature = "serde-safety"))]
pub use crate::layers::serializers::postcard_serde::serde_safety::SafeForPostcardSerde;

#[cfg(all(feature = "serializer-rmp-serde", feature = "serde-safety"))]
pub use crate::layers::serializers::rmp_serde::serde_safety::SafeForRmpSerde;