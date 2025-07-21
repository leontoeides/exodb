//! Contains the error type returned from the layer value-pipeline implementation.

// -------------------------------------------------------------------------------------------------
//
/// An error returned from the layer value-pipeline implementation.
///
/// This includes errors for out of memory, corrupted or malformed data, etc.
#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
#[allow(
    clippy::enum_variant_names,
    reason = "there could be other, non-“unrecognized” variant names in the future"
)]
pub enum Error {
    /// When reading a value from the database, an unrecognized layer type was encountered. This may
    /// indicate data corruption or database version incompatibility.
    #[error(
        "unrecognized layer type: {0:?}, expected: \
        \"0\" for serialization, \
        \"1\" for compression, \
        \"2\" for encryption, or \
        \"3\" for correction"
    )]
    UnrecognizedLayer(u8),

    /// When reading a value from the database, an unrecognized serialization method was
    /// encountered. This may indicate data corruption or database version incompatibility.
    #[error(
        "unrecognized serialization method: {0:?}, expected: \
        \"0\" for bincode-native, \
        \"1\" for bincode-serde, \
        \"2\" for bitcode-native, \
        \"3\" for bitcode-serde, \
        \"4\" for borsh, \
        \"5\" for message-pack, \
        \"6\" for musli-descriptive, \
        \"7\" for musli-storage, \
        \"8\" for musli-wire, \
        \"9\" for musli-zerocopy, \
        \"10\" for postcard-serde, \
        \"11\" for rkyv, or \
        \"12\" for zerocopy"
    )]
    UnrecognizedSerializer(u8),

    /// When reading a value from the database, an unrecognized compression method was
    /// encountered. This may indicate data corruption or database version incompatibility.
    #[error(
        "unrecognized compression method: {0:?}, expected: \
        \"0\" for brotli, \
        \"1\" for bzip2, \
        \"2\" for deflate, \
        \"3\" for gzip, \
        \"4\" for lz4, \
        \"5\" for zlib, or \
        \"6\" for zstd"
    )]
    UnrecognizedCompressor(u8),

    /// When reading a value from the database, an unrecognized encryption method was
    /// encountered. This may indicate data corruption or database version incompatibility.
    #[error(
        "unrecognized encryption method: {0:?}, expected: \
        \"0\" for aes-gcm, or \
        \"1\" for chacha20"
    )]
    UnrecognizedEncryptor(u8),

    /// When reading a value from the database, an unrecognized error correction method was
    /// encountered. This may indicate data corruption or database version incompatibility.
    #[error(
        "unrecognized error correction method: {0:?}, expected: \
        \"0\" for reed-solomon"
    )]
    UnrecognizedCorrector(u8),

    /// When reading a value from the database, an unrecognized direction was encountered. This may
    /// indicate data corruption or database version incompatibility.
    #[error(
        "unrecognized direction: {0:?}, expected: \
        \"0\" for none, \
        \"1\" for on-read, \
        \"2\" for on-write, or \
        \"3\" for both"
    )]
    UnrecognizedDirection(u8),
}