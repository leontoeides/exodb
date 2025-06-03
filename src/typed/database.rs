/*

| Feature                   | Description                                                            |
| ------------------------- | ---------------------------------------------------------------------- |
| `open_table`              | Open a named table and bind it to `TableRef<K, V>` or `TableMut<K, V>` |
| `create_table_if_missing` | Optional helper to initialize new tables if needed                     |
| `bulk_insert_indexed<T>`  | Auto-indexing on insert if `T: Indexable + HasPrimaryKey`              |
| `resolve_index`           | Look up values by secondary key                                        |
| `txn()`                   | Begin read or write transaction                                        |
| `flush` / `checkpoint`    | Optional (could be exposed or deferred to redb's interface)            |

| Power                         | Description                                                   |
| ----------------------------- | ------------------------------------------------------------- |
| `Database::register<T>()`     | Associates table name, index structure, primary key type      |
| `Database::insert_indexed<T>` | Automatically indexes a value using all `Key` entries         |
| `Database::flush()`           | Exposes flush/sync logic for embedded use                     |
| `Database::reindex<T>()`      | Rebuild indexes from known records (for non-deferred indexes) |



*/


use crate::Error;
use crate::typed::transaction::ReadTransaction;
use crate::typed::transaction::WriteTransaction;

/// The entry point for working with a redb database using typed keys and values.
///
/// This type wraps a `redb::Database` and provides ergonomic access to typed tables,
/// leveraging the `Codec` trait for automatic encoding and decoding.
///
/// For ordered operations, use tables with key types that also implement [`OrderedWhenEncoded`].
pub struct Database(redb::Database);

impl Database {
    /// Opens or creates a database at the given file path.
    pub fn open(path: impl AsRef<std::path::Path>) -> Result<Self, Error> {
        let redb = redb::Database::open(path)?;
        Ok(Self(redb))
    }

    /// Begins a read-only transaction.
    pub fn read(&self) -> Result<ReadTransaction, Error> {
        Ok(ReadTransaction::new(self.0.begin_read()?))
    }

    /// Begins a writable transaction.
    pub fn write(&self) -> Result<WriteTransaction, Error> {
        Ok(WriteTransaction::new(self.0.begin_write()?))
    }
}
