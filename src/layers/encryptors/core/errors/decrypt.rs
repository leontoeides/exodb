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
    /// To understand the possible errors this encryption may produce, please refer to the official
    /// documentation: <https://docs.rs/chacha20poly1305>
    #[cfg(feature = "encrypt-chacha20")]
    #[error("chacha20poly1305 decryption failed")]
    ChaCha20 { #[from] #[source] source: chacha20poly1305::Error },

    /// Error returned from the [aes-gcm](https://crates.io/crates/aes-gcm) crate.
    ///
    /// To understand the possible errors this encryption may produce, please refer to the official
    /// documentation: <https://docs.rs/aes-gcm>
    #[cfg(feature = "encrypt-aes-gcm")]
    #[error("aes-gcm decryption failed")]
    AesGcm { #[from] #[source] source: aes_gcm::Error },

    /// Error parsing layer parameters. This may indicate data corruption or a database version
    /// mismatch.
    #[error("error parsing layer parameters")]
    Parameters { #[from] #[source] source: crate::layers::encryptors::core::parameters::Error },
}