//! Support for [Pascal Seitz](https://github.com/PSeitz)'s
//! [lz4_flex](https://crates.io/crates/lz4_flex) crate.

use crate::layers::compressors::{Compressible, Compressor, Method};
use crate::layers::descriptors::Direction;
use crate::layers::ValueBuf;

// -------------------------------------------------------------------------------------------------

pub struct Lz4FlexCompressor;

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl<'b, V: Compressible> Compressor<'b, V> for Lz4FlexCompressor {
    /// Compresses a value that has been previously serialized into bytes.
    ///
    /// # Errors
    ///
    /// * To understand the possible errors this compressor may produce, please refer to the
    ///   official documentation: <https://docs.rs/lz4_flex>
    #[inline]
    fn compress(
        uncompressed_bytes: ValueBuf<'b>
    ) -> Result<ValueBuf<'b>, crate::layers::compressors::CompressError> {
        Ok(lz4_flex::block::compress_prepend_size(uncompressed_bytes.as_slice()).into())
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
    ///   official documentation: <https://docs.rs/lz4_flex>
    #[inline]
    fn decompress(
        compressed_bytes: ValueBuf<'b>
    ) -> Result<ValueBuf<'b>, crate::layers::compressors::DecompressError> {
        Ok(lz4_flex::block::decompress_size_prepended(compressed_bytes.as_ref())?.into())
    }

    /// Returns the compression method that the current `Compressor` trait implements.
    ///
    /// This enables runtime identification of the compression algorithm in use, allowing
    /// applications to log compression details, or store metadata about how data was processed in
    /// the data pipeline.
    #[inline]
    fn method() -> &'static Method {
        &Method::Lz4
    }
}