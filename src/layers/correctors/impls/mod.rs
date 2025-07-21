//! `Corrector` error correction implementations. A single implementation is selected at
//! compile-time in the host application's `Cargo.toml` file.

// -------------------------------------------------------------------------------------------------
//
// Corrector Feature Guard

/// Helper macro: counts how many of the listed features are turned on.
macro_rules! count_features {
    ($($feat:literal),* $(,)?) => {
        0_usize $(+ cfg!(feature = $feat) as usize)*
    };
}

const _CORRECTOR_FEATURE_COUNT: usize = count_features!(
    "ecc-reed-solomon",
);

const _: () = {
    assert!(
        // Only one corrector feature can be enabled. To fix: 1. open `Cargo.toml` file, 2. find
        // `[dependencies]` section and where `atlatl` is, 3. ensure only one corrector is enabled.
        !(_CORRECTOR_FEATURE_COUNT > 1),
        "Multiple corrector features enabled! Please enable only one of: \
        `ecc-reed-solomon`",
    );
};

// -------------------------------------------------------------------------------------------------
//
// Corrector Implementations

#[cfg(feature = "ecc-reed-solomon")]
pub mod reed_solomon;

#[cfg(feature = "ecc-reed-solomon")]
/// `ReedSolomon` has been selected as the `ActiveCorrector` using `Cargo.toml` features.
pub use crate::layers::correctors::impls::reed_solomon::ReedSolomon as ActiveCorrector;