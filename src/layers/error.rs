/// An error returned from the layer value-pipeline implementation.
///
/// This includes errors for out of memory, corrupted or malformed data, etc.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// May represent a protection, data integrity check, or data recovery failure.
    #[error("ECC data correction failure")]
    Correction(#[from] crate::layers::correctors::Error),

    /// May represent an encryption or descryption failure.
    #[error("encryption failure")]
    Encryption(#[from] crate::layers::encryptors::Error),

    /// May represent a compression or decompression failure.
    #[error("compression failure")]
    Compression(#[from] crate::layers::compressors::Error),

    /// May represent a serialization or deserialization failure.
    #[error("serialization failure")]
    Serialization(#[from] crate::layers::serializers::Error),

    #[error("Layer mismatch or incoherent configuration: {0}")]
    LayerMismatch(String),

    #[error("Layer processing failed with unknown cause")]
    Unknown,
}