//! Trait that provides the ability to upgrade read-only key-set references into owned & mutable
//! key-sets.

/// Trait for upgrading a read-only key set into an owned, mutable [`KeySet`].
///
/// This is commonly used when write access is required—such as performing set operations that
/// modify the collection or need ownership of the keys.
///
/// # Errors
///
/// Returns an error if deserialization of the underlying [`ArchivedKeySet`] fails.
pub trait UpgradableKeySet {
    /// Upgrades a `ReadableKeySet` view into an owned & mutable [`KeySet`] by completing the
    /// deserialization process, if necessary.
    ///
    /// This method is typically used when write access is required to complete a set operation.
    ///
    /// # Errors
    ///
    /// * This method will return an error if deserialization of the underlying [`ArchivedKeySet`]
    ///   fails.
    fn upgrade(self) -> Result<crate::indexing::KeySet, crate::Error>;
}