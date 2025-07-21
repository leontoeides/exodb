//! Contains the error type returned from the nonce handling implementation.

// -------------------------------------------------------------------------------------------------
//
/// An error returned from the nonce handling implementation.
#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum Error {
    /// The provided [cryptographic
    /// nonce](https://en.wikipedia.org/wiki/Cryptographic_nonce) was the wrong size.
    ///
    /// # Notes
    ///
    /// * Different encryption ciphers can have different nonce size requirements. Consult the
    ///   documentation of the encryptor backend you are using for more details.
    ///
    /// * A nonce in encryption is a unique, random or pseudo-random number used only once in a
    ///   cryptographic communication to ensure security by preventing replay attacks and ensuring
    ///   that identical plaintexts produce different ciphertexts.
    #[error(
        "invalid nonce: \
        expected {expected_size:?} bytes \
        but {provided_size:?} bytes were provided"
    )]
    InvalidNonceLength {
        expected_size: usize,
        provided_size: usize,
    },
}