//! Bzip2 compression using [Alex Crichton](https://github.com/alexcrichton),
//! [bjorn3](https://github.com/bjorn3), and [Folkert de Vries](https://github.com/folkertdev)'s
//! [bzip2](https://crates.io/crates/bzip2) crate.

use crate::layers::compressors::impls::RESERVATION_FACTOR;
use crate::layers::compressors::{Compressible, Compressor, Level, Method};
use crate::layers::core::Bytes;
use std::io::{Read, Write};

// -------------------------------------------------------------------------------------------------
//
/// bzip2 is a free and open-source file compression algorithm that uses the Burrows–Wheeler
/// algorithm.
///
/// It compresses single files and is not a file archiver, relying on external utilities such as tar
/// for handling multiple files. The algorithm uses several layers of compression techniques, such
/// as run-length encoding (RLE), Burrows–Wheeler transform (BWT), move-to-front transform (MTF),
/// and Huffman coding. bzip2 compresses data in blocks between 100 and 900 kB and uses the
/// Burrows–Wheeler transform to convert frequently recurring character sequences into strings of
/// identical letters. The move-to-front transform and Huffman coding are then applied. The
/// compression performance is asymmetric, with decompression being faster than compression.
///
/// bzip2 is particularly efficient for text data, and decompression is relatively fast. It
/// typically compresses files to anywhere between 10% - 15% of their original size while
/// maintaining a speed of roughly twice the speed of compression and up to six times the speed of
/// decompression of the PPM compression family.
pub struct Bzip2<V> {
    /// A marker to tie this `Bzip2` compressor to a specific type `V` without storing any actual
    /// data.
    phantom_data: std::marker::PhantomData<V>
}

// -------------------------------------------------------------------------------------------------
//
// Private Functions

/// Translates the developer's compression `Level` setting into a compression level that the `bzip2`
/// crate understands.
#[inline]
const fn compression_level<V: Compressible>() -> u32 {
    match V::LEVEL {
        Level::Minimum => 1,
        Level::Medium  => 6, // `bzip2` crate's default compression level
        Level::Maximum => 9,
    }
}

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl<'b, V: Compressible> Compressor<'b, V> for Bzip2<V> {
    /// Returns the compression method that the current `Compressor` trait implements.
    ///
    /// This enables runtime identification of the compression algorithm in use, allowing
    /// applications to log compression details, or store metadata about how data was processed in
    /// the data pipeline.
    const METHOD: &'static Method = &Method::Bzip2;

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
    /// compression behavior and potential limitations: <https://docs.rs/bzip2>
    fn compress(
        uncompressed_bytes: Bytes<'b>
    ) -> Result<Bytes<'b>, crate::layers::compressors::CompressError> {
        let mut encoder = bzip2::write::BzEncoder::new(
            Vec::with_capacity(uncompressed_bytes.len()),
            bzip2::Compression::new(compression_level::<V>())
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
    /// decompression behavior and potential limitations: <https://docs.rs/bzip2>
    fn decompress(
        compressed_bytes: Bytes<'b>,
    ) -> Result<Bytes<'b>, crate::layers::compressors::DecompressError> {
        let mut decoder = bzip2::read::BzDecoder::new(compressed_bytes.as_ref());
        let mut decompressed_bytes = Vec::with_capacity(
            compressed_bytes.len().saturating_mul(RESERVATION_FACTOR)
        );
        decoder.read_to_end(&mut decompressed_bytes)?;
        Ok(decompressed_bytes.into())
    }
}