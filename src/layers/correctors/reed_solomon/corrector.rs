use crate::layers::correctors::{Correctable, Method, reed_solomon::ReedSolomon};
use crate::layers::descriptors::Direction;
use crate::layers::ValueBuf;

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl<'b, V: Correctable> crate::layers::correctors::Corrector<'b, V> for ReedSolomon<V> {
    /// Protects the provided data, typically by adding parity shards.
    ///
    /// If protection is unnecessary or cannot be applied, the original `ValueBuf` will be returned
    /// untouched.
    ///
    /// # Errors
    ///
    /// Consult the documentation of the corrector backend you are using for more detail on
    /// protection behavior and potential limitations: <http://docs.rs/reed-solomon-erasure>
    #[inline]
    fn protect(
        unprotected_bytes: ValueBuf<'b>
    ) -> Result<ValueBuf<'b>, crate::layers::correctors::ProtectError> {
        Ok(ReedSolomon::<V>::protect(unprotected_bytes)?)
    }

    /// Recovers a protected data buffer, validating and restoring its contents.
    ///
    /// This method may return the original buffer untouched if no corruption is detected, or may
    /// allocate a repaired version if any issues are found.
    ///
    /// # Errors
    ///
    /// This method may fail for several reasons, including:
    ///
    /// * Input bytes are corrupted or malformed.
    ///
    /// Consult the documentation of the corrector backend you are using for more detail on recovery
    /// behavior and potential limitations: <http://docs.rs/reed-solomon-erasure>
    #[inline]
    fn recover(
        protected_bytes: ValueBuf<'b>
    ) -> Result<ValueBuf<'b>, crate::layers::correctors::RecoverError> {
        Ok(ReedSolomon::<V>::recover(protected_bytes)?)
    }

    /// Returns the error correction method that the current `Corrector` trait implements.
    ///
    /// This enables runtime identification of the error correction algorithm in use, allowing
    /// applications to log compression details, or store metadata about how data was processed in
    /// the data pipeline.
    #[inline]
    fn method() -> &'static Method {
        &Method::ReedSolomon
    }
}