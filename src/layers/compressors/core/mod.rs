//! Common types and traits that are used across the various compression implementations.

mod compressible;
pub use crate::layers::compressors::core::compressible::Compressible;

mod traits;
pub use crate::layers::compressors::core::traits::Compressor;

#[cfg(feature = "compress-dictionaries")]
mod dictionary_bytes;

#[cfg(feature = "compress-dictionaries")]
pub use crate::layers::compressors::core::dictionary_bytes::DictionaryBytes;

mod errors;
pub use crate::layers::compressors::core::errors::CompressError;
pub use crate::layers::compressors::core::errors::DecompressError;
pub use crate::layers::compressors::core::errors::Error;

mod level;
pub use crate::layers::compressors::core::level::Level;

mod method;
pub use crate::layers::compressors::core::method::Method;