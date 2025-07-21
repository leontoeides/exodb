use crate::layers::core::{bytes::Error, Bytes};
use crate::layers::correctors::ActiveCorrector;
use crate::layers::{Correctable, Corrector};

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl Bytes<'_> {
    /// Adds parity data to the original bytes, creating redundant information that enables
    /// automatic detection and correction of future corruption.
    ///
    /// If protection is unnecessary or cannot be applied, the original `Bytes` will be returned
    /// untouched.
    ///
    /// # Errors
    ///
    /// Consult the documentation of the corrector backend you are using for more detail on
    /// protection behavior and potential limitations.
    ///
    /// # Generics & Lifetimes
    ///
    /// * `V` generic represents the user's value type, for example: `User`, `String`, etc.
    /// * `b` lifetime represents bytes potentially being borrowed from the `redb` database.
    #[inline]
    pub fn protect<V: Correctable>(
        self
    ) -> Result<Self, Error> {
        if V::DIRECTION.is_write() {
            Ok(ActiveCorrector::<V>::protect(self)?)
        } else {
            Ok(self)
        }
    }

    /// Analyzes protected data for corruption and automatically repairs any detected errors using
    /// the embedded parity information, or returns the original data if no corruption is found.
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
    /// behavior and potential limitations.
    ///
    /// # Generics & Lifetimes
    ///
    /// * `V` generic represents the user's value type, for example: `User`, `String`, etc.
    /// * `b` lifetime represents bytes potentially being borrowed from the `redb` database.
    #[inline]
    pub fn recover<V: Correctable>(
        self
    ) -> Result<Self, Error> {
        if V::DIRECTION.is_read() {
            Ok(ActiveCorrector::<V>::recover(self)?)
        } else {
            Ok(self)
        }
    }
}