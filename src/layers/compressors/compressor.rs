use crate::layers::ValueBuf;

// -------------------------------------------------------------------------------------------------
//
/// The `Compressor` trait provides data compression and decompression functionality as part of an
/// optional data processing pipeline. This pipeline can include: disk storage → ECC repair →
/// decryption → decompression → deserialization (for reads) or the reverse for writes, with each
/// stage being optional and potentially zero-copy.
///
/// # Lifetimes
///
/// * The `b` lifetime represents bytes potentially being borrowed from the `redb` database.
///
/// # Key Concepts
///
/// ## Pipeline Integration
///
/// This trait operates within a larger data processing pipeline where compression is just one
/// optional stage. The zero-copy design allows for efficient processing of large datasets while
/// maintaining security properties throughout the pipeline.
///
/// ## ValueBuf and Zero-Copy Design
///
/// The `ValueBuf<'b>` wrapper allows the compression layer to work with borrowed data from the
/// database without unnecessary copying, improving performance for large datasets.
///
/// # Considerations
///
/// * Corruption detection: Choose compression algorithms that include integrity checks or combine
///   with the error correction layer.
pub trait Compressor<'b, V: crate::layers::compressors::Compressible> {
    /// Reduces data size by eliminating redundancy and using efficient encoding, operating on
    /// serialized bytes to create smaller representations.
    ///
    /// # Arguments
    ///
    /// * `uncompressed_bytes` · The original serialized data to be compressed, wrapped in a
    ///   `ValueBuf` that may reference borrowed database bytes.
    ///
    /// # Errors
    ///
    /// Consult the documentation of the compressor backend you are using for more detail on
    /// compression behavior and potential limitations.
    fn compress(
        uncompressed_bytes: ValueBuf<'b>
    ) -> Result<ValueBuf<'b>, crate::layers::compressors::CompressError>;

    /// Restores compressed data to its original form, expanding the efficient encoding back to the
    /// full serialized bytes.
    ///
    /// # Arguments
    ///
    /// * `compressed_bytes` · The compressed data to be restored to its original form, wrapped in a
    ///   `ValueBuf`.
    ///
    /// # Lifetimes
    ///
    /// * The `b` lifetime represents bytes potentially being borrowed from the `redb` database.
    ///
    /// # Errors
    ///
    /// This method may fail for several reasons, including:
    ///
    /// * Input bytes are corrupted or malformed.
    ///
    /// Consult the documentation of the compressor backend you are using for more detail on
    /// decompression behavior and potential limitations.
    fn decompress(
        compressed_bytes: ValueBuf<'b>
    ) -> Result<ValueBuf<'b>, crate::layers::compressors::DecompressError>;

    /// Returns the compression method that the current `Compressor` trait implements.
    ///
    /// This enables runtime identification of the compression algorithm in use, allowing
    /// applications to log compression details, or store metadata about how data was processed in
    /// the data pipeline.
    fn method() -> &'static crate::layers::compressors::Method;
}