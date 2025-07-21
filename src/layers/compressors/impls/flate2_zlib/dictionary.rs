//! Compression implementation with support for dictionaries.

use crate::layers::compressors::impls::flate2_zlib::{compression_level, Zlib};
use crate::layers::compressors::impls::{BUFFER_LEN, RESERVATION_FACTOR};
use crate::layers::compressors::{Compressible, Compressor, DictionaryBytes, Method};
use crate::layers::core::Bytes;

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl<'b, 'd, V: Compressible> Compressor<'b, 'd, V> for Zlib<V> {
    /// Returns the compression method that the current `Compressor` trait implements.
    ///
    /// This enables runtime identification of the compression algorithm in use, allowing
    /// applications to log compression details, or store metadata about how data was processed in
    /// the data pipeline.
    const METHOD: &'static Method = &Method::Zlib;

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
    #[allow(
        clippy::cast_possible_truncation,
        reason = "u32 is sufficient for a single value on 32-bit platforms"
    )]
    fn compress(
        uncompressed_bytes: Bytes<'b>,
        dictionary: Option<DictionaryBytes<'d>>
    ) -> Result<Bytes<'b>, crate::layers::compressors::CompressError> {
        let mut encoder = flate2::Compress::new(compression_level::<V>(), true);
        if let Some(dictionary) = dictionary {
            encoder.set_dictionary(dictionary.as_ref())?;
        }
        let mut compressed_bytes = Vec::with_capacity(uncompressed_bytes.len());
        let mut input_pos = 0;

        loop {
            let old_len = compressed_bytes.len();
            compressed_bytes.resize(old_len + BUFFER_LEN, 0);

            let before_in = encoder.total_in() as usize;
            let before_out = encoder.total_out() as usize;

            let status = encoder.compress(
                &uncompressed_bytes[input_pos..],
                &mut compressed_bytes[old_len..],
                flate2::FlushCompress::Finish,
            )?;

            let consumed = encoder.total_in() as usize - before_in;
            let produced = encoder.total_out() as usize - before_out;

            input_pos += consumed;
            compressed_bytes.truncate(old_len + produced);

            if status == flate2::Status::StreamEnd { break; }
        }

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
    #[allow(
        clippy::cast_possible_truncation,
        reason = "u32 is sufficient for a single value on 32-bit platforms"
    )]
    fn decompress(
        compressed_bytes: Bytes<'b>,
        dictionary: Option<DictionaryBytes<'d>>
    ) -> Result<Bytes<'b>, crate::layers::compressors::DecompressError> {
        let mut decoder = flate2::Decompress::new(true);
        if let Some(dictionary) = dictionary {
            decoder.set_dictionary(dictionary.as_ref())?;
        }
        let mut decompressed_bytes = Vec::with_capacity(compressed_bytes.len() * RESERVATION_FACTOR);
        let mut input_pos = 0;

        loop {
            let old_len = decompressed_bytes.len();
            decompressed_bytes.resize(old_len + BUFFER_LEN, 0);

            let before_in = decoder.total_in() as usize;
            let before_out = decoder.total_out() as usize;

            let status = decoder.decompress(
                &compressed_bytes[input_pos..],
                &mut decompressed_bytes[old_len..],
                flate2::FlushDecompress::Finish,
            )?;

            let consumed = decoder.total_in() as usize - before_in;
            let produced = decoder.total_out() as usize - before_out;

            input_pos += consumed;
            decompressed_bytes.truncate(old_len + produced);

            if status == flate2::Status::StreamEnd { break; }
        }

        Ok(decompressed_bytes.into())
    }
}