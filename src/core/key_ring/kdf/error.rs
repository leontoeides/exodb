//! Contains the error type returned from the key handling implementation.

// -------------------------------------------------------------------------------------------------
//
/// An error returned from the key handling implementation.
#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum Error {
    /// The provided [key](https://en.wikipedia.org/wiki/Key_(cryptography)) was the wrong size.
    ///
    /// # Notes
    ///
    /// * Different encryption ciphers can have different key size requirements. Consult the
    ///   documentation of the encryptor backend you are using for more details.
    ///
    /// * An encryption key is a string of characters or series of bytes used to lock (encrypt) or
    ///   unlock (decrypt) data, keeping it secure from unauthorized access
    #[error(
        "invalid key: \
        expected {expected_size:?} bytes \
        but {provided_size:?} bytes were provided"
    )]
    InvalidKeyLength {
        expected_size: usize,
        provided_size: usize,
    },
}