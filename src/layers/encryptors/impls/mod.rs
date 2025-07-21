//! `Encryptor` encryption implementations. A single implementation is selected at compile-time in
//! the host application's `Cargo.toml` file.

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
/// `ChaCha20` has been selected as the `ActiveEncryptor` using `Cargo.toml` feature.
pub use crate::layers::encryptors::impls::chacha20::ChaCha20 as ActiveEncryptor;

#[cfg(feature = "encrypt-chacha20")]
/// Key size for the active encryptor. `ChaCha20Poly1305`'s key size is `32`-bytes or `256`-bits.
pub use crate::layers::encryptors::impls::chacha20::KEY_SIZE;

#[cfg(feature = "encrypt-chacha20")]
/// Nonce size for the active encryptor. `ChaCha20Poly1305`'s nonce size is `12`-bytes or `96`-bits.
pub use crate::layers::encryptors::impls::chacha20::NONCE_SIZE;

#[cfg(all(feature = "encrypt-chacha20", feature = "kdf-sha256"))]
/// Digest to be used for hashing text passwords when working with the `ring` crate.
pub use crate::layers::encryptors::impls::chacha20::RING_SHA256_DIGEST;

#[cfg(feature = "encrypt-aes-gcm")]
mod aes_gcm;

#[cfg(feature = "encrypt-aes-gcm")]
/// `AesGcm` has been selected as the `ActiveEncryptor` using `Cargo.toml` feature.
pub use crate::layers::encryptors::impls::aes_gcm::AesGcm as ActiveEncryptor;

#[cfg(feature = "encrypt-aes-gcm")]
/// Key size for the active encryptor. `AesGcm`'s key size is `32`-bytes or `256`-bits.
pub use crate::layers::encryptors::impls::aes_gcm::KEY_SIZE;

#[cfg(feature = "encrypt-aes-gcm")]
/// Nonce size for the active encryptor. `AesGcm`'s nonce size is `12`-bytes or `96`-bits.
pub use crate::layers::encryptors::impls::aes_gcm::NONCE_SIZE;

#[cfg(all(feature = "encrypt-aes-gcm", feature = "kdf-sha256"))]
/// Digest to be used for hashing text passwords when working with the `ring` crate.
pub use crate::layers::encryptors::impls::aes_gcm::RING_SHA256_DIGEST;