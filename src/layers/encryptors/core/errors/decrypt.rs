//! Contains the error type returned from the encryption implementation while decrypting data.

// -------------------------------------------------------------------------------------------------
//
/// An error returned from the encryption implementation while decrypting data.
///
/// This includes errors for corrupted or malformed data, etc.
#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum Error {
    /// Error returned from the [chacha20poly1305](https://crates.io/crates/chacha20poly1305) crate.
    ///
    /// This error typically means that the provided key was invalid for the given ciphertext.
    /// However, it may also indicate that the encrypted data itself has been corrupted due to bit
    /// rot, incorrect nonce usage, or tampering.
    ///
    /// Atlatl does not attempt to distinguish between these causes, and intentionally surfaces this
    /// generic `AccessDenied` error to preserve abstraction boundaries and avoid leaking
    /// information that could aid an attacker.
    ///
    /// # Common Causes
    ///
    /// * A key was provided, but does not match the one originally used to encrypt the value.
    /// * The encrypted value or associated metadata was corrupted or truncated.
    /// * The encryption method or context was changed in an incompatible way.
    ///
    /// # Suggestions
    ///
    /// * Ensure that the `KeyRing` contains the correct key for the value and context.
    /// * Confirm that the database or storage medium is not experiencing data corruption.
    /// * Avoid changing encryption parameters (for example, BLAKE3 or AES-256 context string)
    ///   between versions without planning a migration strategy.
    ///
    /// To understand the possible errors this encryption may produce, please refer to the official
    /// documentation: <https://docs.rs/chacha20poly1305>
    #[cfg(feature = "encrypt-chacha20")]
    #[error("access denied: decryption failed")]
    AccessDenied { #[from] #[source] source: chacha20poly1305::Error },

    /// Error returned from the [aes-gcm](https://crates.io/crates/aes-gcm) crate.
    ///
    /// This error typically means that the provided key was invalid for the given ciphertext.
    /// However, it may also indicate that the encrypted data itself has been corrupted due to bit
    /// rot, incorrect nonce usage, or tampering.
    ///
    /// Atlatl does not attempt to distinguish between these causes, and intentionally surfaces this
    /// generic `AccessDenied` error to preserve abstraction boundaries and avoid leaking
    /// information that could aid an attacker.
    ///
    /// # Common Causes
    ///
    /// * A key was provided, but does not match the one originally used to encrypt the value.
    /// * The encrypted value or associated metadata was corrupted or truncated.
    /// * The encryption method or context was changed in an incompatible way.
    ///
    /// # Suggestions
    ///
    /// * Ensure that the `KeyRing` contains the correct key for the value and context.
    /// * Confirm that the database or storage medium is not experiencing data corruption.
    /// * Avoid changing encryption parameters (for example, BLAKE3 or AES-256 context string)
    ///   between versions without planning a migration strategy.
    ///
    /// To understand the possible errors this encryption may produce, please refer to the official
    /// documentation: <https://docs.rs/aes-gcm>
    #[cfg(feature = "encrypt-aes-gcm")]
    #[error("access denied: decryption failed")]
    AccessDenied { #[from] #[source] source: aes_gcm::Error },

    /// Error parsing layer parameters. This may indicate data corruption or a database version
    /// mismatch.
    #[error("error parsing layer parameters")]
    Parameters { #[from] #[source] source: crate::layers::encryptors::core::parameters::Error },
}