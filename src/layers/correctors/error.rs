/// An error returned from the ECC error correction layer.
///
/// This includes errors for out of memory, corrupted or malformed data, etc.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// An error was encountered while protecting data.
    #[error("protection of data failed")]
    Protect { #[from] #[source] source: crate::layers::correctors::ProtectError },

    /// An error was encountered while checking data integrity or recovering data.
    #[error("error occured while checking data integrity or during recovery of corrupted data")]
    Recover { #[from] #[source] source: crate::layers::correctors::RecoverError },
}