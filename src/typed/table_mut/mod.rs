//! A typed wrapper around a mutable `redb` table for a specific key/value type pair.

mod extract_if;
mod ordered_table;
mod range;

pub use crate::typed::table_mut::ordered_table::OrderedTable;

use crate::indexing::HasPrimaryKey;
use crate::typed::table_mut::{extract_if::ExtractIf, range::Range};
use crate::{Codec, Error};
use redb::{ReadableTable, ReadableTableMetadata, TableHandle};
use std::marker::PhantomData;

// -------------------------------------------------------------------------------------------------
//
// Type Aliases

/// A type alias for a low-level mutable `redb` table with raw byte slice keys and values.
///
/// This is the untyped foundation beneath [`TableMut`], where encoding and decoding are handled
/// externally.
pub type RawTable<'txn> = redb::Table<'txn, &'static [u8], &'static [u8]>;

// -------------------------------------------------------------------------------------------------
//
/// A typed wrapper around a mutable `redb` table for a specific key/value type pair.
///
/// The key and value types must implement the `Codec` trait to handle serialization and
/// deserialization. This allows for pluggable, safe, and ergonomic interaction with `redb` using
/// your own data types.
///
/// # Errors
///
/// All methods may return a [`crate::Error`] variant originating from underlying storage issues.
///
/// | Variant         | Cause                                             | Resolution                            |
/// |-----------------|---------------------------------------------------|---------------------------------------|
/// | `Corrupted`     | Internal table structure was damaged or invalid   | Backup and restore or recreate table  |
/// | `ValueTooLarge` | Attempted to store a value that exceeded max size | Consider breaking into smaller values |
/// | `Io`            | I/O failure (disk error, permission issue, etc.)  | Check file permissions and storage    |
/// | `PreviousIo`    | Prior I/O failure poisoned the database           | Reopen or recover the environment     |
/// | `LockPoisoned`  | A panic occurred while holding a database lock    | Restart process or retry operation    |
#[derive(Debug)]
pub struct TableMut<'txn, K, V>
where
    K: Codec<K>,
    V: Codec<V>
{
    redb_table: RawTable<'txn>,
    _phantom: PhantomData<(K, V)>,
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl<'txn, K, V> TableMut<'txn, K, V>
where
    K: Codec<K>,
    V: Codec<V>
{
    /// Creates a new [`TableMut`] wrapper around a raw `redb` table with byte slice keys and
    /// values.
    ///
    /// This method initializes the typed interface for inserting, retrieving, and modifying
    /// structured data using [`Codec`] implementations for the key and value types.
    ///
    /// Internally, this wraps the raw `redb` table into a type-safe exoskeleton.
    ///
    /// # Notes
    ///
    /// * This method call is passed-through to the `redb` Rust embedded database.
    #[cfg(feature = "redb-pass-through")]
    #[inline]
    #[must_use]
    pub fn new(table: RawTable<'txn>) -> Self {
        table.into()
    }

    /// Applies a predicate to each key-value pair in the table and returns an iterator over entries
    /// where the predicate evaluates to `true`.
    ///
    /// Entries are only removed if they are yielded by the iterator. If the iterator is dropped
    /// early, any remaining entries are preserved.
    ///
    /// ## Predicate Behavior
    ///
    /// | Want to Remove Record? | Then Return |
    /// |------------------------|-------------|
    /// | Yes                    | `true`      |
    /// | No                     | `false`     |
    ///
    /// # Errors
    ///
    /// Returns an error if:
    ///
    /// * Decoding any key or value fails, or
    /// * If a storage-level error occurs.
    ///
    /// # Notes
    ///
    /// * This method call is passed-through to the `redb` Rust embedded database.
    #[cfg(feature = "redb-pass-through")]
    #[inline]
    #[allow(clippy::type_complexity, reason="required for lifetime bounds")]
    pub fn extract_if<F>(
        &mut self,
        mut predicate: F,
    ) -> Result<ExtractIf<'_, K, V, F>, Error>
    where
        F: for<'f> FnMut(&K, &V) -> bool + 'txn,
    {
        let closure: Box<dyn for<'a, 'b> FnMut(&'a [u8], &'b [u8]) -> bool> = Box::new(
            move |k: &[u8], v: &[u8]| -> bool {
                match (K::deserialize(k), V::deserialize(v)) {
                    (Ok(k_dec), Ok(v_dec)) => predicate(&k_dec, &v_dec),
                    _ => false,
                }
            }
        );

        Ok(self.redb_table.extract_if(closure)?.into())
    }

    /// Applies a predicate to each key-value pair in the table and retains only those for which the
    /// predicate returns `true`.
    ///
    /// Entries for which the predicate returns `false` are removed.
    ///
    /// ## Predicate Behavior
    ///
    /// | Want to Keep Record? | Then Return |
    /// |----------------------|-------------|
    /// | Yes                  | `true`      |
    /// | No                   | `false`     |
    ///
    /// # Errors
    ///
    /// Returns an error if:
    ///
    /// * Decoding any key or value fails, or
    /// * If a storage-level error occurs.
    ///
    /// # Notes
    ///
    /// * This method call is passed-through to the `redb` Rust embedded database.
    #[cfg(feature = "redb-pass-through")]
    #[inline]
    #[allow(clippy::type_complexity, reason="required for lifetime bounds")]
    pub fn retain<F>(
        &mut self,
        mut predicate: F,
    ) -> Result<(), Error>
    where
        F: for<'f> FnMut(&K, &V) -> bool + 'txn,
    {
        let closure: Box<dyn for<'a, 'b> FnMut(&'a [u8], &'b [u8]) -> bool> = Box::new(
            move |k: &[u8], v: &[u8]| -> bool {
                match (K::deserialize(k), V::deserialize(v)) {
                    (Ok(k_dec), Ok(v_dec)) => predicate(&k_dec, &v_dec),
                    _ => false,
                }
            }
        );

        Ok(self.redb_table.retain(closure)?)
    }

    /// Inserts a new key-value pair into the table, replacing any existing entry with the same key.
    ///
    /// This method requires the caller to explicitly provide both the key and the value.
    ///
    /// Returns the previous value if the key already existed, or `None` if it was newly inserted.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    ///
    /// * Encoding the key or value fails,
    /// * Decoding the previous value fails (if any), or
    /// * Insertion fails due to storage-related issues.
    pub fn insert(&mut self, key: &K, value: &V) -> Result<Option<V>, Error> {
        let key_bytes = K::serialize(key)?;
        let value_bytes = V::serialize(value)?;
        if let Some(value) = self.redb_table.insert(
            key_bytes.as_slice(),
            value_bytes.as_slice()
        )? {
            Ok(Some(V::deserialize(value.value())?))
        } else {
            Ok(None)
        }
    }

    /// Inserts a new value into the table, using the value's own primary key.
    ///
    /// This method requires only the value. The key is automatically extracted using the
    /// [`HasPrimaryKey`] trait, which must be implemented for the value type.
    ///
    /// Returns the previous value if the key already existed, or `None` if it was newly inserted.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    ///
    /// * Encoding the key or value fails,
    /// * Decoding the previous value fails (if any), or
    /// * Insertion fails due to storage-related issues.
    pub fn insert_keyed<'v>(&mut self, value: &'v V) -> Result<Option<V>, Error>
    where
        K: 'v,
        V: HasPrimaryKey<'v, K>
    {
        let primary_key = value.primary_key();
        self.insert(primary_key.as_ref(), value)
    }

    /// Inserts multiple key-value pairs into the table, replacing any existing entries with the
    /// same keys.
    ///
    /// This method requires the caller to explicitly provide both the key and the value.
    ///
    /// Each key is encoded and inserted along with its corresponding value. If a key already exists
    /// in the table, the value is replaced, and the previous value is discarded.
    ///
    /// This method is more efficient than inserting values one at a time, as it avoids repeated
    /// internal checks and amortizes encoding costs.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    ///
    /// * Encoding any key or value fails, or
    /// * Insertion fails due to storage-related issues.
    ///
    /// # Notes
    ///
    /// * Duplicate keys within the iterator will be inserted in sequence — the final value wins.
    /// * Does **not** return previous values. Use individual inserts if you need them.
    pub fn bulk_insert(
        &mut self,
        entries: impl IntoIterator<Item = (K, V)>
    ) -> Result<(), Error> {
        for (key, value) in entries {
            let key_bytes = K::serialize(&key)?;
            let value_bytes = V::serialize(&value)?;
            // We discard previous value for performance; user can call `insert` manually if needed
            let _ = self.redb_table.insert(key_bytes.as_slice(), value_bytes.as_slice())?;
        }
        Ok(())
    }

    /// Inserts multiple values into the table, replacing any existing entries with the same keys.
    ///
    /// This method requires only the value. The key is automatically extracted using the
    /// [`HasPrimaryKey`] trait, which must be implemented for the value type.
    ///
    /// Each key is encoded and inserted along with its corresponding value. If a key already exists
    /// in the table, the value is replaced, and the previous value is discarded.
    ///
    /// This method is more efficient than inserting values one at a time, as it avoids repeated
    /// internal checks and amortizes encoding costs.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    ///
    /// * Encoding any key or value fails,
    /// * Insertion fails due to storage-related issues.
    ///
    /// # Notes
    ///
    /// * Duplicate keys within the iterator will be inserted in sequence — the final value wins.
    /// * Does **not** return previous values. Use individual inserts if you need them.
    pub fn bulk_insert_keyed<'v>(
        &mut self,
        entries: impl IntoIterator<Item = &'v V>
    ) -> Result<(), Error>
    where
        K: 'v,
        V: HasPrimaryKey<'v, K> + 'v
    {
        for value in entries {
            let primary_key = value.primary_key();
            let key_bytes = primary_key.to_bytes()?;
            let value_bytes = V::serialize(value)?;
            // We discard previous value for performance; user can call `insert` manually if needed
            let _ = self.redb_table.insert(key_bytes.as_slice(), value_bytes.as_slice())?;
        }
        Ok(())
    }

    /// Removes a key-value pair from the table.
    ///
    /// This method requires the caller to provide key.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    ///
    /// * Encoding the key fails,
    /// * Decoding the removed value fails (if any), or
    /// * Removal fails due to storage-related issues.
    pub fn remove(&mut self, key: &K) -> Result<Option<V>, Error> {
        let key_bytes = K::serialize(key)?;
        if let Some(value) = self.redb_table.remove(key_bytes.as_slice())? {
            Ok(Some(V::deserialize(value.value())?))
        } else {
            Ok(None)
        }
    }

    /// Removes a key-value pair from the table.
    ///
    /// This method requires the caller to provide key.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    ///
    /// * Encoding the key fails,
    /// * Decoding the removed value fails (if any), or
    /// * Removal fails due to storage-related issues.
    pub fn remove_keyed<'v>(&mut self, value: &'v V) -> Result<Option<V>, Error>
    where
        K: 'v,
        V: HasPrimaryKey<'v, K>
    {
        let primary_key = value.primary_key();
        self.remove(primary_key.as_ref())
    }

    /// Retrieves the value associated with the given key, if it exists.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    ///
    /// * Encoding the key fails,
    /// * Decoding the value fails (if any), or
    /// * A storage error occurs.
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

impl<'txn, K, V> From<RawTable<'txn>> for TableMut<'txn, K, V>
where
    K: Codec<K>,
    V: Codec<V>
{
    /// Converts a raw `redb` table into a typed [`TableMut`] wrapper.
    ///
    /// This implementation allows ergonomic initialization of typed tables via `.into()` or similar
    /// conversions from [`RawTable`].
    ///
    /// The resulting table provides ergonomic access to decoded key-value operations via the
    /// [`Codec`] trait.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let typed_table: TableMut<MyKey, MyValue> = raw_table.into();
    /// ```
    fn from(table: RawTable<'txn>) -> Self {
        Self { redb_table: table, _phantom: PhantomData }
    }
}