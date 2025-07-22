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
    #[cfg(any(feature = "serialize-bincode-native", feature = "serialize-bincode-serde"))]
    #[error("bincode deserialization failed")]
    Bincode { #[from] #[source] source: bincode::error::DecodeError },

    /// Error returned from the [bitcode](https://crates.io/crates/bitcode) crate.
    ///
    /// To understand the possible errors this serializer may produce, please refer to the official
    /// documentation: <https://docs.rs/bitcode>
    #[cfg(any(feature = "serialize-bitcode-native", feature = "serialize-bitcode-serde"))]
    #[error("bitcode deserialization failed")]
    Bitcode { #[from] #[source] source: bitcode::Error },

    /// Error returned from the [borsh](https://crates.io/crates/borsh) crate.
    ///
    /// To understand the possible errors this serializer may produce, please refer to the official
    /// documentation: <https://docs.rs/borsh>
    #[cfg(feature = "serialize-borsh")]
    #[error("borsh deserialization failed")]
    Borsh { #[from] #[source] source: std::io::Error },

    /// Error returned from [rmp-serde](https://crates.io/crates/rmp-serde)'s decoder.
    ///
    /// To understand the possible errors this serializer may produce, please refer to the official
    /// documentation: <https://docs.rs/rmp-serde>
    #[cfg(feature = "serialize-messagepack")]
    #[error("message-pack deserialization failed")]
    MessagePack { #[from] #[source] source: rmp_serde::decode::Error },

    /// Error returned from the [musli](https://crates.io/crates/musli) crate.
    ///
    /// To understand the possible errors this serializer may produce, please refer to the official
    /// documentation: <https://docs.rs/musli>
    #[cfg(feature = "serialize-musli-descriptive")]
    #[error("müsli descriptive-format deserialization failed")]
    MusliDescriptive { #[from] #[source] source: musli::descriptive::Error },

    /// Error returned from the [musli](https://crates.io/crates/musli) crate.
    ///
    /// To understand the possible errors this serializer may produce, please refer to the official
    /// documentation: <https://docs.rs/musli>
    #[cfg(feature = "serialize-musli-storage")]
    #[error("müsli storage-format deserialization failed")]
    MusliStorage { #[from] #[source] source: musli::storage::Error },

    /// Error returned from the [musli-zerocopy](https://crates.io/crates/musli-zerocopy) crate.
    ///
    /// To understand the possible errors this serializer may produce, please refer to the official
    /// documentation: <https://docs.rs/musli-zerocopy>
    #[cfg(feature = "serialize-musli-zerocopy")]
    #[error("müsli zero-copy deserialization failed")]
    MusliStorage { #[from] #[source] source: musli_zerocopy::Error },

    /// Error returned from the [musli](https://crates.io/crates/musli) crate.
    ///
    /// To understand the possible errors this serializer may produce, please refer to the official
    /// documentation: <https://docs.rs/musli>
    #[cfg(feature = "serialize-musli-wire")]
    #[error("müsli wire-format deserialization failed")]
    MusliStorage { #[from] #[source] source: musli::wire::Error },

    /// Error returned from the [postcard](https://crates.io/crates/postcard) crate.
    ///
    /// To understand the possible errors this serializer may produce, please refer to the official
    /// documentation: <https://docs.rs/postcard>
    #[cfg(feature = "serialize-postcard-serde")]
    #[error("postcard deserialization failed")]
    Postcard { #[from] #[source] source: postcard::Error },

    /// Error returned from the [rkyv](https://crates.io/crates/rkyv) crate.
    ///
    /// To understand the possible errors this serializer may produce, please refer to the official
    /// documentation: <https://docs.rs/rkyv>
    #[cfg(feature = "serialize-rkyv")]
    #[error("rkyv deserialization failed")]
    Rkyv { #[from] #[source] source: rkyv::rancor::Error },

    /// Error returned from the [zerocopy](https://crates.io/crates/zerocopy) crate.
    ///
    /// To understand the possible errors this serializer may produce, please refer to the official
    /// documentation: <https://docs.rs/zerocopy>
    #[cfg(feature = "serialize-zerocopy")]
    #[error("source was improperly aligned, was incorrect size, or contained invalid data")]
    Zerocopy,
}