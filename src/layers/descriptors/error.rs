/// An error returned from the layer value-pipeline implementation.
///
/// This includes errors for out of memory, corrupted or malformed data, etc.
#[derive(thiserror::Error, Debug)]
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
    UnrecognizedLayer(u16),

    /// When reading a value from the database, an unrecognized serialization method was
    /// encountered. This may indicate data corruption or database version incompatibility.
    #[error(
        "unrecognized serialization method: {0:?}, expected: \
        \"0\" for bincode-native, \
        \"8\" for bincode-serde, \
        \"16\" for bitcode-native, \
        \"24\" for bitcode-serde, \
        \"32\" for borsh, \
        \"40\" for musli-descriptive, \
        \"48\" for musli-storage, \
        \"56\" for musli-wire, \
        \"64\" for postcard-serde, \
        \"72\" for rkyv, \
        \"80\" for rmp-serde, or \
        \"88\" for zerocopy"
    )]
    UnrecognizedSerializer(u16),

    /// When reading a value from the database, an unrecognized compression method was
    /// encountered. This may indicate data corruption or database version incompatibility.
    #[error(
        "unrecognized compression method: {0:?}, expected: \
        \"0\" for brotli, \
        \"8\" for bzip2, \
        \"16\" for deflate, \
        \"24\" for gzip, \
        \"32\" for lz4, \
        \"40\" for zlib, or \
        \"48\" for zstd"
    )]
    UnrecognizedCompressor(u16),

    /// When reading a value from the database, an unrecognized encryption method was
    /// encountered. This may indicate data corruption or database version incompatibility.
    #[error(
        "unrecognized encryption method: {0:?}, expected: \
        \"0\" for aes-gcm, or \
        \"8\" for chacha20"
    )]
    UnrecognizedEncryptor(u16),

    /// When reading a value from the database, an unrecognized error correction method was
    /// encountered. This may indicate data corruption or database version incompatibility.
    #[error(
        "unrecognized error correction method: {0:?}, expected: \
        \"0\" for reed-solomon"
    )]
    UnrecognizedCorrector(u16),

    /// When reading a value from the database, an unrecognized direction was encountered. This may
    /// indicate data corruption or database version incompatibility.
    #[error(
        "unrecognized direction: {0:?}, expected: \
        \"0\" for none, \
        \"256\" for read-only, \
        \"512\" for write-only, or \
        \"768\" for both"
    )]
    UnrecognizedDirection(u16),

    /// Set bits were found in the reserved area of the a layer descriptor. This may indicate data
    /// corruption or database version incompatibility.
    ///
    /// # Layer Descriptor
    ///
    /// Represents:
    /// 1. Layer type (for example, serialization, compression, encryption, error correction, etc.)
    /// 2. Implementation type (for example, Brotli, LZ4, Zlib, etc.)
    /// 3. Direction applicatiom (for example, compress on write-only, read data back as compressed)
    ///
    /// The format is `000000DDIIIIILLL` where:
    /// * `LLL` (bits 0-2): Layer type (3 bits = 8 layers types max.)
    /// * `IIIII` (bits 3-7): Implementation (5 bits = 32 implementations per layer max.)
    /// * `DD` (bits 8-9): Direction when applied (2 bits = 4 directions)
    /// * `000000` (bits 10-15): Reserved for future use
    #[error("reserved bits are set in descriptor: 0x{0:x}")]
    ReservationBitsUsed(u16),

    /// When reading a value from the database, an unexpected layer was encountered. This is usually
    /// due to database misconfigurations relating to layer directions.
    #[error("layer type mismatch: expected a {0:?} layer, found a {1:?} layer")]
    LayerMismatch(String, String),
}