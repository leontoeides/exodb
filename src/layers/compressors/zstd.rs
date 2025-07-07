//! Support for [Alexandre Bury](https://github.com/gyscos)'s [zstd](https://crates.io/crates/zstd)
//! crate.

use crate::layers::compressors::{Compressible, CompressionLevel, Compressor, Error, Method};
use crate::layers::ValueBuf;
use zstd::compression_level_range;

// -------------------------------------------------------------------------------------------------

pub struct ZstdCompressor;

// -------------------------------------------------------------------------------------------------
//
// Constants

/// The decompressed data should be at most capacity bytes, or an error will be returned.
const MAX_CAPACITY: usize = 1_073_741_824;

// -------------------------------------------------------------------------------------------------
//
// Private Functions

/// Translates the `CompressionLevel` setting into a compression level that the `zstd` crate
/// understands.
#[inline]
fn compression_level<V: Compressible>() -> i32 {
    match V::compression_level() {
        CompressionLevel::Fast => *compression_level_range().start(),
        CompressionLevel::Balanced => 3, // `zstd` crate's current default compression level
        CompressionLevel::Maximum => *compression_level_range().end(),
    }
}

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl<'b, V: Compressible> crate::layers::compressors::Compressor<'b, V> for ZstdCompressor {
    /// Compresses a value that has been previously serialized into bytes.
    ///
    /// # Errors
    ///
    /// * To understand the possible errors this compressor may produce, please refer to the
    ///   official documentation: <https://docs.rs/zstd>
    #[inline]
    fn compress(
        uncompressed_bytes: ValueBuf<'b>
    ) -> Result<ValueBuf<'b>, crate::layers::compressors::Error> {
        let compressed_bytes = zstd::bulk::compress(
            uncompressed_bytes.as_slice(),
            compression_level::<V>()
        )?;
        Ok(compressed_bytes.into())
    }

    /// Decompresses a slice of bytes into a value (in serialized-bytes form.)
    ///
    /// # Lifetimes
    ///
    /// * The `b` lifetime represents bytes potentially being borrowed from the `redb` database.
    ///
    /// # Errors
    ///
    /// * To understand the possible errors this decompressor may produce, please refer to the
    ///   official documentation: <https://docs.rs/zstd>
    #[inline]
    fn decompress(
        compressed_bytes: ValueBuf<'b>
    ) -> Result<ValueBuf<'b>, crate::layers::compressors::Error> {
        Ok(zstd::bulk::decompress(compressed_bytes.as_ref(), MAX_CAPACITY)?.into())
    }

    /// Returns the compression method that the current `Compressor` trait implements.
    ///
    /// This enables runtime identification of the compression algorithm in use, allowing
    /// applications to log compression details, or store metadata about how data was processed in
    /// the data pipeline.
    #[inline]
    fn method() -> &'static Method {
        &Method::Zstd
    }
}