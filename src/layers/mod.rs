//! Data transformation primitives: serialization, compression, correction, encryption.

pub mod descriptors;
pub use crate::layers::descriptors::Descriptor;
pub use crate::layers::descriptors::Direction;
pub use crate::layers::descriptors::Layer;

pub mod error;
pub use crate::layers::error::Error;

pub mod tail_reader;
pub use crate::layers::tail_reader::TailReader;

mod value;
pub use crate::layers::value::Value;

mod value_buf;
pub use crate::layers::value_buf::ValueBuf;

// -------------------------------------------------------------------------------------------------
//
// Layer Implementations

#[cfg(feature = "serializers")]
pub mod serializers;

#[cfg(feature = "serializers")]
pub use crate::layers::serializers::Serializer;

#[cfg(feature = "serializers")]
pub use crate::layers::serializers::Serializable;

#[cfg(feature = "compressors")]
pub mod compressors;

#[cfg(feature = "compressors")]
pub use crate::layers::compressors::Compressor;

#[cfg(feature = "compressors")]
pub use crate::layers::compressors::Compressible;

#[cfg(feature = "correctors")]
pub mod correctors;

#[cfg(feature = "correctors")]
pub use crate::layers::correctors::Corrector;

#[cfg(feature = "correctors")]
pub use crate::layers::correctors::Correctable;

#[cfg(feature = "encryptors")]
pub mod encryptors;

#[cfg(feature = "encryptors")]
pub use crate::layers::encryptors::Encryptor;

#[cfg(feature = "encryptors")]
pub use crate::layers::encryptors::Encryptable;