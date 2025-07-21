//! Support for [Sebastian Thiel](https://github.com/Byron) and [Josh
//! Triplett](https://github.com/joshtriplett)'s [flate2](https://crates.io/crates/flate2)
//! crate's gzip implementation.

use crate::layers::compressors::impls::RESERVATION_FACTOR;
use crate::layers::compressors::{Compressible, Compressor, Level, Method};
use crate::layers::core::Bytes;
use flate2::Compression;
use std::io::{Read, Write};

// -------------------------------------------------------------------------------------------------
//
/// GZIP compression is a widely used technique based on the DEFLATE algorithm, which combines the
/// LZ77 and Huffman coding algorithms to reduce file size without losing any data
///
/// It is often used for lossless compression of files, particularly for web content such as HTML,
/// CSS, and JavaScript. This method is popular because it provides a good balance between
/// compression ratio and speed, making it suitable for a variety of applications.
///
/// The GZIP algorithm works by identifying repeated sequences of bytes in the data and replacing
/// them with shorter representations. These shortened sequences are then assigned a variable number
/// of bits based on their frequency of occurrence. More frequent sequences receive fewer bits,
/// while rarer ones are assigned more bits. This process results in a smaller compressed file than
/// the original file.
pub struct Gzip<V> {
    /// A marker to tie this `Gzip` compressor to a specific type `V` without storing any actual
    /// data.
    phantom_data: std::marker::PhantomData<V>
}

// -------------------------------------------------------------------------------------------------
//
// Private Functions

/// Translates the developer's compression `Level` setting into a compression level that the
/// `flate2` crate understands.
#[inline]
const fn compression_level<V: Compressible>() -> Compression {
    match V::LEVEL {
        Level::Minimum => Compression::new(1),
        Level::Medium  => Compression::new(5),
        Level::Maximum => Compression::new(9),
    }
}

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl<'b, V: Compressible> Compressor<'b, V> for Gzip<V> {
    /// Returns the compression method that the current `Compressor` trait implements.
    ///
    /// This enables runtime identification of the compression algorithm in use, allowing
    /// applications to log compression details, or store metadata about how data was processed in
    /// the data pipeline.
    const METHOD: &'static Method = &Method::Gzip;

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
        let mut encoder = flate2::write::GzEncoder::new(
            Vec::with_capacity(uncompressed_bytes.len()),
            compression_level::<V>()
        );
        encoder.write_all(&uncompressed_bytes)?;
        Ok(encoder.finish()?.into())
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
        let mut decoder = flate2::read::GzDecoder::new(compressed_bytes.as_ref());
        let mut decompressed_bytes = Vec::with_capacity(
            compressed_bytes.len().saturating_mul(RESERVATION_FACTOR)
        );
        decoder.read_to_end(&mut decompressed_bytes)?;
        Ok(decompressed_bytes.into())
    }
}