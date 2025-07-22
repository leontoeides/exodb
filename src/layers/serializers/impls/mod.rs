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
    "serialize-bincode-native",
    "serialize-bincode-serde",
    "serialize-bitcode-native",
    "serialize-bitcode-serde",
    "serialize-borsh",
    "serialize-messagepack",
    "serialize-musli-descriptive",
    "serialize-musli-storage",
    "serialize-musli-wire",
    "serialize-musli-zerocopy",
    "serialize-postcard-serde",
    "serialize-rkyv",
    "serialize-zerocopy"
);

const _: () = {
    assert!(
        // Only one serializer feature can be enabled. To fix: 1. open the `Cargo.toml` file, 2. find the
        // `[dependencies]` section where `atlatl` is declared, 3. ensure only one serializer is enabled.
        !(_SERIALIZER_FEATURE_COUNT > 1),
        "Multiple serializer features enabled! Enable only one of: \
        `serialize-bincode-native`, \
        `serialize-bincode-serde`, \
        `serialize-bitcode-native`, \
        `serialize-bitcode-serde`, \
        `serialize-borsh`, \
        `serialize-messagepack`, \
        `serialize-musli-descriptive`, \
        `serialize-musli-storage`, \
        `serialize-musli-wire`, \
        `serialize-musli-zerocopy`, \
        `serialize-postcard-serde`, \
        `serialize-rkyv`, or \
        `serialize-zerocopy`",
    );
};

// -------------------------------------------------------------------------------------------------
//
// Serializer Implementations

#[cfg(feature = "serialize-bincode-native")]
pub mod bincode_native;

#[cfg(feature = "serialize-bincode-serde")]
pub mod bincode_serde;

#[cfg(feature = "serialize-bitcode-native")]
pub mod bitcode_native;

#[cfg(feature = "serialize-bitcode-serde")]
pub mod bitcode_serde;

#[cfg(feature = "serialize-borsh")]
pub mod borsh;

#[cfg(feature = "serialize-messagepack")]
pub mod rmp_serde;

#[cfg(feature = "serialize-musli-descriptive")]
pub mod musli_descriptive;

#[cfg(feature = "serialize-musli-storage")]
pub mod musli_storage;

#[cfg(feature = "serialize-musli-wire")]
pub mod musli_wire;

#[cfg(feature = "serialize-musli-zerocopy")]
pub mod musli_zerocopy;

#[cfg(feature = "serialize-postcard-serde")]
pub mod postcard_serde;

#[cfg(feature = "serialize-rkyv")]
pub mod rkyv;

#[cfg(feature = "serialize-zerocopy")]
pub mod zerocopy;

// -------------------------------------------------------------------------------------------------
//
// Serde Safety

#[cfg(all(feature = "serialize-bincode-serde", feature = "serde-safety"))]
pub use crate::layers::serializers::impls::bincode_serde::serde_safety::SafeForBincodeSerde as SafeForSerde;

#[cfg(all(feature = "serialize-bitcode-serde", feature = "serde-safety"))]
pub use crate::layers::serializers::impls::bitcode_serde::serde_safety::SafeForBitcodeSerde as SafeForSerde;

#[cfg(all(feature = "serialize-postcard-serde", feature = "serde-safety"))]
pub use crate::layers::serializers::impls::postcard_serde::serde_safety::SafeForPostcardSerde as SafeForSerde;

#[cfg(all(feature = "serialize-messagepack", feature = "serde-safety"))]
pub use crate::layers::serializers::impls::rmp_serde::serde_safety::SafeForMessagePack as SafeForSerde;