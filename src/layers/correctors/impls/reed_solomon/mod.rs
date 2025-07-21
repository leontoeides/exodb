//! Support for [Darren Li](https://github.com/darrenldl), [Michael
//! Vines](https://github.com/mvines), and [Nazar Mokrynskyi](https://github.com/nazar-pc)'s
//! [reed-solomon-erasure](https://crates.io/crates/reed-solomon-erasure) crate.

mod corrector;

mod error;
pub use crate::layers::correctors::impls::reed_solomon::error::Error;

pub mod parameters;
pub use crate::layers::correctors::impls::reed_solomon::parameters::Parameters;

#[allow(clippy::module_inception)]
mod reed_solomon;
pub use crate::layers::correctors::impls::reed_solomon::reed_solomon::ReedSolomon;

mod tests;

// -------------------------------------------------------------------------------------------------
//
// Constants

/// Minimum data length in bytes. Data smaller than this will not be encoded or decoded.
const DATA_LEN_MIN: usize = 3;

/// Maximum data length in bytes. Data larger than this will not be encoded or decoded.
const DATA_LEN_MAX: usize = 1_073_741_824; // 1 Gygabite, safe for GF(2⁸)