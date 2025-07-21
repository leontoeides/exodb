use crate::layers::core::Bytes;
use crate::layers::correctors::impls::reed_solomon::ReedSolomon;
use crate::layers::correctors::{Correctable, Method};

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl<'b, V: Correctable> crate::layers::correctors::Corrector<'b, V> for ReedSolomon<V> {
    /// Returns the error correction method that the current `Corrector` trait implements.
    ///
    /// This enables runtime identification of the error correction algorithm in use, allowing
    /// applications to log compression details, or store metadata about how data was processed in
    /// the data pipeline.
    const METHOD: Method = Method::ReedSolomon;

    /// Reed-Solomon protection works by adding parity data, which are extra symbols calculated from
    /// the original data, to enable the recovery of lost or corrupted data symbols during
    /// transmission or storage.
    ///
    /// If protection is unnecessary or cannot be applied, the original `Bytes` will be returned
    /// untouched.
    ///
    /// # Arguments
    ///
    /// * `unprotected_bytes` · The original data to be protected against corruption, wrapped in a
    ///   `Bytes` that may reference borrowed application bytes.
    ///
    /// # Errors
    ///
    /// Consult the documentation of the corrector backend you are using for more detail on
    /// protection behavior and potential limitations: <http://docs.rs/reed-solomon-erasure>
    ///
    /// # Generics & Lifetimes
    ///
    /// * `V` generic represents the user's value type, for example: `User`, `String`, etc.
    /// * `b` lifetime represents bytes potentially being borrowed from the `redb` database.
    #[inline]
    fn protect(
        unprotected_bytes: Bytes<'b>
    ) -> Result<Bytes<'b>, crate::layers::correctors::ProtectError> {
        Ok(Self::add_parity(unprotected_bytes)?)
    }

    /// Reed-Solomon recovery works by encoding data into blocks with additional parity blocks,
    /// allowing the reconstruction of lost or corrupted data as long as the number of errors does
    /// not exceed the number of parity blocks.
    ///
    /// CRC-32 is used to detect data corruption by generating a 32-bit checksum that can identify
    /// changes in the data during transmission or storage.
    ///
    /// # Errors
    ///
    /// This method may fail for several reasons, including:
    ///
    /// * Input bytes are corrupted or malformed.
    ///
    /// Consult the documentation of the corrector backend you are using for more detail on recovery
    /// behavior and potential limitations.
    ///
    /// # Generics & Lifetimes
    ///
    /// * `V` generic represents the user's value type, for example: `User`, `String`, etc.
    /// * `b` lifetime represents bytes potentially being borrowed from the `redb` database.
    #[inline]
    fn recover(
        protected_bytes: Bytes<'b>
    ) -> Result<Bytes<'b>, crate::layers::correctors::RecoverError> {
        Ok(Self::check_and_recover(protected_bytes)?)
    }
}