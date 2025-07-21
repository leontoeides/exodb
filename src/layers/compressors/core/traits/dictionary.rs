//! The `DictionaryCompressor` trait provides data compression and decompression functionality as
//! part of an optional data processing pipeline.

use crate::layers::compressors::DictionaryBytes;
use crate::layers::core::Bytes;

// -------------------------------------------------------------------------------------------------
//
/// The `DictionaryCompressor` trait provides data compression and decompression functionality as
/// part of an optional data processing pipeline.
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
/// * 'd' lifetime represents a dictionary potentially being borrowed from a `Dictionary` or
///   `DictionaryProvider`.
///
/// # Considerations
///
/// * Corruption detection: Choose compression algorithms that include integrity checks or combine
///   with the error correction layer.
///
/// # Key Concepts
///
/// ## Dictionaries
///
/// Compression dictionaries are optional. They contain common patterns and structures that help
/// achieve better compression ratios for similar data. They're particularly effective for
/// structured data with repeated patterns, field names, or common values.
///
/// ### Benefits of Pre-defined Dictionaries
///
/// Using a well-crafted dictionary can significantly improve compression efficiency by providing
/// the compressor with domain-specific knowledge about your data structure. This is especially
/// valuable for JSON, protocol buffers, or other structured formats where field names and common
/// values repeat frequently.
///
/// ### ⚠️ Dictionary Warnings ⚠️
///
/// * The dictionary is **not** embedded in the compressed data. If the dictionary is lost or
///   modified, compressed data becomes unreadable.
///
/// * Dictionaries must be identical between compression and decompression operations. Dictionary
///   mismatches will cause decompression to fail or produce corrupted data.
///
/// * Dictionaries may contain sensitive information patterns from your data. Handle them with
///   appropriate security measures in your application.
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
pub trait DictionaryCompressor<'b, 'd, V: crate::layers::compressors::Compressible> {
    /// Returns the compression method that the current `DictionaryCompressor` trait implements.
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
    /// * `dictionary` - Optional external dictionary that can give better performance and
    ///   compression ratios. **The dictionary is not stored with the compressed data. If provided,
    ///   the same dictionary must be used to decompress.**
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
    /// * 'd' lifetime represents a key potentially being borrowed from a `Dictionary` or
    ///   `DictionaryProvider`.
    fn compress(
        uncompressed_bytes: Bytes<'b>,
        dictionary: Option<DictionaryBytes<'d>>
    ) -> Result<Bytes<'b>, crate::layers::compressors::CompressError>;

    /// Restores compressed data to its original form, expanding the encoded data to the original
    /// representation.
    ///
    /// # Arguments
    ///
    /// * `compressed_bytes` · The compressed data to be restored to its original form, wrapped in a
    ///   `Bytes` buffer that may reference data borrowed from storage.
    ///
    /// * `dictionary` - Optional external dictionary that can give better performance and
    ///   compression ratios. **Must be identical to the dictionary used during compression or
    ///   decompression will fail.**
    ///
    /// # Errors
    ///
    /// This method may fail for several reasons, including:
    ///
    /// * Input bytes are corrupted or malformed
    /// * Dictionary mismatch (if provided dictionary differs from compression-time dictionary)
    ///
    /// Consult the documentation of the compressor backend you are using for more detail on
    /// decompression behavior and potential limitations.
    ///
    /// # Generics & Lifetimes
    ///
    /// * `V` generic represents the user's value type, for example: `User`, `String`, etc.
    /// * `b` lifetime represents bytes potentially being borrowed from the host application.
    /// * 'd' lifetime represents a key potentially being borrowed from a `Dictionary` or
    ///   `DictionaryProvider`.
    fn decompress(
        compressed_bytes: Bytes<'b>,
        dictionary: Option<DictionaryBytes<'d>>
    ) -> Result<Bytes<'b>, crate::layers::compressors::DecompressError>;
}