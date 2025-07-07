//! A typed wrapper around a read-only `redb` table for a specific key/value type pair.

use crate::Codec;
use crate::typed::TableRef;

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl<K, V> TableRef<K, V>
where
    K: Codec<K>,
    V: Codec<V>
{
    /// Retrieves the value associated with the given key, if it exists.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    ///
    /// * Encoding the key or decoding the value fails, or
    /// * If a storage error occurs.
    ///
    /// # Notes
    ///
    /// * This method call is passed-through to the `redb` Rust embedded database.
    #[cfg(feature = "redb-pass-through")]
    #[inline]
    pub fn get(&self, key: &K) -> Result<Option<V>, Error> {
        let key_bytes = K::serialize(key)?;
        if let Some(value) = self.redb_table.get(key_bytes.as_slice())? {
            Ok(Some(V::deserialize(value.value())?))
        } else {
            Ok(None)
        }
    }

    /// Returns storage usage statistics for the table.
    ///
    /// # Errors
    ///
    /// * Returns an error if the underlying storage fails to produce stats.
    ///
    /// # Notes
    ///
    /// * This method call is passed-through to the `redb` Rust embedded database.
    #[cfg(feature = "redb-pass-through")]
    #[inline]
    pub fn stats(&self) -> Result<redb::TableStats, Error> {
        Ok(self.redb_table.stats()?)
    }

    /// Returns the total number of entries in the table.
    ///
    /// # Errors
    ///
    /// * Returns an error if the operation fails due to storage issues.
    ///
    /// # Notes
    ///
    /// * This method call is passed-through to the `redb` Rust embedded database.
    #[cfg(feature = "redb-pass-through")]
    #[inline]
    pub fn len(&self) -> Result<u64, Error> {
        Ok(self.redb_table.len()?)
    }

    /// Returns `true` if the table contains no entries.
    ///
    /// # Errors
    ///
    /// * Returns an error if the operation fails due to storage issues.
    ///
    /// # Notes
    ///
    /// * This method call is passed-through to the `redb` Rust embedded database.
    #[cfg(feature = "redb-pass-through")]
    #[inline]
    pub fn is_empty(&self) -> Result<bool, Error> {
        Ok(self.redb_table.is_empty()?)
    }

    /// Returns the name of the underlying table.
    ///
    /// # Notes
    ///
    /// * This method call is passed-through to the `redb` Rust embedded database.
    #[cfg(feature = "redb-pass-through")]
    #[inline]
    #[must_use]
    pub fn name(&self) -> &str {
        self.redb_table.name()
    }
}

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl<K, V> From<redb::ReadOnlyTable<&[u8], &[u8]>> for TableRef<K, V>
where
    K: Codec<K>,
    V: Codec<V>
{
    /// Converts a raw `redb` table into a typed [`TableRef`] wrapper.
    ///
    /// This implementation allows ergonomic initialization of typed tables via `.into()` or similar
    /// conversions from [`RawReadOnlyTable`].
    ///
    /// The resulting table provides ergonomic access to decoded key-value operations via the
    /// [`Codec`] trait.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let typed_table: TableMut<MyKey, MyValue> = raw_table.into();
    /// ```
    fn from(table: redb::ReadOnlyTable<&[u8], &[u8]>) -> Self {
        Self { redb_table: table, _phantom: std::marker::PhantomData }
    }
}