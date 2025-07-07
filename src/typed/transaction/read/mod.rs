//! Read transaction methods that are routed directly to `redb`.

mod queries;
mod non_unique;

use crate::Codec;
use crate::typed::TableRef;
use crate::typed::transaction::Error;

// -------------------------------------------------------------------------------------------------

pub type RedbReadOnlyTable = redb::ReadOnlyTable<&'static [u8], &'static [u8]>;

// -------------------------------------------------------------------------------------------------
//
/// A wrapper around a `redb` read transaction.
///
/// Read-only transactions may exist concurrently with writes
#[derive(Debug)]
pub struct Transaction(redb::ReadTransaction);

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl Transaction {
    /// Wraps a `redb` read transaction into an `atlatl` one.
    #[inline]
    #[must_use]
    pub fn new(redb: redb::ReadTransaction) -> Self {
        redb.into()
    }

    /// Open the given table
    ///
    /// # Notes
    ///
    /// * This method call is passed-through to the `redb` Rust embedded database.
    #[inline]
    pub fn open_table<K, V>(&self, name: &str) -> Result<TableRef<K, V>, Error>
    where
        K: Codec<K>,
        V: Codec<V>,
    {
        let table_definition = redb::TableDefinition::<&[u8], &[u8]>::new(name);
        Ok(TableRef::new(self.0.open_table(table_definition)?))
    }

    /// Open the given table
    ///
    /// # Notes
    ///
    /// * This method call is passed directly to the `redb` key-value store.
    #[cfg(feature = "redb-pass-through")]
    #[inline]
    pub fn open_redb_table<K, V>(
        &self,
        definition: redb::TableDefinition<'_, K, V>,
    ) -> Result<redb::ReadOnlyTable<K, V>, Error>
    where
        K: redb::Key + 'static,
        V: redb::Value + 'static
    {
        Ok(self.0.open_table(definition)?)
    }

    /// Open the given table without a type
    ///
    /// # Notes
    ///
    /// * This method call is passed directly to the `redb` key-value store.
    #[cfg(feature = "redb-pass-through")]
    #[inline]
    pub fn open_untyped_table<K, V>(
        &self,
        handle: impl redb::TableHandle,
    ) -> Result<redb::ReadOnlyUntypedTable, Error> {
        Ok(self.0.open_untyped_table(handle)?)
    }

    /// Open the given table
    ///
    /// # Notes
    ///
    /// * This method call is passed directly to the `redb` key-value store.
    #[cfg(feature = "redb-pass-through")]
    #[inline]
    pub fn open_multimap_table<K, V>(
        &self,
        definition: redb::MultimapTableDefinition<'_, K, V>,
    ) -> Result<redb::ReadOnlyMultimapTable<K, V>, Error>
    where
        K: redb::Key + 'static,
        V: redb::Key + 'static
    {
        Ok(self.0.open_multimap_table(definition)?)
    }

    /// Open the given table
    ///
    /// # Notes
    ///
    /// * This method call is passed directly to the `redb` key-value store.
    #[cfg(feature = "redb-pass-through")]
    #[inline]
    pub fn open_untyped_multimap_table<K, V>(
        &self,
        handle: impl redb::MultimapTableHandle,
    ) -> Result<redb::ReadOnlyUntypedMultimapTable, Error> {
        Ok(self.0.open_untyped_multimap_table(handle)?)
    }

    /// List all the tables
    ///
    /// # Notes
    ///
    /// * This method call is passed directly to the `redb` key-value store.
    #[cfg(feature = "redb-pass-through")]
    #[inline]
    pub fn list_tables(
        &self
    ) -> Result<impl Iterator<Item = redb::UntypedTableHandle>, Error> {
        Ok(self.0.list_tables()?)
    }

    /// List all the multimap tables
    ///
    /// # Notes
    ///
    /// * This method call is passed directly to the `redb` key-value store.
    #[cfg(feature = "redb-pass-through")]
    #[inline]
    pub fn list_multimap_tables(
        &self
    ) -> Result<impl Iterator<Item = redb::UntypedMultimapTableHandle>, Error> {
        Ok(self.0.list_multimap_tables()?)
    }

    /// Close the transaction
    ///
    /// Transactions are automatically closed when they and all objects referencing them have been
    /// dropped, so this method does not normally need to be called. This method can be used to
    /// ensure that there are no outstanding objects remaining.
    ///
    /// Returns `ReadTransactionStillInUse` error if a table or other object retrieved from the
    /// transaction still references this transaction
    ///
    /// # Notes
    ///
    /// * This method call is passed directly to the `redb` key-value store.
    #[inline]
    pub fn close(
        self
    ) -> Result<(), Error> {
        Ok(self.0.close().map_err(Box::new)?)
    }
}

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl From<redb::ReadTransaction> for Transaction {
    /// Converts a `redb` read transaction into an `atlatl` read transaction.
    fn from(redb: redb::ReadTransaction) -> Self {
        Self(redb)
    }
}