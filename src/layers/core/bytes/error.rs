//! An error that occurs when attempting to access a `ValueBuf`.

// -------------------------------------------------------------------------------------------------
//
/// An error that occurs when attempting to access a `ValueBuf`.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Raw bytes provided, but this type expects to serialize typed values.
    ///
    /// This type is configured to serialize on write (using `OnWrite` or `Both` direction), which
    /// means it expects a typed value like `Value::Value(my_struct)`, not raw bytes.
    ///
    /// # Resolutions
    ///
    /// * Use `Value::Value(typed_data)` instead of `Value::Bytes`.
    /// * Or change the type's `Direction` to `None`/`OnRead` in the `Serializable` trait
    ///   implementation to accept raw bytes.
    #[error("expected typed value, got raw bytes - this type serializes on write")]
    ExpectedTypedValueGotBytes,

    /// Typed value provided, but this type expects raw bytes only.
    ///
    /// This type is configured to skip serialization (using `None` or `OnRead` direction),
    /// which means it expects raw bytes like `Value::Bytes`, not typed values that need
    /// serializing.
    ///
    /// # Resolutions
    ///
    /// * Use `Value::Bytes(raw_bytes)` instead of `Value::Value` or `Value::ValueRef`.
    /// * Or change the type's direction to `OnWrite`/`Both` in the `Serializable` trait
    ///   implementation to enable serialization.
    #[error("expected raw bytes, got typed value - this type doesn't serialize")]
    ExpectedBytesGotTypedValue,

    /// Attempted to read beyond the available data in the buffer.
    ///
    /// This indicates:
    /// * Truncated data: The data was cut off during storage or transmission.
    /// * Incorrect length information: The metadata about data size is wrong.
    /// * Protocol mismatch: Reading with wrong expectations about data format.
    ///
    /// Troubleshooting steps:
    /// 1. Verify the complete data was stored and retrieved.
    /// 2. Check that the data format matches what the reader expects.
    /// 3. Ensure no data was lost during transmission or storage.
    #[error(
        "attempted to read {bytes_read} bytes \
        but only {bytes_remaining} bytes remain in buffer"
    )]
    EndOfBuffer {
        bytes_read: usize,
        bytes_remaining: usize
    },

    /// An error was encountered while attempting to read a layer descriptor.
    ///
    /// This typically occurs due to:
    /// * Data corruption: The stored data may have been partially overwritten or corrupted.
    /// * Version mismatch: The data was written with a different version of this library.
    /// * Wrong data format: The data may not be in the expected format for this system.
    ///
    /// Troubleshooting steps:
    /// 1. Verify the data source is correct and hasn't been modified.
    /// 2. Check if the data was created with a compatible version of this library.
    /// 3. Ensure you're reading the data with the same configuration used to write it.
    #[error("error occurred while reading a layer descriptor")]
    Descriptor { #[from] #[source] source: crate::layers::core::descriptors::Error },

    /// A generic error occurred during processing.
    ///
    /// This is typically used for:
    /// - **Configuration mismatches**: The system configuration doesn't match the data
    /// - **Unsupported operations**: Attempting operations not supported by the current setup
    /// - **Internal inconsistencies**: Unexpected internal state during processing
    ///
    /// **Troubleshooting steps:**
    /// 1. Check that your system configuration matches the data requirements
    /// 2. Verify all required features are enabled in your `Cargo.toml`
    /// 3. Review the operation being performed and ensure it's supported
    #[error("processing error occurred")]
    Other,

    /// Failed to serialize data into the configured format.
    ///
    /// This can happen when:
    /// * Unsupported data types: The serializer doesn't support certain types in your data.
    /// * Data too large: The data exceeds the serializer's size limits.
    /// * Circular references: The data contains references that create cycles.
    ///
    /// Troubleshooting steps:
    /// 1. Check the serializer backend documentation for supported types and limitations.
    /// 2. Verify your data structure is compatible with the chosen serializer.
    /// 3. Consider using a different serializer if the current one doesn't meet your needs.
    #[error("serialization failed")]
    Serialize { #[from] #[source] source: crate::layers::serializers::SerializeError },

    /// Failed to deserialize data from the stored format.
    ///
    /// Common causes include:
    /// * Schema mismatch: The data structure has changed since the data was written.
    /// * Corrupted data: The serialized data has been damaged or truncated.
    /// * Type incompatibility: Trying to deserialize into an incompatible type.
    ///
    /// Troubleshooting steps:
    /// 1. Ensure the target type matches the original data structure.
    /// 2. Verify the data hasn't been corrupted during transmission or storage.
    /// 3. Consider using a more flexible deserialization approach if schema evolved.
    #[error("deserialization failed")]
    Deserialize { #[from] #[source] source: crate::layers::serializers::DeserializeError },

    /// The data was serialized with a different method than the system is configured to use.
    ///
    /// This occurs when:
    /// * Configuration change: The serializer was changed after data was written.
    /// * Environment mismatch: Reading data in a different environment than where it was written.
    /// * Feature flags: Different serialization features are enabled between write and read.
    ///
    /// Resolution steps:
    /// 1. Change your configuration: Update your `Cargo.toml` to enable the `{layer_serializer}`
    ///    feature.
    /// 2. Migrate the data: Re-serialize existing data using the new `{configured_serializer}`
    ///    method
    ///
    /// Example fix for Cargo.toml:
    /// ```toml
    /// [dependencies]
    /// your-crate = {{ features = ["{layer_serializer}"] }}
    /// ```
    #[error(
        "serialization method mismatch: data uses '{layer_serializer}' \
        but system is configured for '{configured_serializer}'"
    )]
    SerializationMismatch {
        layer_serializer: crate::layers::serializers::Method,
        configured_serializer: crate::layers::serializers::Method
    },

    /// Failed to compress data during the compression layer processing.
    ///
    /// This can occur due to:
    /// * Memory limitations: Insufficient memory available for compression.
    /// * Data characteristics: Some data types compress poorly or cause issues.
    ///
    /// Troubleshooting steps:
    /// 1. Check available system memory and consider reducing data size.
    /// 2. Try a different compression algorithm if multiple options are available.
    #[error("compression failed")]
    Compress { #[from] #[source] source: crate::layers::compressors::CompressError },

    /// Failed to decompress data during the decompression layer processing.
    ///
    /// Common causes include:
    /// * Corrupted compressed data: The compressed data has been damaged.
    /// * Incomplete data: The compressed data was truncated or partially lost.
    ///
    /// Troubleshooting steps:
    /// 1. Verify the data hasn't been corrupted during transmission or storage.
    /// 2. Test with known-good compressed data to isolate the issue.
    #[error("decompression failed")]
    Decompress {
        #[from]
        #[source]
        source: crate::layers::compressors::DecompressError
    },

    /// The data was compressed with a different method than the system is configured to use.
    ///
    /// Possible resolution steps:
    /// * Update configuration: Enable the `{layer_compressor}` feature in your `Cargo.toml`
    /// * Recompress data: Convert existing data to use the `{configured_compressor}` method
    #[error(
        "compression method mismatch: data uses '{layer_compressor}' \
        but system is configured for '{configured_compressor}'"
    )]
    CompressionMismatch {
        layer_compressor: crate::layers::compressors::Method,
        configured_compressor: crate::layers::compressors::Method
    },

    /// Failed to encrypt data during the encryption layer processing.
    ///
    /// This typically indicates:
    /// * Invalid encryption key: The provided key is malformed or incorrect length.
    /// * Algorithm constraints: The data doesn't meet the encryption algorithm's requirements.
    /// * System resources: Insufficient memory or entropy for encryption.
    ///
    /// Troubleshooting steps:
    /// 1. Verify the encryption key is the correct length and format.
    /// 2. Check that the system has sufficient entropy available.
    /// 3. Ensure the data size is compatible with the encryption method.
    #[error("encryption failed")]
    Encrypt {
        #[from]
        #[source]
        source: crate::layers::encryptors::EncryptError
    },

    /// Failed to decrypt data during the decryption layer processing.
    ///
    /// Common causes include:
    /// * Wrong decryption key: The key doesn't match the one used for encryption.
    /// * Corrupted encrypted data: The encrypted data has been modified or damaged.
    /// * Authentication failure: The data failed integrity checks during decryption.
    ///
    /// Troubleshooting steps:
    /// 1. Verify you're using the correct decryption key.
    /// 2. Check that the encrypted data hasn't been modified or corrupted.
    /// 3. Ensure the decryption method matches the encryption method used.
    #[error("decryption failed")]
    Decrypt {
        #[from]
        #[source]
        source: crate::layers::encryptors::DecryptError
    },

    /// The data was encrypted with a different method than the system is configured to use.
    ///
    /// Resolution steps:
    /// 1. Update configuration: Enable the `{layer_encryptor}` feature in your `Cargo.toml`.
    /// 2. Re-encrypt data: Decrypt with the original method and re-encrypt with the new method.
    /// 3. Key management: Ensure proper key management for the transition.
    #[error(
        "encryption method mismatch: data uses '{layer_encryptor}' \
        but system is configured for '{configured_encryptor}'"
    )]
    EncryptionMismatch {
        layer_encryptor: crate::layers::encryptors::Method,
        configured_encryptor: crate::layers::encryptors::Method
    },

    /// Failed to protect data.
    ///
    /// This can occur due to:
    /// * Memory limitations: Insufficient memory available for compression.
    ///
    /// Troubleshooting steps:
    /// 1. Check available system memory and consider reducing data size.
    #[error("data protection failed")]
    Protect { #[from] #[source] source: crate::layers::correctors::ProtectError },

    /// Failed to validate checksums or recover data.
    ///
    /// This can happen due to:
    /// * Severe data corruption: The corruption exceeds the error correction capability.
    /// * Insufficient parity data: Not enough error correction information was stored.
    /// * Algorithm limitations: The error correction method has reached its limits.
    ///
    /// Troubleshooting steps:
    /// 1. Check if the corruption is within the error correction method's capability.
    /// 2. Verify that all parity data is available and uncorrupted.
    /// 3. Consider using stronger error correction for future data.
    #[error("data recovery failed")]
    Recover { #[from] #[source] source: crate::layers::correctors::RecoverError },

    /// The data was protected with a different error correction method than the system is
    /// configured to use.
    ///
    /// Resolution steps:
    /// 1. Update configuration: Enable the `{layer_corrector}` feature in your `Cargo.toml`.
    /// 2. Data migration: Re-generate error correction codes using the new method.
    #[error(
        "error correction method mismatch: data uses '{layer_corrector}' \
        but system is configured for '{configured_corrector}'"
    )]
    CorrectionMismatch {
        layer_corrector: crate::layers::correctors::Method,
        configured_corrector: crate::layers::correctors::Method
    },

    /// The wrong type of data was provided.
    ///
    /// This typically happens if bytes were provided for serialization, or a typed value was
    /// provided for deserialization.
    ///
    /// This likely represents an error in the host application.
    #[error("wrong type of data was provided for serialization or deserialization")]
    ValueOrBytes { #[from] #[source] source: crate::layers::core::value_or_bytes::Error },
}