//! `UpgradableKeySet` implementation for `vec::Keyset`

use crate::indexing::key_set::vec::KeySet;

// -------------------------------------------------------------------------------------------------
//
// Upgradable Key-Set Implementation

impl crate::indexing::key_set::UpgradableKeySet for KeySet {
    /// Upgrades a `ReadableKeySet` view into an owned & mutable [`KeySet`] by completing the
    /// deserialization process, if necessary.
    ///
    /// In this case, the caller already posseses an owned and mutable `KeySet`. So we return an
    /// `Ok(self)`. This should compile to nothing, a no-op.
    ///
    /// This method is typically used when write access is required to complete a set operation.
    ///
    /// # Errors
    ///
    /// * This method will return an error if deserialization of the underlying [`ArchivedKeySet`]
    ///   fails.
    #[inline]
    fn upgrade(self) -> Result<KeySet, crate::Error> {
        Ok(self)
    }
}