//! The `Compressor` traits provide a set of common interfaces for compressing and decompressing
//! data.

// Standard Compressor

#[cfg(not(feature = "compress-dictionaries"))]
mod standard;

#[cfg(not(feature = "compress-dictionaries"))]
pub use crate::layers::compressors::core::traits::standard::StandardCompressor as Compressor;

// Dictionary Compressor

#[cfg(feature = "compress-dictionaries")]
mod dictionary;

#[cfg(feature = "compress-dictionaries")]
pub use crate::layers::compressors::core::traits::dictionary::DictionaryCompressor as Compressor;