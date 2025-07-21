//! Compression implementation with support for dictionaries.

use crate::layers::compressors::impls::zstd::{compression_level, MAX_CAPACITY, Zstd};
use crate::layers::compressors::{Compressible, Compressor, DictionaryBytes, Method};
use crate::layers::core::Bytes;

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl<'b, 'd, V: Compressible> Compressor<'b, 'd, V> for Zstd<V> {
    /// Returns the compression method that the current `Compressor` trait implements.
    ///
    /// This enables runtime identification of the compression algorithm in use, allowing
    /// applications to log compression details, or store metadata about how data was processed in
    /// the data pipeline.
    const METHOD: &'static Method = &Method::Zstd;

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
    /// compression behavior and potential limitations: <https://docs.rs/flate2>
    fn compress(
        uncompressed_bytes: Bytes<'b>,
        dictionary: Option<DictionaryBytes<'d>>
    ) -> Result<Bytes<'b>, crate::layers::compressors::CompressError> {
        if let Some(dictionary) = dictionary {
            let mut compressor = zstd::bulk::Compressor::new(compression_level::<V>())?;
            compressor.set_prepared_dictionary(dictionary.as_encoder_dict())?;
            let compressed_bytes = compressor.compress(uncompressed_bytes.as_slice())?;
            Ok(compressed_bytes.into())
        } else {
            let compressed_bytes = zstd::bulk::compress(
                uncompressed_bytes.as_slice(),
                compression_level::<V>()
            )?;
            Ok(compressed_bytes.into())
        }
    }

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
    /// decompression behavior and potential limitations: <https://docs.rs/flate2>
    fn decompress(
        compressed_bytes: Bytes<'b>,
        dictionary: Option<DictionaryBytes<'d>>
    ) -> Result<Bytes<'b>, crate::layers::compressors::DecompressError> {
        if let Some(dictionary) = dictionary {
            let mut compressor = zstd::bulk::Decompressor::with_prepared_dictionary(
                dictionary.as_decoder_dict()
            )?;
            let decompressed_bytes = compressor.decompress(
                compressed_bytes.as_slice(),
                MAX_CAPACITY
            )?;
            Ok(decompressed_bytes.into())
        } else {
            Ok(zstd::bulk::decompress(compressed_bytes.as_ref(), MAX_CAPACITY)?.into())
        }
    }
}