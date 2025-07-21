//! Error types used across the various compression implementations.

mod compress;
pub use crate::layers::compressors::core::errors::compress::Error as CompressError;

mod decompress;
pub use crate::layers::compressors::core::errors::decompress::Error as DecompressError;

mod compression;
pub use crate::layers::compressors::core::errors::compression::Error;