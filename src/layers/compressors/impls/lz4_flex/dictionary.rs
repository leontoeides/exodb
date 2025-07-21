//! Compression implementation with support for dictionaries.

use crate::layers::compressors::impls::lz4_flex::Lz4Flex;
use crate::layers::compressors::{Compressible, Compressor, DictionaryBytes, Method};
use crate::layers::core::Bytes;

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl<'b, 'd, V: Compressible> Compressor<'b, 'd, V> for Lz4Flex<V> {
    /// Returns the compression method that the current `Compressor` trait implements.
    ///
    /// This enables runtime identification of the compression algorithm in use, allowing
    /// applications to log compression details, or store metadata about how data was processed in
    /// the data pipeline.
    const METHOD: &'static Method = &Method::Lz4;

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
    /// compression behavior and potential limitations: <https://docs.rs/lz4_flex>
    fn compress(
        uncompressed_bytes: Bytes<'b>,
        dictionary: Option<DictionaryBytes<'d>>
    ) -> Result<Bytes<'b>, crate::layers::compressors::CompressError> {
        dictionary.map_or_else(
            || Ok(lz4_flex::block::compress_prepend_size(
                uncompressed_bytes.as_slice()
            ).into()),
            |dictionary| Ok(lz4_flex::block::compress_prepend_size_with_dict(
                uncompressed_bytes.as_slice(),
                dictionary.into()
            ).into())
        )
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
    /// decompression behavior and potential limitations: <https://docs.rs/lz4_flex>
    fn decompress(
        compressed_bytes: Bytes<'b>,
        dictionary: Option<DictionaryBytes<'d>>
    ) -> Result<Bytes<'b>, crate::layers::compressors::DecompressError> {
        let (metadata, compressed_bytes) = compressed_bytes.into_parts();
        let decompressed_bytes: Vec<u8> = dictionary.map_or_else(
            || lz4_flex::block::decompress_size_prepended(
                compressed_bytes.as_ref()
            ),
            |dictionary| lz4_flex::block::decompress_size_prepended_with_dict(
                compressed_bytes.as_ref(),
                dictionary.into()
            )
        )?;
        Ok(Bytes::from_parts(metadata, decompressed_bytes.into()))
    }
}