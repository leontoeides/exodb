/// Error returned from the codec or deserializer. This includes errors for corrupted or malformed
/// data, unexpected types, etc.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Error returned from [bincode](https://crates.io/crates/bincode)'s decoder.
    ///
    /// To understand the possible errors this codec may produce, please refer to the official
    /// documentation: <https://docs.rs/bincode>
    #[cfg(any(feature = "bincode-native", feature = "bincode-serde"))]
    #[error(transparent)]
    BincodeDecode(#[from] bincode::error::DecodeError),

    /// Error returned from [bincode](https://crates.io/crates/bincode)'s decoder.
    ///
    /// To understand the possible errors this codec may produce, please refer to the official
    /// documentation: <https://docs.rs/bincode>
    #[cfg(any(feature = "bincode-native", feature = "bincode-serde"))]
    #[error(transparent)]
    BincodeEncode(#[from] bincode::error::EncodeError),

    /// Error returned from the [bitcode](https://crates.io/crates/bitcode) crate.
    ///
    /// To understand the possible errors this codec may produce, please refer to the official
    /// documentation: <https://docs.rs/bitcode>
    #[cfg(any(feature = "bitcode-native", feature = "bitcode-serde"))]
    #[error(transparent)]
    Bitcode(#[from] bitcode::Error),

    /// Error returned from the [borsh](https://crates.io/crates/borsh) crate.
    #[cfg(feature = "borsh")]
    #[error(transparent)]
    Borsh(#[from] std::io::Error),

    /// Error returned from the [musli](https://crates.io/crates/musli) crate.
    ///
    /// To understand the possible errors this encoder may produce, please refer to the official
    /// documentation: <https://docs.rs/musli>
    #[cfg(feature = "musli-descriptive")]
    #[error(transparent)]
    MusliDescriptive(#[from] musli::descriptive::Error),

    /// Error returned from the [musli](https://crates.io/crates/musli) crate.
    ///
    /// To understand the possible errors this encoder may produce, please refer to the official
    /// documentation: <https://docs.rs/musli>
    #[cfg(feature = "musli-storage")]
    #[error(transparent)]
    MusliStorage(#[from] musli::storage::Error),

    /// Error returned from the [musli](https://crates.io/crates/musli) crate.
    ///
    /// To understand the possible errors this codec may produce, please refer to the official
    /// documentation: <https://docs.rs/musli>
    #[cfg(feature = "musli-wire")]
    #[error(transparent)]
    MusliWire(#[from] musli::wire::Error),

    /// Error returned from the [postcard](https://crates.io/crates/postcard) crate.
    ///
    /// To understand the possible errors this codec may produce, please refer to the official
    /// documentation: <https://docs.rs/postcard>
    #[cfg(feature = "postcard-serde")]
    #[error(transparent)]
    Postcard(#[from] postcard::Error),

    /// Error returned from the [rkyv](https://crates.io/crates/rkyv) crate.
    ///
    /// To understand the possible errors this codec may produce, please refer to the official
    /// documentation: <https://docs.rs/rkyv>
    #[cfg(feature = "rkyv")]
    #[error(transparent)]
    Rkyv(#[from] rkyv::rancor::Error),

    /// Error returned from [rmp-serde](https://crates.io/crates/rmp-serde)'s decoder.
    ///
    /// To understand the possible errors this codec may produce, please refer to the official
    /// documentation: <https://docs.rs/rmp-serde>
    #[cfg(feature = "rmp-serde")]
    #[error(transparent)]
    RmpSerdeDecode(#[from] rmp_serde::decode::Error),

    /// Error returned from [rmp-serde](https://crates.io/crates/rmp-serde)'s encoder.
    ///
    /// To understand the possible errors this codec may produce, please refer to the official
    /// documentation: <https://docs.rs/rmp-serde>
    #[cfg(feature = "rmp-serde")]
    #[error(transparent)]
    RmpSerdeEncode(#[from] rmp_serde::encode::Error),

    // Error returned from the [zerocopy](https://crates.io/crates/zerocopy) crate.
    ///
    /// To understand the possible errors this codec may produce, please refer to the official
    /// documentation: <https://docs.rs/zerocopy>
    #[error("source was improperly aligned, was incorrect size, or contained invalid data")]
    Zerocopy,
}