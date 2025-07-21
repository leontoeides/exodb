//! Support for data transformation layers: serialization, compression, correction, and encryption.

pub mod core;

mod error;
pub use crate::layers::error::Error;

// -------------------------------------------------------------------------------------------------
//
// Serialization Layer

#[cfg(feature = "serializers")]
pub mod serializers;

#[cfg(feature = "serializers")]
pub use crate::layers::serializers::Serializable;

#[cfg(feature = "serializers")]
pub use crate::layers::serializers::Serializer;

// -------------------------------------------------------------------------------------------------
//
// Compression Layer

#[cfg(feature = "compressors")]
pub mod compressors;

#[cfg(feature = "compressors")]
pub use crate::layers::compressors::ActiveCompressor;

#[cfg(feature = "compressors")]
pub use crate::layers::compressors::Compressor;

#[cfg(feature = "compressors")]
pub use crate::layers::compressors::Compressible;

// -------------------------------------------------------------------------------------------------
//
// Error Correction Layer

#[cfg(feature = "correctors")]
pub mod correctors;

#[cfg(feature = "correctors")]
pub use crate::layers::correctors::ActiveCorrector;

#[cfg(feature = "correctors")]
pub use crate::layers::correctors::Corrector;

#[cfg(feature = "correctors")]
pub use crate::layers::correctors::Correctable;

// -------------------------------------------------------------------------------------------------
//
// Encryption Layer

#[cfg(feature = "encryptors")]
pub mod encryptors;

#[cfg(feature = "encryptors")]
pub use crate::layers::encryptors::ActiveEncryptor;

#[cfg(feature = "encryptors")]
pub use crate::layers::encryptors::Encryptor;

#[cfg(feature = "encryptors")]
pub use crate::layers::encryptors::Encryptable;