//! Provides errors for the Reed-Solomon error `correction` module.

// -------------------------------------------------------------------------------------------------
//
/// Represents an error from the Reed-Solomon error `correction` module.
///
/// These errors are encountered when:
/// * Protecting data with parity shards,
/// * Verifying the integrity of data read from the database, or
/// * Recovering corrupted data using parity shards.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// [Reed-Solomon]((https://crates.io/crates/reed-solomon-erasure) encoding or decoding
    /// [error](https://docs.rs/reed-solomon-erasure/latest/reed_solomon_erasure/enum.Error.html).
    #[error(transparent)]
    ReedSolomon(#[from] reed_solomon_erasure::Error),

    /// Error processing layer parameters. This may indicate data corruption, a database version
    /// mismatch, or misconfiguration.
    #[error("error processing layer parameters")]
    Parameters { #[from] #[source] source: crate::layers::correctors::reed_solomon::parameters::Error },
}