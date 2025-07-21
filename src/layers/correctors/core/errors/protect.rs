//! Contains the error type returned from the error encoding implementation while protecting data.

// -------------------------------------------------------------------------------------------------
//
/// An error returned from the error encoding implementation while protecting data.
///
/// This includes errors for out of memory, etc.
#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum Error {
    /// Error returned from the
    /// [reed_solomon_erasure](https://crates.io/crates/reed_solomon_erasure) crate.
    ///
    /// To understand the possible errors that this ECC corrector may produce, please refer to the
    /// official documentation: <https://docs.rs/reed_solomon_erasure>
    #[cfg(feature = "ecc-reed-solomon")]
    #[error("reed-solomon protection of data with parity shards failed")]
    ReedSolomon {
        #[from]
        #[source]
        source: crate::layers::correctors::impls::reed_solomon::Error
    },
}