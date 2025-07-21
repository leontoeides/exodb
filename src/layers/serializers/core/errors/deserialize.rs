//! Contains the error type returned from the deserialization implementation while deserializing
//! data.

// -------------------------------------------------------------------------------------------------
//
/// An error returned from the deserialization implementation while deserializing data.
///
/// This includes errors for corrupted or malformed data, etc.
#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum Error {
    /// Error returned from [bincode](https://crates.io/crates/bincode)'s decoder.
    ///
    /// To understand the possible errors this deserializer may produce, please refer to the
    /// official documentation: <https://docs.rs/bincode>
    #[cfg(any(feature = "serializer-bincode-native", feature = "serializer-bincode-serde"))]
    #[error("bincode deserialization failed")]
    Bincode { #[from] #[source] source: bitcode::error::DecodeError },

    /// Error returned from the [bitcode](https://crates.io/crates/bitcode) crate.
    ///
    /// To understand the possible errors this serializer may produce, please refer to the official
    /// documentation: <https://docs.rs/bitcode>
    #[cfg(any(feature = "serializer-bitcode-native", feature = "serializer-bitcode-serde"))]
    #[error("bitcode deserialization failed")]
    Bitcode { #[from] #[source] source: bitcode::Error },

    /// Error returned from the [borsh](https://crates.io/crates/borsh) crate.
    ///
    /// To understand the possible errors this serializer may produce, please refer to the official
    /// documentation: <https://docs.rs/borsh>
    #[cfg(feature = "serializer-borsh")]
    #[error("borsh deserialization failed")]
    Borsh { #[from] #[source] source: std::io::Error },

    /// Error returned from [rmp-serde](https://crates.io/crates/rmp-serde)'s decoder.
    ///
    /// To understand the possible errors this serializer may produce, please refer to the official
    /// documentation: <https://docs.rs/rmp-serde>
    #[cfg(feature = "serializer-messagepack")]
    #[error("message-pack deserialization failed")]
    MessagePack { #[from] #[source] source: rmp_serde::decode::Error },

    /// Error returned from the [musli](https://crates.io/crates/musli) crate.
    ///
    /// To understand the possible errors this serializer may produce, please refer to the official
    /// documentation: <https://docs.rs/musli>
    #[cfg(feature = "serializer-musli-descriptive")]
    #[error("müsli descriptive-format deserialization failed")]
    MusliDescriptive { #[from] #[source] source: musli::descriptive::Error },

    /// Error returned from the [musli](https://crates.io/crates/musli) crate.
    ///
    /// To understand the possible errors this serializer may produce, please refer to the official
    /// documentation: <https://docs.rs/musli>
    #[cfg(feature = "serializer-musli-storage")]
    #[error("müsli storage-format deserialization failed")]
    MusliStorage { #[from] #[source] source: musli::storage::Error },

    /// Error returned from the [musli-zerocopy](https://crates.io/crates/musli-zerocopy) crate.
    ///
    /// To understand the possible errors this serializer may produce, please refer to the official
    /// documentation: <https://docs.rs/musli-zerocopy>
    #[cfg(feature = "serializer-musli-zerocopy")]
    #[error("müsli zero-copy deserialization failed")]
    MusliStorage { #[from] #[source] source: musli_zerocopy::Error },

    /// Error returned from the [musli](https://crates.io/crates/musli) crate.
    ///
    /// To understand the possible errors this serializer may produce, please refer to the official
    /// documentation: <https://docs.rs/musli>
    #[cfg(feature = "serializer-musli-wire")]
    #[error("müsli wire-format deserialization failed")]
    MusliStorage { #[from] #[source] source: musli::wire::Error },

    /// Error returned from the [postcard](https://crates.io/crates/postcard) crate.
    ///
    /// To understand the possible errors this serializer may produce, please refer to the official
    /// documentation: <https://docs.rs/postcard>
    #[cfg(feature = "serializer-postcard-serde")]
    #[error("postcard deserialization failed")]
    Postcard { #[from] #[source] source: postcard::Error },

    /// Error returned from the [rkyv](https://crates.io/crates/rkyv) crate.
    ///
    /// To understand the possible errors this serializer may produce, please refer to the official
    /// documentation: <https://docs.rs/rkyv>
    #[cfg(feature = "serializer-rkyv")]
    #[error("rkyv deserialization failed")]
    Rkyv { #[from] #[source] source: rkyv::rancor::Error },

    /// Error returned from the [zerocopy](https://crates.io/crates/zerocopy) crate.
    ///
    /// To understand the possible errors this serializer may produce, please refer to the official
    /// documentation: <https://docs.rs/zerocopy>
    #[cfg(feature = "serializer-zerocopy")]
    #[error("source was improperly aligned, was incorrect size, or contained invalid data")]
    Zerocopy,
}