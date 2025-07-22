//! Zstandard compression using [Alexandre Bury](https://github.com/gyscos)'s
//! [zstd](https://crates.io/crates/zstd) crate.

#[cfg(feature = "compress-dictionaries")]
mod dictionary;

#[cfg(not(feature = "compress-dictionaries"))]
mod standard;

// -------------------------------------------------------------------------------------------------
//
/// Zstandard, commonly referred to as ZSTD, is a fast, lossless data compression algorithm that
/// offers a compelling balance between compression ratio and speed. It was developed by Yann Collet
/// at Facebook, and it has gained widespread adoption across the tech industry due to its high
/// compression ratios, fast decompression speeds, and scalability. ZSTD is designed to provide a
/// better balance of compression speed and compression ratio compared to other popular algorithms
/// like gzip and bzip2.
///
/// One of the key benefits of ZSTD is its ability to offer high compression ratios while
/// maintaining fast decompression speeds, making it suitable for real-time applications where quick
/// data processing is crucial. Additionally, ZSTD supports a wide range of compression levels,
/// from -22 to 22, allowing users to choose the optimal trade-off between compression speed and
/// ratio. This flexibility makes it versatile for various applications, from real-time data
/// compression in network communications to efficient storage of large datasets.
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