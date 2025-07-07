//! An error that occurs when attempting to access a `ValueBuf`.

// -------------------------------------------------------------------------------------------------
//
/// An error that occurs when attempting to access a `ValueBuf`.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Insufficient data in buffer to perform requested read.
    ///
    /// This typically indicates a corrupted, truncated, or malformed buffer where the expected
    /// data is missing from the end of the byte array.
    #[error(
        "attempted to read {bytes_read} bytes from buffer \
        but only {bytes_remaining} bytes remain"
    )]
    EndOfBuffer {
        bytes_read: usize,
        bytes_remaining: usize
    },

    /// An error was encountered while attempting to read a layer descriptor.
    ///
    /// This typically occurs due to data corruption or database version mismatches.
    #[error("error occured while reading a layer descriptor")]
    Descriptor { #[from] #[source] source: crate::layers::descriptors::Error },

    /// Correction error. An error was encountered while attempting to validate a layer's checksums
    /// or recover corrupted data.
    #[error("validation or data recovery failed")]
    Recover { #[from] #[source] source: crate::layers::correctors::RecoverError },

    /// Encryption error. An error was encountered while attempting to decrypt data.
    #[error("decryption failed")]
    Decrypt { #[from] #[source] source: crate::layers::encryptors::DecryptError },

    /// Compression error. An error was encountered while attempting to decompress data.
    #[error("decompression failed")]
    Decompress { #[from] #[source] source: crate::layers::compressors::DecompressError },

    /// Serialization error. An error was encountered while attempting to deserialize data.
    #[error("deserialization failed")]
    Deserialize { #[from] #[source] source: crate::layers::serializers::DeserializeError },

    #[error("other")]
    Other,
}