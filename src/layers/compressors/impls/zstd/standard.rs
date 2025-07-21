//! Standard compression implementation with no support for dictionaries.

use crate::layers::compressors::impls::zstd::{compression_level, MAX_CAPACITY, Zstd};
use crate::layers::compressors::{Compressible, Compressor, Method};
use crate::layers::core::Bytes;

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl<'b, V: Compressible> Compressor<'b, V> for Zstd<V> {
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
    /// # Errors
    ///
    /// Consult the documentation of the compressor backend you are using for more detail on
    /// compression behavior and potential limitations: <https://docs.rs/flate2>
    fn compress(
        uncompressed_bytes: Bytes<'b>
    ) -> Result<Bytes<'b>, crate::layers::compressors::CompressError> {
        let compressed_bytes = zstd::bulk::compress(
            uncompressed_bytes.as_slice(),
            compression_level::<V>()
        )?;
        Ok(compressed_bytes.into())
    }

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
    /// decompression behavior and potential limitations: <https://docs.rs/flate2>
    fn decompress(
        compressed_bytes: Bytes<'b>
    ) -> Result<Bytes<'b>, crate::layers::compressors::DecompressError> {
        Ok(zstd::bulk::decompress(compressed_bytes.as_ref(), MAX_CAPACITY)?.into())
    }
}