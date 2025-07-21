//! Contains the error type returned from the compression implementation while decompressing data.

// -------------------------------------------------------------------------------------------------
//
/// An error returned from the compression implementation while decompressing data.
///
/// This includes errors for corrupted or malformed data, etc.
#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum Error {
    /// Error returned from the [brotli](https://crates.io/crates/brotli) crate.
    ///
    /// To understand the possible errors this decompressor may produce, please refer to the
    /// official documentation: <https://docs.rs/brotli>
    #[cfg(feature = "compress-brotli")]
    #[error("brotli decompression failed")]
    Brotli { #[from] #[source] source: std::io::Error },

    /// Error returned from the [bzip2](https://crates.io/crates/bzip2) crate.
    ///
    /// To understand the possible errors this decompressor may produce, please refer to the
    /// official documentation: <https://docs.rs/bzip2>
    #[cfg(feature = "compress-bzip2")]
    #[error("bzip2 decompression failed")]
    Bzip2 { #[from] #[source] source: std::io::Error },

    /// Error returned from the [flate2](https://crates.io/crates/flate2) crate.
    ///
    /// To understand the possible errors this decompressor may produce, please refer to the
    /// official documentation: <https://docs.rs/flate2>
    #[cfg(feature = "compress-deflate")]
    #[error("deflate decompression failed")]
    Deflate { #[from] #[source] source: std::io::Error },

    /// Error returned from the [flate2](https://crates.io/crates/flate2) crate.
    ///
    /// To understand the possible errors this decompressor may produce, please refer to the
    /// official documentation: <https://docs.rs/flate2>
    #[cfg(feature = "compress-gzip")]
    #[error("gzip decompression failed")]
    Gzip { #[from] #[source] source: std::io::Error },

    /// Error returned from the [lz4_flex](https://crates.io/crates/lz4_flex) crate.
    ///
    /// To understand the possible errors this decompressor may produce, please refer to the
    /// official documentation: <https://docs.rs/lz4_flex>
    #[cfg(feature = "compress-lz4")]
    #[error("lz4 decompression failed")]
    Lz4 { #[from] #[source] source: lz4_flex::block::DecompressError },

    /// Error returned from the [flate2](https://crates.io/crates/flate2) crate.
    ///
    /// To understand the possible errors this decompressor may produce, please refer to the
    /// official documentation: <https://docs.rs/flate2>
    #[cfg(feature = "compress-zlib")]
    #[error("zlib decompression failed")]
    Zlib { #[from] #[source] source: flate2::DecompressError },

    /// Error returned from the [zstd](https://crates.io/crates/zstd) crate.
    ///
    /// To understand the possible errors this decompressor may produce, please refer to the
    /// official documentation: <https://docs.rs/zstd>
    #[cfg(feature = "compress-zstd")]
    #[error("zstd decompression failed")]
    Zstd { #[from] #[source] source: std::io::Error },
}