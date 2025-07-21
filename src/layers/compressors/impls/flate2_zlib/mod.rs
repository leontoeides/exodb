//! Support for [Sebastian Thiel](https://github.com/Byron) and [Josh
//! Triplett](https://github.com/joshtriplett)'s [flate2](https://crates.io/crates/flate2)
//! crate's zlib implementation.

#[cfg(feature = "compress-dictionaries")]
mod dictionary;

#[cfg(not(feature = "compress-dictionaries"))]
mod standard;

// -------------------------------------------------------------------------------------------------
//
/// Zlib compression algorithm is a method of lossless data compression that uses the DEFLATE
/// algorithm, which combines the LZ77 and Huffman coding techniques.
///
/// The DEFLATE algorithm is used for both compression and decompression, with the compression
/// process encoding the input data into compressed data and the decompression process decoding the
/// deflate bit stream to produce the original data.
///
/// Zlib provides a range of compression levels, from 0 (no compression) to 9 (maximum compression),
/// allowing users to balance between speed and compression ratio. The algorithm is designed to be
/// efficient and portable, with a memory footprint that is independent of the input data.
///
/// Additionally, zlib is known for its ability to compress data without expanding it, which is a
/// significant advantage over other compression methods like LZW, which can sometimes increase the
/// file size.
pub struct Zlib<V> {
    /// A marker to tie this `Zlib` compressor to a specific type `V` without storing any actual
    /// data.
    phantom_data: std::marker::PhantomData<V>
}

// -------------------------------------------------------------------------------------------------
//
// Private Functions

use crate::layers::compressors::Level;
use flate2::Compression;

/// Translates the developer's compression `Level` setting into a compression level that the
/// `flate2` crate understands.
#[inline]
const fn compression_level<V: crate::layers::compressors::Compressible>() -> Compression {
    match V::LEVEL {
        Level::Minimum => Compression::new(1),
        Level::Medium  => Compression::new(5),
        Level::Maximum => Compression::new(9),
    }
}