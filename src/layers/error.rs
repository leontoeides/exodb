//! Contains the error type returned from the layer value-pipeline implementation.

// -------------------------------------------------------------------------------------------------
//
/// An error returned from the layer value-pipeline implementation.
///
/// This includes errors for out of memory, corrupted or malformed data, etc.
#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum Error {
    /// A protection, data integrity check, or data recovery failure.
    #[error("ECC data correction failure")]
    Correction(#[from] crate::layers::correctors::Error),

    /// An encryption or descryption failure.
    #[error("encryption failure")]
    Encryption(#[from] crate::layers::encryptors::Error),

    /// A compression or decompression failure.
    #[error("compression failure")]
    Compression(#[from] crate::layers::compressors::Error),

    /// A serialization or deserialization failure.
    #[error("serialization failure")]
    Serialization(#[from] crate::layers::serializers::Error),
}