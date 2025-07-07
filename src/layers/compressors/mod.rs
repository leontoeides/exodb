//! Compression algorithms for reducing byte size of stored or transmitted data. Applied after
//! serialization and before encryption.

mod compressible;
pub use crate::layers::compressors::compressible::Compressible;

mod level;
pub use crate::layers::compressors::level::Level;

mod compressor;
pub use crate::layers::compressors::compressor::Compressor;

mod compress_error;
pub use crate::layers::compressors::compress_error::CompressError;

mod decompress_error;
pub use crate::layers::compressors::decompress_error::DecompressError;

mod error;
pub use crate::layers::compressors::error::Error;

mod method;
pub use crate::layers::compressors::method::Method;

// -------------------------------------------------------------------------------------------------
//
// Compressor Feature Guard

/// Helper macro: counts how many of the listed features are turned on.
macro_rules! count_features {
    ($($feat:literal),* $(,)?) => {
        0_usize $(+ cfg!(feature = $feat) as usize)*
    };
}

const _COMPRESSOR_FEATURE_COUNT: usize = count_features!(
    "compress-brotli",
    "compress-bzip2",
    "compress-deflate",
    "compress-gzip",
    "compress-lz4",
    "compress-zlib",
    "compress-zstd",
);

const _: () = {
    assert!(
        // Only one compressor feature can be enabled. To fix: 1. open `Cargo.toml` file, 2. find
        // `[dependencies]` section and where `atlatl` is, 3. ensure only one compressor is enabled.
        !(_COMPRESSOR_FEATURE_COUNT > 1),
        "Multiple compressor features enabled! Please enable only one of: \
        `compress-brotli`, \
        `compress-bzip2`, \
        `compress-deflate`, \
        `compress-gzip`, \
        `compress-lz4`, \
        `compress-zlib`, or \
        `compress-zstd`",
    );
};

// -------------------------------------------------------------------------------------------------
//
// Compressor Implementations

#[cfg(any(feature = "compress-brotli"))]
mod brotli;

#[cfg(any(feature = "compress-bzip2"))]
pub use crate::layers::compressors::brotli::BrotliCompressor;

#[cfg(any(feature = "compress-bzip2"))]
mod bzip2;

#[cfg(any(feature = "compress-bzip2"))]
pub use crate::layers::compressors::bzip2::Bzip2Compressor;

#[cfg(any(feature = "compress-deflate"))]
mod flate2_deflate;

#[cfg(any(feature = "compress-deflate"))]
pub use crate::layers::compressors::flate2_deflate::DeflateCompressor;

#[cfg(any(feature = "compress-gzip"))]
mod flate2_gzip;

#[cfg(any(feature = "compress-gzip"))]
pub use crate::layers::compressors::flate2_gzip::GzipCompressor;

#[cfg(feature = "compress-lz4")]
mod lz4_flex;

#[cfg(feature = "compress-lz4")]
pub use crate::layers::compressors::lz4_flex::Lz4FlexCompressor;

#[cfg(any(feature = "compress-zlib"))]
mod flate2_zlib;

#[cfg(any(feature = "compress-zlib"))]
pub use crate::layers::compressors::flate2_zlib::ZlibCompressor;

#[cfg(feature = "compress-zstd")]
mod zstd;

#[cfg(feature = "compress-zstd")]
pub use crate::layers::compressors::zstd::ZstdCompressor;