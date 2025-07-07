//! Support for [Alex Crichton](https://github.com/alexcrichton),
//! [bjorn3](https://github.com/bjorn3), and [Folkert de Vries](https://github.com/folkertdev)'s
//! [bzip2](https://crates.io/crates/bzip2) crate.

use crate::layers::compressors::{Compressible, CompressionLevel, Compressor, Error, Method};
use crate::layers::ValueBuf;
use std::io::{Read, Write};

// -------------------------------------------------------------------------------------------------

pub struct Bzip2Compressor;

// -------------------------------------------------------------------------------------------------
//
// Private Functions

/// Translates the developer's `CompressionLevel` setting into a compression level that the `bzip2`
/// crate understands.
#[inline]
fn compression_level<V: Compressible>() -> u32 {
    match V::compression_level() {
        CompressionLevel::Fast => 1,
        CompressionLevel::Balanced => 6, // `bzip2` crate's default compression level
        CompressionLevel::Maximum => 9,
    }
}

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl<'b, V: Compressible> crate::layers::compressors::Compressor<'b, V> for Bzip2Compressor {
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
            let mut encoder = bzip2::write::BzEncoder::new(
                Vec::with_capacity(uncompressed_bytes.len()),
                bzip2::Compression::new(compression_level::<V>())
            );
            encoder.write_all(&uncompressed_bytes)?;
            Ok(encoder.finish()?.into())
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
            let mut decoder = bzip2::read::BzDecoder::new(compressed_bytes.as_ref());
            let mut decompressed_bytes = Vec::with_capacity(compressed_bytes.len() * 4);
            decoder.read_to_end(&mut decompressed_bytes)?;
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
        &Method::Bzip2
    }
}