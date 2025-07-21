//! `Compressor` data compression implementations. A single implementation is selected at
//! compile-time in the host application's `Cargo.toml` file.

// -------------------------------------------------------------------------------------------------
//
// Constants

/// The amount of data that is temporarily stored and processed during the compression process. It
/// plays a crucial role in determining the efficiency and performance of the compression algorithm.
#[cfg(any(feature = "compress-brotli", feature = "compress-zlib"))]
const BUFFER_LEN: usize = 4_096; // `4_096` = typical compression buffer size.

/// Reservation factor. A simple heuristic to help ompressor implementations try to guess how much
/// memory to reserve for the decompressed data.
///
/// If the factor is too low, it may cause unnecessary memory allocations, slowing down
/// decompression. If the factor is too high, the implemenation may use more memory than necessary.
#[cfg(any(
    feature = "compress-brotli",
    feature = "compress-bzip2",
    feature = "compress-deflate",
    feature = "compress-gzip",
    feature = "compress-zlib",
))]
const RESERVATION_FACTOR: usize = 4; // `4` = reserve 4 times the compressed size.

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

#[cfg(feature = "compress-brotli")]
mod brotli;

#[cfg(feature = "compress-brotli")]
/// `Brotli` has been selected as the `ActiveCompressor` using `Cargo.toml` feature.
pub use crate::layers::compressors::impls::brotli::Brotli as ActiveCompressor;

#[cfg(feature = "compress-bzip2")]
mod bzip2;

#[cfg(feature = "compress-bzip2")]
/// `BZip2` has been selected as the `ActiveCompressor` using `Cargo.toml` feature.
pub use crate::layers::compressors::impls::bzip2::Bzip2 as ActiveCompressor;

#[cfg(feature = "compress-deflate")]
mod flate2_deflate;

#[cfg(feature = "compress-deflate")]
/// `Deflate` has been selected as the `ActiveCompressor` using `Cargo.toml` feature.
pub use crate::layers::compressors::impls::flate2_deflate::Deflate as ActiveCompressor;

#[cfg(feature = "compress-gzip")]
mod flate2_gzip;

#[cfg(feature = "compress-gzip")]
/// `Gzip` has been selected as the `ActiveCompressor` using `Cargo.toml` feature.
pub use crate::layers::compressors::impls::flate2_gzip::Gzip as ActiveCompressor;

#[cfg(feature = "compress-lz4")]
mod lz4_flex;

#[cfg(feature = "compress-lz4")]
/// `Lz4Flex` has been selected as the `ActiveCompressor` using `Cargo.toml` feature.
pub use crate::layers::compressors::impls::lz4_flex::Lz4Flex as ActiveCompressor;

#[cfg(feature = "compress-zlib")]
mod flate2_zlib;

#[cfg(feature = "compress-zlib")]
/// `Zlib` has been selected as the `ActiveCompressor` using `Cargo.toml` feature.
pub use crate::layers::compressors::impls::flate2_zlib::Zlib as ActiveCompressor;

#[cfg(feature = "compress-zstd")]
pub(super) mod zstd;

#[cfg(feature = "compress-zstd")]
/// `Zstd` has been selected as the `ActiveCompressor` using `Cargo.toml` feature.
pub use crate::layers::compressors::impls::zstd::Zstd as ActiveCompressor;