//! Compression dictionaries are used to store frequently occurring patterns or sequences in data,
//! allowing for more performant encoding and greater size reductions.

#[cfg(not(feature = "compress-zstd"))]
mod standard;

#[cfg(not(feature = "compress-zstd"))]
pub use crate::layers::compressors::core::dictionary_bytes::standard::DictionaryBytesStandard as DictionaryBytes;

#[cfg(feature = "compress-zstd")]
mod zstd;

#[cfg(feature = "compress-zstd")]
pub use crate::layers::compressors::core::dictionary_bytes::zstd::DictionaryBytesZstd as DictionaryBytes;

// -------------------------------------------------------------------------------------------------
//
// Dictionaries Feature Guard

#[cfg(all(
    feature = "compress-dictionaries",
    any(
        feature = "compress-brotli",
        feature = "compress-bzip2",
        feature = "compress-deflate",
        feature = "compress-gzip"
    ),
    not(any(
        feature = "compress-lz4",
        feature = "compress-zlib",
        feature = "compress-zstd"
    ))
))]
compile_error!(
    "The `compress-dictionaries` feature requires a compatible compressor. \
    For dictionary support, enable one of the following compressors: \
    `compress-lz4`, `compress-zlib`, or `compress-zstd`"
);
