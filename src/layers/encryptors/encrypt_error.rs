/// An error returned from the encryption implementation while encrypting data.
///
/// This includes errors for out of memory, etc.
#[derive(thiserror::Error, Debug)]
pub enum EncryptError {
    /// Error returned from the [chacha20poly1305](https://crates.io/crates/chacha20poly1305) crate.
    ///
    /// To understand the possible errors this encryption may produce, please refer to the official
    /// documentation: <https://docs.rs/chacha20poly1305>
    #[cfg(feature = "encrypt-chacha20")]
    #[error("chacha20poly1305 encryption failed")]
    ChaCha20 { #[from] #[source] source: chacha20poly1305::Error },

    /// Error returned from the [aes-gcm](https://crates.io/crates/aes-gcm) crate.
    ///
    /// To understand the possible errors this encryption may produce, please refer to the official
    /// documentation: <https://docs.rs/aes-gcm>
    #[cfg(feature = "encrypt-aes-gcm")]
    #[error("aes-gcm encryption failed")]
    AesGcm { #[from] #[source] source: aes_gcm::Error },

    /// Error processing layer parameters. This may indicate data corruption, a database version
    /// mismatch, or misconfiguration.
    #[error("error processing layer parameters")]
    Parameters { #[from] #[source] source: crate::layers::encryptors::parameters::Error },
}