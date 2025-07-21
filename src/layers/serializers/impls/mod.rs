//! Serialization implementations for transforming typed data into byte buffers used for storage.

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
    "serializer-messagepack",
    "serializer-musli-descriptive",
    "serializer-musli-storage",
    "serializer-musli-wire",
    "serializer-musli-zerocopy",
    "serializer-postcard-serde",
    "serializer-rkyv",
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
        `serializer-messagepack`, \
        `serializer-musli-descriptive`, \
        `serializer-musli-storage`, \
        `serializer-musli-wire`, \
        `serializer-musli-zerocopy`, \
        `serializer-postcard-serde`, \
        `serializer-rkyv`, or \
        `serializer-zerocopy`",
    );
};

// -------------------------------------------------------------------------------------------------
//
// Serializer Implementations

#[cfg(feature = "serializer-bincode-native")]
pub mod bincode_native;

#[cfg(feature = "serializer-bincode-serde")]
pub mod bincode_serde;

#[cfg(feature = "serializer-bitcode-native")]
pub mod bitcode_native;

#[cfg(feature = "serializer-bitcode-serde")]
pub mod bitcode_serde;

#[cfg(feature = "serializer-borsh")]
pub mod borsh;

#[cfg(feature = "serializer-messagepack")]
pub mod rmp_serde;

#[cfg(feature = "serializer-musli-descriptive")]
pub mod musli_descriptive;

#[cfg(feature = "serializer-musli-storage")]
pub mod musli_storage;

#[cfg(feature = "serializer-musli-wire")]
pub mod musli_wire;

#[cfg(feature = "serializer-musli-zerocopy")]
pub mod musli_zerocopy;

#[cfg(feature = "serializer-postcard-serde")]
pub mod postcard_serde;

#[cfg(feature = "serializer-rkyv")]
pub mod rkyv;

#[cfg(feature = "serializer-zerocopy")]
pub mod zerocopy;

// -------------------------------------------------------------------------------------------------
//
// Serde Safety

#[cfg(all(feature = "serializer-bincode-serde", feature = "serde-safety"))]
pub use crate::layers::serializers::impls::bincode_serde::serde_safety::SafeForBincodeSerde as SafeForSerde;

#[cfg(all(feature = "serializer-bitcode-serde", feature = "serde-safety"))]
pub use crate::layers::serializers::impls::bitcode_serde::serde_safety::SafeForBitcodeSerde as SafeForSerde;

#[cfg(all(feature = "serializer-postcard-serde", feature = "serde-safety"))]
pub use crate::layers::serializers::impls::postcard_serde::serde_safety::SafeForPostcardSerde as SafeForSerde;

#[cfg(all(feature = "serializer-messagepack", feature = "serde-safety"))]
pub use crate::layers::serializers::impls::rmp_serde::serde_safety::SafeForMessagePack as SafeForSerde;