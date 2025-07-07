//! Support for [Servo](https://github.com/servo), [Simon Sapin](https://github.com/SimonSapin), and
//! [Daniel Horn](https://github.com/danielrh)'s [brotli](https://crates.io/crates/brotli) crate.

use crate::layers::compressors::{Compressible, CompressionLevel, Compressor, Error, Method};
use crate::layers::ValueBuf;
use std::io::Cursor;

// -------------------------------------------------------------------------------------------------

pub struct BrotliCompressor;

// -------------------------------------------------------------------------------------------------
//
// Private Functions

/// Translates the developer's `CompressionLevel` setting into a compression level that the `brotli`
/// crate understands.
#[inline]
fn compression_level<V: Compressible>() -> i32 {
    match V::compression_level() {
        CompressionLevel::None => 0,
        CompressionLevel::Fast => 1,
        CompressionLevel::Balanced => 6,
        CompressionLevel::Maximum => 11,
        CompressionLevel::NoDecompress => 11,
    }
}

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl<'b, V: Compressible> crate::layers::compressors::Compressor<'b, V> for BrotliCompressor {
    /// Compresses a value that has been previously serialized into bytes.
    ///
    /// # Errors
    ///
    /// * To understand the possible errors this compressor may produce, please refer to the
    ///   official documentation: <https://docs.rs/bzip2>
    #[inline]
    fn compress(
        uncompressed_bytes: ValueBuf<'b>
    ) -> Result<ValueBuf<'b>, crate::layers::compressors::Error> {
        if V::compression_level() == &CompressionLevel::None {
            Ok(uncompressed_bytes)
        } else {
            let uncompressed_bytes_len: usize = uncompressed_bytes.len();
            let mut compressed_bytes = Vec::with_capacity(uncompressed_bytes_len);
            brotli::BrotliCompress(
                &mut Cursor::new(uncompressed_bytes),
                &mut Cursor::new(&mut compressed_bytes),
                &brotli::enc::BrotliEncoderParams {
                    quality: compression_level::<V>(),
                    size_hint: uncompressed_bytes_len,
                    ..Default::default()
                }
            )?;
            Ok(compressed_bytes.into())
        }
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
    ///   official documentation: <https://docs.rs/bzip2>
    #[inline]
    fn decompress(
        compressed_bytes: ValueBuf<'b>
    ) -> Result<ValueBuf<'b>, crate::layers::compressors::Error> {
        use CompressionLevel::{NoDecompress, None};
        if V::compression_level() == &None || V::compression_level() == &NoDecompress {
            Ok(compressed_bytes)
        } else {
            let mut decompressed_bytes = Vec::with_capacity(compressed_bytes.len() * 4);
            brotli::BrotliDecompress(
                &mut Cursor::new(compressed_bytes),
                &mut Cursor::new(&mut decompressed_bytes)
            )?;
            Ok(decompressed_bytes.into())
        }
    }

    /// Returns the compression method that the current `Compressor` trait implements.
    ///
    /// This enables runtime identification of the compression algorithm in use, allowing
    /// applications to log compression details, or store metadata about how data was processed in
    /// the data pipeline.
    #[inline]
    fn method() -> &'static Method {
        &Method::Brotli
    }
}