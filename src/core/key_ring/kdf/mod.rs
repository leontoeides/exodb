//! A cryptographic Key Derivation Function (KDF) is a process that generates secure secret keys
//! from a source of initial keying material, such as a password or a master key.

mod error;
pub use crate::layers::encryptors::kdf::error::Error;

mod key;
pub use crate::layers::encryptors::kdf::key::Key;

// -------------------------------------------------------------------------------------------------
//
// KDF (Key Derivation Function) Feature Guard

/// Helper macro: counts how many of the listed features are turned on.
macro_rules! count_features {
    ($($feat:literal),* $(,)?) => {
        0_usize $(+ cfg!(feature = $feat) as usize)*
    };
}

const _ENCRYPTOR_FEATURE_COUNT: usize = count_features!(
    "kdf-blake3",
    "kdf-sha256",
);

const _: () = {
    assert!(
        // Only one KDF feature can be enabled. To fix: 1. open the `Cargo.toml` file, 2. find the
        // `[dependencies]` section where `atlatl` is declared, 3. ensure only one KDF is enabled.
        !(_ENCRYPTOR_FEATURE_COUNT > 1),
        "Multiple KDF features enabled! Enable only one of: \
        `kdf-blake3`, or \
        `kdf-sha256`",
    );
};

// -------------------------------------------------------------------------------------------------
//
// KDF (Key Derivation Function) Implementations

#[cfg(feature = "kdf-blake3")]
mod blake3;

#[cfg(feature = "kdf-sha256")]
mod sha256;