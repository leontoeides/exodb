//! Encryption algorithms for securing stored data.

mod core;
pub use crate::layers::encryptors::core::DecryptError;
pub use crate::layers::encryptors::core::EncryptError;
pub use crate::layers::encryptors::core::Encryptable;
pub use crate::layers::encryptors::core::Encryptor;
pub use crate::layers::encryptors::core::Error;
pub use crate::layers::encryptors::core::KeyBytes;
pub use crate::layers::encryptors::core::Method;
pub use crate::layers::encryptors::core::Nonce;

mod impls;
pub use crate::layers::encryptors::impls::ActiveEncryptor;