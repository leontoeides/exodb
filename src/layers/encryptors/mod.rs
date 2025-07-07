//! Encryption algorithms for securing data. Applied after compression, using caller-supplied keys
//! and nonces.

mod decrypt_error;
pub use crate::layers::encryptors::decrypt_error::DecryptError;

mod encrypt_error;
pub use crate::layers::encryptors::encrypt_error::EncryptError;

mod encryptable;
pub use crate::layers::encryptors::encryptable::Encryptable;

mod encryptor;
pub use crate::layers::encryptors::encryptor::Encryptor;

mod error;
pub use crate::layers::encryptors::error::Error;

mod method;
pub use crate::layers::encryptors::method::Method;

mod parameters;
use crate::layers::encryptors::parameters::Parameters;

// -------------------------------------------------------------------------------------------------
//
// Encryptor Feature Guard

/// Helper macro: counts how many of the listed features are turned on.
macro_rules! count_features {
    ($($feat:literal),* $(,)?) => {
        0_usize $(+ cfg!(feature = $feat) as usize)*
    };
}

const _ENCRYPTOR_FEATURE_COUNT: usize = count_features!(
    "encrypt-aes-gcm",
    "encrypt-chacha20",
);

const _: () = {
    assert!(
        // Only one serializer feature can be enabled. To fix: 1. open the `Cargo.toml` file, 2. find the
        // `[dependencies]` section where `atlatl` is declared, 3. ensure only one serializer is enabled.
        !(_ENCRYPTOR_FEATURE_COUNT > 1),
        "Multiple encryptor features enabled! Enable only one of: \
	    `encrypt-aes-gcm`, or \
	    `encrypt-chacha20`",
    );
};

// -------------------------------------------------------------------------------------------------
//
// Encryption Implementations

#[cfg(feature = "encrypt-chacha20")]
mod chacha20;

#[cfg(feature = "encrypt-chacha20")]
pub use crate::layers::encryptors::chacha20::ChaCha20;

#[cfg(feature = "encrypt-aes-gcm")]
mod aes_gcm;

#[cfg(feature = "encrypt-aes-gcm")]
pub use crate::layers::encryptors::aes_gcm::AesGcm;