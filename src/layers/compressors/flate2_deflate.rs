//! Support for [Sebastian Thiel](https://github.com/Byron) and [Josh
//! Triplett](https://github.com/joshtriplett)'s [flate2](https://crates.io/crates/flate2)
//! crate's deflate implementation.

use crate::layers::compressors::{Compressible, CompressionLevel, Compressor, Error, Method};
use crate::layers::ValueBuf;
use flate2::Compression;
use std::io::{Read, Write};

// -------------------------------------------------------------------------------------------------

pub struct DeflateCompressor;

// -------------------------------------------------------------------------------------------------
//
// Private Functions

/// Translates the developer's `CompressionLevel` setting into a compression level that the `flate2`
/// crate understands.
#[inline]
fn compression_level<V: Compressible>() -> Compression {
    match V::compression_level() {
        // The miniz_oxide backend for flate2 does not support level 0 or `Compression::none()`.
        // Instead it interprets them as the default compression level, which is quite slow.
        // `Compression::fast()` should be used instead.
        CompressionLevel::None => Compression::new(1),
        CompressionLevel::Fast => Compression::new(1),
        CompressionLevel::Balanced => Compression::new(5),
        CompressionLevel::Maximum => Compression::new(9),
        CompressionLevel::NoDecompress => Compression::new(9),
    }
}

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl<'b, V: Compressible> crate::layers::compressors::Compressor<'b, V> for DeflateCompressor {
    /// Compresses a value that has been previously serialized into bytes.
    ///
    /// # Errors
    ///
    /// * To understand the possible errors this compressor may produce, please refer to the
    ///   official documentation: <https://docs.rs/flate2>
    #[inline]
    fn compress(
        uncompressed_bytes: ValueBuf<'b>
    ) -> Result<ValueBuf<'b>, crate::layers::compressors::Error> {
        let mut encoder = flate2::write::DeflateEncoder::new(
            Vec::with_capacity(uncompressed_bytes.len()),
            compression_level::<V>()
        );
        encoder.write_all(&uncompressed_bytes)?;
        Ok(encoder.finish()?.into())
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
    ///   official documentation: <https://docs.rs/flate2>
    #[inline]
    fn decompress(
        compressed_bytes: ValueBuf<'b>
    ) -> Result<ValueBuf<'b>, crate::layers::compressors::Error> {
        let mut decoder = flate2::read::DeflateDecoder::new(compressed_bytes.as_ref());
        let mut decompressed_bytes = Vec::with_capacity(compressed_bytes.len() * 4);
        decoder.read_to_end(&mut decompressed_bytes)?;
        Ok(decompressed_bytes.into())
    }

    /// Returns the compression method that the current `Compressor` trait implements.
    ///
    /// This enables runtime identification of the compression algorithm in use, allowing
    /// applications to log compression details, or store metadata about how data was processed in
    /// the data pipeline.
    #[inline]
    fn method() -> &'static Method {
        &Method::Deflate
    }
}