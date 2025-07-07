//! Enables additional operations on `TableMut` when the key type `K` implements
//! `OrderedWhenSerialized`.

use crate::layers::serializers::OrderedWhenSerialized;
use crate::typed::{ResultEntry, table_mut::{ExtractIf, Range, TableMut}};
use crate::{Codec, Error};
use redb::ReadableTable;

// -------------------------------------------------------------------------------------------------
//
/// Enables ordered operations on [`TableMut`] when the key type implements [`OrderedWhenSerialized`].
///
/// `OrderedWhenSerialized` guarantees that the binary encoding of a key preserves its natural
/// ordering, making it safe to use range queries, ordered iteration, and prefix-based lookups.
///
/// Without this marker trait, the encoded key order is undefined and may not correspond to the
/// key's logical order. In such cases, range-based and sequential methods are intentionally
/// unavailable to prevent misuse or misleading behavior.
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
pub trait OrderedTable<'txn, K, V, KR>
where
    K: OrderedWhenSerialized + Codec<K> + for<'a> std::borrow::Borrow<&'a [u8]>,
    V: Codec<V>,
    KR: for<'a> std::borrow::Borrow<&'a [u8]>
{
    /// Applies a predicate to each key-value pair within the specified range and returns an
    /// iterator over entries that match the predicate. Matching entries are removed **only if the
    /// iterator yields them**.
    ///
    /// Entries that are skipped by the iterator will remain in the table.
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
    /// * See [# Errors](#errors) for possible failure conditions.
    #[allow(clippy::type_complexity, reason="required for lifetime bounds")]
    fn extract_from_if<F>(
        &mut self,
        range: impl std::ops::RangeBounds<KR>,
        predicate: F
    ) -> Result<ExtractIf<'_, K, V, F>, Error>
    where
        F: for<'f> FnMut(&K, &V) -> bool + 'txn;

    /// Retains only the entries for which the predicate returns `true` within the specified range.
    /// All entries for which the predicate returns `false` are removed from the table.
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
    /// * See [# Errors](#errors) for possible failure conditions.
    #[allow(clippy::type_complexity, reason="required for lifetime bounds")]
    fn retain_in<F>(
        &mut self,
        range: impl std::ops::RangeBounds<KR>,
        predicate: F
    ) -> Result<(), Error>
    where
        F: for<'f> Fn(&K, &V) -> bool + 'txn;

    /// Removes and returns the first key-value pair in the table, if present.
    ///
    /// # Errors
    ///
    /// * See [# Errors](#errors) for possible failure conditions.
    fn pop_first(&mut self) -> Result<Option<(K, V)>, Error>;

    /// Removes and returns the last key-value pair in the table, if present.
    ///
    /// # Errors
    ///
    /// * See [# Errors](#errors) for possible failure conditions.
    fn pop_last(&mut self) -> Result<Option<(K, V)>, Error>;

    /// Returns a double-ended iterator over key-value pairs in the specified range, ordered by key.
    /// [Read more](https://docs.rs/redb/latest/redb/trait.ReadableTable.html#tymethod.range)
    ///
    /// # Errors
    ///
    /// * See [# Errors](#errors) for possible failure conditions.
    fn range(
        &self,
        bounds: impl std::ops::RangeBounds<KR>
    ) -> Result<impl Iterator<Item = ResultEntry<K, V>>, Error>;

    /// Returns the first key-value pair in the table without removing it.
    ///
    /// # Errors
    ///
    /// * See [# Errors](#errors) for possible failure conditions.
    fn first(&self) -> Result<Option<(K, V)>, Error>;

    /// Returns the last key-value pair in the table without removing it.
    ///
    /// # Errors
    ///
    /// * See [# Errors](#errors) for possible failure conditions.
    fn last(&self) -> Result<Option<(K, V)>, Error>;

    /// Returns a double-ended iterator over all key-value pairs in the table, ordered by key.
    ///
    /// # Errors
    ///
    /// * See [# Errors](#errors) for possible failure conditions.
    #[allow(
        clippy::iter_not_returning_iterator,
        reason="`Range` does implement `Iterator`, clippy may be confused by lifetime elision"
    )]
    fn iter(&self) -> Result<Range<'_, K, V>, Error>;
}

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl<'txn, K, V, KR> OrderedTable<'txn, K, V, KR> for TableMut<'txn, K, V>
where
    K: Codec<K> + for<'a> std::borrow::Borrow<&'a [u8]> + OrderedWhenSerialized,
    V: Codec<V>,
    KR: for<'a> std::borrow::Borrow<&'a [u8]>
{
    /// Applies a predicate to each key-value pair within the specified range and returns an
    /// iterator over entries that match the predicate. Matching entries are removed **only if the
    /// iterator yields them**.
    ///
    /// Entries that are skipped by the iterator will remain in the table.
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
    /// * See [# Errors](#errors) for possible failure conditions.
    #[allow(clippy::type_complexity, reason="required for lifetime bounds")]
    fn extract_from_if<F>(
        &mut self,
        range: impl std::ops::RangeBounds<KR>,
        mut predicate: F
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

        Ok(self.redb_table.extract_from_if(range, closure)?.into())
    }

    /// Retains only the entries for which the predicate returns `true` within the specified range.
    /// All entries for which the predicate returns `false` are removed from the table.
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
    /// * See [# Errors](#errors) for possible failure conditions.
    #[allow(clippy::type_complexity, reason="required for lifetime bounds")]
    fn retain_in<F>(
        &mut self,
        range: impl std::ops::RangeBounds<KR>,
        predicate: F
    ) -> Result<(), Error>
    where
        F: for<'f> Fn(&K, &V) -> bool + 'txn
    {
        let closure: Box<dyn for<'a, 'b> Fn(&'a [u8], &'b [u8]) -> bool> = Box::new(
            move |k: &[u8], v: &[u8]| -> bool {
                match (K::deserialize(k), V::deserialize(v)) {
                    (Ok(k_dec), Ok(v_dec)) => predicate(&k_dec, &v_dec),
                    _ => false,
                }
            }
        );

        Ok(self.redb_table.retain_in(range, closure)?)
    }

    /// Removes and returns the first key-value pair in the table, if present.
    ///
    /// # Errors
    ///
    /// * See [# Errors](#errors) for possible failure conditions.
    fn pop_first(&mut self) -> Result<Option<(K, V)>, Error> {
        self.redb_table
            .pop_first()?
            .map(|(k_guard, v_guard)| Ok::<_, Error>((
                K::deserialize(k_guard.value())?,
                V::deserialize(v_guard.value())?,
            )))
            .transpose()
    }

    /// Removes and returns the last key-value pair in the table, if present.
    ///
    /// # Errors
    ///
    /// * See [# Errors](#errors) for possible failure conditions.
    fn pop_last(&mut self) -> Result<Option<(K, V)>, Error> {
        self.redb_table
            .pop_last()?
            .map(|(k_guard, v_guard)| Ok::<_, Error>((
                K::deserialize(k_guard.value())?,
                V::deserialize(v_guard.value())?,
            )))
            .transpose()
    }

    /// Returns a double-ended iterator over key-value pairs in the specified range, ordered by key.
    /// [Read more](https://docs.rs/redb/latest/redb/trait.ReadableTable.html#tymethod.range)
    ///
    /// # Errors
    ///
    /// * See [# Errors](#errors) for possible failure conditions.
    fn range(
        &self,
        range: impl std::ops::RangeBounds<KR>
    ) -> Result<impl Iterator<Item = Result<(K, V), Error>>, Error> {
        let raw_iter = self.redb_table.range(range)?;

        Ok(raw_iter
            .map(|entry| entry
                .map_err(Into::into)
                .and_then(|(k, v)| Ok::<_, Error>((
                    K::deserialize(k.value())?,
                    V::deserialize(v.value())?,
                )))
            ))
    }

    /// Returns the first key-value pair in the table without removing it.
    ///
    /// # Errors
    ///
    /// * See [# Errors](#errors) for possible failure conditions.
    fn first(&self) -> Result<Option<(K, V)>, Error> {
        self.redb_table
            .first()?
            .map(|(k_guard, v_guard)| Ok::<_, Error>((
                K::deserialize(k_guard.value())?,
                V::deserialize(v_guard.value())?,
            )))
            .transpose()
    }

    /// Returns the last key-value pair in the table without removing it.
    ///
    /// # Errors
    ///
    /// * See [# Errors](#errors) for possible failure conditions.
    fn last(&self) -> Result<Option<(K, V)>, Error> {
        self.redb_table
            .last()?
            .map(|(k_guard, v_guard)| Ok::<_, Error>((
                K::deserialize(k_guard.value())?,
                V::deserialize(v_guard.value())?,
            )))
            .transpose()
    }

    /// Returns a double-ended iterator over all key-value pairs in the table, ordered by key.
    ///
    /// # Errors
    ///
    /// * See [# Errors](#errors) for possible failure conditions.
    fn iter(
        &self
    ) -> Result<Range<'_, K, V>, Error> {
        Ok(self.redb_table.iter()?.into())
    }
}