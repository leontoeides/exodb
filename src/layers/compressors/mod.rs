//! Compression algorithms for reducing byte size of stored data.

// -------------------------------------------------------------------------------------------------
//
// Common Assets

mod core;
pub use crate::layers::compressors::core::CompressError;
pub use crate::layers::compressors::core::Compressible;
pub use crate::layers::compressors::core::Compressor;
pub use crate::layers::compressors::core::DecompressError;
pub use crate::layers::compressors::core::Error;
pub use crate::layers::compressors::core::Level;
pub use crate::layers::compressors::core::Method;

#[cfg(feature = "compress-dictionaries")]
pub use crate::layers::compressors::core::DictionaryBytes;

// -------------------------------------------------------------------------------------------------
//
// Compressor Implementations

mod impls;
pub use crate::layers::compressors::impls::ActiveCompressor;