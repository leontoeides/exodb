//! Common types and traits that are used across the various encryption implementations.

mod encryptable;
pub use crate::layers::encryptors::core::encryptable::Encryptable;

mod encryptor;
pub use crate::layers::encryptors::core::encryptor::Encryptor;

mod errors;
pub use crate::layers::encryptors::core::errors::DecryptError;
pub use crate::layers::encryptors::core::errors::EncryptError;
pub use crate::layers::encryptors::core::errors::Error;

mod key_bytes;
pub use crate::layers::encryptors::core::key_bytes::KeyBytes;

mod method;
pub use crate::layers::encryptors::core::method::Method;

mod nonce;
pub use crate::layers::encryptors::core::nonce::Nonce;

mod parameters;
pub(super) use crate::layers::encryptors::core::parameters::Parameters;