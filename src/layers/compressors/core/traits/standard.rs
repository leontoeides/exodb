//! The `StandardCompressor` trait provides data compression and decompression functionality as part of an
//! optional data processing pipeline.

use crate::layers::core::Bytes;

// -------------------------------------------------------------------------------------------------
//
/// The `StandardCompressor` trait provides data compression and decompression functionality as part
/// of an optional data processing pipeline.
///
/// This pipeline can include: disk storage → ECC repair → decryption → decompression →
/// deserialization (for reads) or the reverse for writes, with each stage being optional and
/// potentially zero-copy.
///
/// # Generics & Lifetimes
///
/// * `V` generic represents the user's value type, for example: `User`, `String`, etc.
/// * `b` lifetime represents bytes potentially being borrowed from the host application or the
///   `redb` database.
///
/// # Considerations
///
/// * Corruption detection: Choose compression algorithms that include integrity checks or combine
///   with the error correction layer.
///
/// # Key Concepts
///
/// ## Pipeline Integration
///
/// This trait operates within a larger data processing pipeline where compression is just one
/// optional stage. The zero-copy design allows for efficient processing of large datasets while
/// maintaining security properties throughout the pipeline.
///
/// ## `Bytes` and Zero-Copy Design
///
/// The `Bytes<'b>` wrapper allows the compression layer to work with borrowed data from the
/// database without unnecessary copying, improving performance for large datasets.
pub trait StandardCompressor<'b, V: crate::layers::compressors::Compressible> {
    /// Returns the compression method that the current `StandardCompressor` trait implements.
    ///
    /// This enables runtime identification of the compression algorithm in use, allowing
    /// applications to log compression details, or store metadata about how data was processed in
    /// the data pipeline.
    const METHOD: &'static crate::layers::compressors::Method;

    /// Reduces the size of data by identifying and eliminating redundancy, creating smaller
    /// representations of bytes, and allowing for more efficient storage or transmission.
    ///
    /// # Arguments
    ///
    /// * `uncompressed_bytes` · The original serialized data to be compressed, wrapped in a
    ///   `Bytes` buffer that may reference data borrowed from the host application.
    ///
    /// # Errors
    ///
    /// Consult the documentation of the compressor backend you are using for more detail on
    /// compression behavior and potential limitations.
    ///
    /// # Generics & Lifetimes
    ///
    /// * `V` generic represents the user's value type, for example: `User`, `String`, etc.
    /// * `b` lifetime represents bytes potentially being borrowed from the host application.
    fn compress(
        uncompressed_bytes: Bytes<'b>
    ) -> Result<Bytes<'b>, crate::layers::compressors::CompressError>;

    /// Restores compressed data to its original form, expanding the encoded data to the original
    /// representation.
    ///
    /// # Arguments
    ///
    /// * `compressed_bytes` · The compressed data to be restored to its original form, wrapped in a
    ///   `Bytes` buffer that may reference data borrowed from storage.
    ///
    /// # Errors
    ///
    /// This method may fail for several reasons, including:
    ///
    /// * Input bytes are corrupted or malformed
    ///
    /// Consult the documentation of the compressor backend you are using for more detail on
    /// decompression behavior and potential limitations.
    ///
    /// # Generics & Lifetimes
    ///
    /// * `V` generic represents the user's value type, for example: `User`, `String`, etc.
    /// * `b` lifetime represents bytes potentially being borrowed from the host application.
    fn decompress(
        compressed_bytes: Bytes<'b>
    ) -> Result<Bytes<'b>, crate::layers::compressors::DecompressError>;
}