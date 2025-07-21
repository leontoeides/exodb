//! Support for [Servo](https://github.com/servo), [Simon Sapin](https://github.com/SimonSapin), and
//! [Daniel Horn](https://github.com/danielrh)'s [brotli](https://crates.io/crates/brotli) crate.

use crate::layers::compressors::impls::{BUFFER_LEN, RESERVATION_FACTOR};
use crate::layers::compressors::{Compressible, Compressor, Level, Method};
use crate::layers::core::Bytes;
use std::io::{Cursor, Write};

// -------------------------------------------------------------------------------------------------
//
/// Brotli is a lossless data compression algorithm developed by Google. It uses a combination of
/// the LZ77 algorithm, Huffman coding, and 2nd-order context modeling to compress data efficiently.
///
/// Brotli is designed specifically for the web and is used by web servers and content delivery
/// networks to compress HTTP content, making websites load faster.
///
/// Compared to Gzip, Brotli offers a higher compression ratio, which means it can reduce file sizes
/// more effectively, resulting in faster loading times for web pages.
///
/// Brotli is open-source and available under the MIT License.
pub struct Brotli<V> {
    /// A marker to tie this `Brotli` compressor to a specific type `V` without storing any actual
    /// data.
    phantom_data: std::marker::PhantomData<V>
}

// -------------------------------------------------------------------------------------------------
//
// Constants

/// Log of how big the ring buffer should be for copying prior data. The `brotli` crate default is
/// `22`.
const LGWIN: u32 = 22; // Brotli default

// -------------------------------------------------------------------------------------------------
//
// Private Functions

/// Translates the developer's compression `Level` setting into a compression level that the
/// `brotli` crate understands.
#[inline]
const fn compression_level<V: Compressible>() -> u32 {
    match V::LEVEL {
        Level::Minimum => 1,
        Level::Medium  => 6,
        Level::Maximum => 11,
    }
}

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl<'b, V: Compressible> Compressor<'b, V> for Brotli<V> {
    /// Returns the compression method that the current `Compressor` trait implements.
    ///
    /// This enables runtime identification of the compression algorithm in use, allowing
    /// applications to log compression details, or store metadata about how data was processed in
    /// the data pipeline.
    const METHOD: &'static Method = &Method::Brotli;

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
    /// compression behavior and potential limitations: <https://docs.rs/brotli>
    fn compress(
        uncompressed_bytes: Bytes<'b>
    ) -> Result<Bytes<'b>, crate::layers::compressors::CompressError> {
        let mut compressed_bytes = Vec::with_capacity(uncompressed_bytes.len());
        {
            let mut writer = brotli::CompressorWriter::new(
                &mut compressed_bytes,
                BUFFER_LEN,
                compression_level::<V>(),
                LGWIN,
            );
            writer.write_all(&uncompressed_bytes)?;
            writer.flush()?; // Important!
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
    /// # Errors
    ///
    /// This method may fail for several reasons, including:
    ///
    /// * Input bytes are corrupted or malformed
    ///
    /// Consult the documentation of the compressor backend you are using for more detail on
    /// decompression behavior and potential limitations: <https://docs.rs/brotli>
    fn decompress(
        compressed_bytes: Bytes<'b>
    ) -> Result<Bytes<'b>, crate::layers::compressors::DecompressError> {
        let (metadata, compressed_data) = compressed_bytes.into_parts();
        let mut output = Vec::with_capacity(
            compressed_data.len().saturating_mul(RESERVATION_FACTOR)
        );
        let mut decompressor = brotli::Decompressor::new(Cursor::new(compressed_data), BUFFER_LEN);
        std::io::copy(&mut decompressor, &mut output)?;
        Ok(Bytes::from_parts(metadata, output.into()))
    }
}