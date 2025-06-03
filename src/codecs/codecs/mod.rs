//! Implmentations for (or serializers) such as `bincode-native`, `rmp-serde`, etc.

#[cfg(any(
    all(feature = "bincode-native", feature = "bitcode-native"),
    all(feature = "bincode-native", feature = "bitcode-serde"),
    all(feature = "bincode-native", feature = "borsh"),
    all(feature = "bincode-native", feature = "postcard-serde"),
    all(feature = "bincode-native", feature = "rmp-serde"),
    all(feature = "bitcode-native", feature = "bincode-native"),
    all(feature = "bitcode-native", feature = "bitcode-serde"),
    all(feature = "bitcode-native", feature = "borsh"),
    all(feature = "bitcode-native", feature = "postcard-serde"),
    all(feature = "bitcode-native", feature = "rmp-serde"),
    all(feature = "bitcode-serde", feature = "bincode-native"),
    all(feature = "bitcode-serde", feature = "bitcode-native"),
    all(feature = "bitcode-serde", feature = "borsh"),
    all(feature = "bitcode-serde", feature = "postcard-serde"),
    all(feature = "bitcode-serde", feature = "rmp-serde"),
    all(feature = "borsh", feature = "bincode-native"),
    all(feature = "borsh", feature = "bitcode-native"),
    all(feature = "borsh", feature = "bitcode-serde"),
    all(feature = "borsh", feature = "postcard-serde"),
    all(feature = "borsh", feature = "rmp-serde"),
    all(feature = "postcard-serde", feature = "bincode-native"),
    all(feature = "postcard-serde", feature = "bitcode-native"),
    all(feature = "postcard-serde", feature = "bitcode-serde"),
    all(feature = "postcard-serde", feature = "borsh"),
    all(feature = "postcard-serde", feature = "rmp-serde"),
    all(feature = "rmp-serde", feature = "bincode-native"),
    all(feature = "rmp-serde", feature = "bitcode-native"),
    all(feature = "rmp-serde", feature = "bitcode-serde"),
    all(feature = "rmp-serde", feature = "borsh"),
    all(feature = "rmp-serde", feature = "postcard-serde"),
))]
compile_error!(
    "Multiple codec features enabled! Please enable only one of: \
    `bincode-native`, \
    `bitcode-native`, \
    `bitcode-serde`, \
    `borsh`, \
    `postcard`, or \
    `rmp-serde`",
);

#[cfg(feature = "bincode-native")]
mod bincode_native;

#[cfg(feature = "bitcode-native")]
mod bitcode_native;

#[cfg(feature = "bitcode-serde")]
pub mod bitcode_serde;

#[cfg(feature = "borsh")]
mod borsh;

#[cfg(feature = "postcard-serde")]
pub mod postcard_serde;

#[cfg(feature = "rmp-serde")]
pub mod rmp_serde;