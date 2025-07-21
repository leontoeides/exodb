//! Contains the error type returned from the compression implementation while compressing data.

// -------------------------------------------------------------------------------------------------
//
/// An error returned from the compression implementation while compressing data.
///
/// This includes errors for out of memory, etc.
#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum Error {
    /// Error returned from the [brotli](https://crates.io/crates/brotli) crate.
    ///
    /// To understand the possible errors this compressor may produce, please refer to the official
    /// documentation: <https://docs.rs/brotli>
    #[cfg(feature = "compress-brotli")]
    #[error("brotli compression failed")]
    Brotli { #[from] #[source] source: std::io::Error },

    /// Error returned from the [bzip2](https://crates.io/crates/bzip2) crate.
    ///
    /// To understand the possible errors this compressor may produce, please refer to the official
    /// documentation: <https://docs.rs/bzip2>
    #[cfg(feature = "compress-bzip2")]
    #[error("bzip2 compression failed")]
    Bzip2 { #[from] #[source] source: std::io::Error },

    /// Error returned from the [flate2](https://crates.io/crates/flate2) crate.
    ///
    /// To understand the possible errors this compressor may produce, please refer to the official
    /// documentation: <https://docs.rs/flate2>
    #[cfg(feature = "compress-deflate")]
    #[error("deflate compression failed")]
    Deflate { #[from] #[source] source: std::io::Error },

    /// Error returned from the [flate2](https://crates.io/crates/flate2) crate.
    ///
    /// To understand the possible errors this compressor may produce, please refer to the official
    /// documentation: <https://docs.rs/flate2>
    #[cfg(feature = "compress-gzip")]
    #[error("gzip compression failed")]
    Gzip { #[from] #[source] source: std::io::Error },

    /// Error returned from the [flate2](https://crates.io/crates/flate2) crate.
    ///
    /// To understand the possible errors this compressor may produce, please refer to the official
    /// documentation: <https://docs.rs/flate2>
    #[cfg(feature = "compress-zlib")]
    #[error("zlib compression failed")]
    Zlib { #[from] #[source] source: flate2::CompressError },

    /// Error returned from the [zstd](https://crates.io/crates/zstd) crate.
    ///
    /// To understand the possible errors this compressor may produce, please refer to the official
    /// documentation: <https://docs.rs/zstd>
    #[cfg(feature = "compress-zstd")]
    #[error("zstd compression failed")]
    Zstd { #[from] #[source] source: std::io::Error },
}