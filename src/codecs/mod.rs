// mod utils;
mod codec;
mod error;
mod ordered_when_encoded;

pub use crate::codecs::codec::Codec;
pub use crate::codecs::error::Error;
pub use crate::codecs::ordered_when_encoded::OrderedWhenEncoded;

// -------------------------------------------------------------------------------------------------
//
// Codec Feature Guard

/// Helper macro: counts how many of the listed features are turned on.
macro_rules! count_features {
    ($($feat:literal),* $(,)?) => {
        0_usize $(+ cfg!(feature = $feat) as usize)*
    };
}

const _CODEC_FEATURE_COUNT: usize = count_features!(
    "bincode-native",
    "bincode-serde",
    "bitcode-native",
    "bitcode-serde",
    "borsh",
    "musli-descriptive",
    "musli-storage",
    "musli-wire",
    "postcard-serde",
    "rkyv",
    "rmp-serde",
    "zerocopy"
);

const _: () = {
    assert!(
        // Only one codec feature can be enabled. To fix: 1. open your `Cargo.toml` file, 2. find the
        // `[dependencies]` section where `exodb` is declared, 3. ensure only one codec feature is enabled.
        !(_CODEC_FEATURE_COUNT > 1),
        "Multiple codec features enabled! Please enable only one of: \
        `bincode-native`, \
        `bincode-serde`, \
        `bitcode-native`, \
        `bitcode-serde`, \
        `borsh`, \
        `musli-descriptive`, \
        `musli-storage`, \
        `musli-wire`, \
        `postcard-serde`, \
        `rkyv`, \
        `rmp-serde`, or \
        `zerocopy`",
    );
};

// -------------------------------------------------------------------------------------------------
//
// Codec Implementations

#[cfg(feature = "bincode-native")]
mod bincode_native;

#[cfg(feature = "bincode-serde")]
pub mod bincode_serde;

#[cfg(feature = "bitcode-native")]
mod bitcode_native;

#[cfg(feature = "bitcode-serde")]
pub mod bitcode_serde;

#[cfg(feature = "borsh")]
mod borsh;

#[cfg(feature = "musli-storage")]
mod musli_storage;

#[cfg(feature = "musli-wire")]
mod musli_wire;

#[cfg(feature = "musli-descriptive")]
mod musli_descriptive;

#[cfg(feature = "postcard-serde")]
pub mod postcard_serde;

#[cfg(feature = "rkyv")]
mod rkyv;

#[cfg(feature = "rmp-serde")]
pub mod rmp_serde;

#[cfg(feature = "zerocopy")]
mod zerocopy;

// -------------------------------------------------------------------------------------------------
//
// Serde Safety

#[cfg(all(feature = "bincode-serde", feature = "serde-safety"))]
pub use crate::codecs::bincode_serde::serde_safety::SafeForBincodeSerde;

#[cfg(all(feature = "bitcode-serde", feature = "serde-safety"))]
pub use crate::codecs::bitcode_serde::serde_safety::SafeForBitcodeSerde;

#[cfg(all(feature = "postcard-serde", feature = "serde-safety"))]
pub use crate::codecs::postcard_serde::serde_safety::SafeForPostcardSerde;

#[cfg(all(feature = "rmp-serde", feature = "serde-safety"))]
pub use crate::codecs::rmp_serde::serde_safety::SafeForRmpSerde;