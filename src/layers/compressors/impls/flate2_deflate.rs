//! DEFLATE compression using [Sebastian Thiel](https://github.com/Byron) and [Josh
//! Triplett](https://github.com/joshtriplett)'s [flate2](https://crates.io/crates/flate2)
//! crate's deflate implementation.

#![allow(clippy::doc_markdown)]

use crate::layers::compressors::impls::RESERVATION_FACTOR;
use crate::layers::compressors::{Compressible, Compressor, Level, Method};
use crate::layers::core::Bytes;
use flate2::Compression;
use std::io::{Read, Write};

// -------------------------------------------------------------------------------------------------
//
/// In computing, Deflate (stylized as DEFLATE, and also called Flate) is a lossless data
/// compression file format that uses a combination of LZ77 and Huffman coding.
///
/// It was designed by Phil Katz, for version 2 of his PKZIP archiving tool. Deflate was later
/// specified in Request for Comments (RFC) 1951 (1996).
///
/// Katz also designed the original algorithm used to construct Deflate streams. This algorithm
/// received software patent U.S. patent 5,051,745, assigned to PKWare, Inc. As stated in the RFC
/// document, an algorithm producing Deflate files was widely thought to be implementable in a
/// manner not covered by patents. This led to its widespread use. For example, in gzip compressed
/// files and Portable Network Graphics (PNG) image files, in addition to the ZIP file format for
/// which Katz originally designed it. The patent has since expired.
pub struct Deflate<V> {
    /// A marker to tie this `Deflate` compressor to a specific type `V` without storing any actual
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

impl<'b, V: Compressible> Compressor<'b, V> for Deflate<V> {
    /// Returns the compression method that the current `Compressor` trait implements.
    ///
    /// This enables runtime identification of the compression algorithm in use, allowing
    /// applications to log compression details, or store metadata about how data was processed in
    /// the data pipeline.
    const METHOD: &'static Method = &Method::Deflate;

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
        let mut encoder = flate2::write::DeflateEncoder::new(
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
        let mut decoder = flate2::read::DeflateDecoder::new(compressed_bytes.as_ref());
        let mut decompressed_bytes = Vec::with_capacity(
            compressed_bytes.len().saturating_mul(RESERVATION_FACTOR)
        );
        decoder.read_to_end(&mut decompressed_bytes)?;
        Ok(decompressed_bytes.into())
    }
}