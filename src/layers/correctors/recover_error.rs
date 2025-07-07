/// An error returned from the error correction implementation while checking the integrity of
/// data or during the data recovery process.
///
/// This includes errors for corrupted or malformed data, etc.
#[derive(thiserror::Error, Debug)]
pub enum RecoverError {
    /// Error returned from the
    /// [reed_solomon_erasure](https://crates.io/crates/reed_solomon_erasure) crate.
    ///
    /// To understand the possible errors that this ECC corrector may produce, please refer to the
    /// official documentation: <https://docs.rs/reed_solomon_erasure>
    #[cfg(feature = "ecc-reed-solomon")]
    #[error(
        "error validating checksums or \
        reed-solomon recovery of data from parity shards failed
    ")]
    ReedSolomon { #[from] #[source] source: crate::layers::correctors::reed_solomon::Error },
}