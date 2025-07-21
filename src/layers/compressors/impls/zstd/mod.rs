//! Support for [Alexandre Bury](https://github.com/gyscos)'s [zstd](https://crates.io/crates/zstd)
//! crate.

#[cfg(feature = "compress-dictionaries")]
mod dictionary;

#[cfg(not(feature = "compress-dictionaries"))]
mod standard;

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
pub struct Zstd<V> {
    /// A marker to tie this `Zstd` compressor to a specific type `V` without storing any actual
    /// data.
    phantom_data: std::marker::PhantomData<V>
}

// -------------------------------------------------------------------------------------------------
//
// Constants

/// The decompressed data should be at most capacity bytes, or an error will be returned.
const MAX_CAPACITY: usize = u32::MAX as usize;

// -------------------------------------------------------------------------------------------------
//
// Private Functions

use crate::layers::compressors::{Compressible, Level};
use zstd::compression_level_range;

/// Translates the developer's compression `Level` setting into a compression level that the
/// `flate2` crate understands.
#[inline]
pub fn compression_level<V: Compressible>() -> i32 {
    match V::LEVEL {
        Level::Minimum => *compression_level_range().start(),
        Level::Medium  => 3,
        Level::Maximum => *compression_level_range().end(),
    }
}