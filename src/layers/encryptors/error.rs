/// An error returned from the encryption layer.
///
/// This includes errors for out of memory, corrupted or malformed data, etc.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// An error was encountered while encrypting data.
    #[error("encryption of data failed")]
    Encrypt { #[from] #[source] source: crate::layers::encryptors::EncryptError },

    /// An error was encountered while decrypting data.
    #[error("decryption of data failed")]
    Decrypt { #[from] #[source] source: crate::layers::encryptors::DecryptError },
}