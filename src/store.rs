#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Codec(#[from] crate::codecs::Error),

    #[error(transparent)]
    RedbCommit(#[from] redb::CommitError),

    #[error(transparent)]
    RedbDatabase(#[from] redb::DatabaseError),

    #[error(transparent)]
    RedbTable(#[from] redb::TableError),

    #[error(transparent)]
    RedbTransaction(#[from] Box<redb::TransactionError>),

    #[error(transparent)]
    Redbstore(#[from] redb::StorageError),
}

use redb::ReadableTable;
use redb::Table;
use crate::codecs::Codec;
use std::marker::PhantomData;

/// A typed wrapper around a redb table for a specific key/value type pair.
///
/// The key and value types must implement the `Codec` trait to handle serialization and deserialization.
/// This allows for pluggable, safe, and ergonomic interaction with redb using your own data types.
type RawTable<'txn> = Table<'txn, &'static [u8], &'static [u8]>;

pub struct ExoTable<'txn, K, V>
where
    K: Codec<K>,
    V: Codec<V>,
{
    table: RawTable<'txn>,
    _phantom: PhantomData<(K, V)>,
}


impl<'txn, K, V> ExoTable<'txn, K, V>
where
    K: Codec<K>,
    V: Codec<V>,
{
    pub fn new(table: Table<'txn, &[u8], &[u8]>) -> Self {
        Self {
            table,
            _phantom: PhantomData,
        }
    }

    pub fn insert(&mut self, key: &K, value: &V) -> Result<(), crate::store::Error> {
        let key_bytes = K::encode(key)?;
        let value_bytes = V::encode(value)?;
        {
            let _ = self.table.insert(key_bytes.as_slice(), value_bytes.as_slice())?;
        }
        Ok(())
    }

    pub fn get(&self, key: &K) -> Result<Option<V>, crate::store::Error> {
        let key_bytes = K::encode(key)?;
        if let Some(val) = self.table.get(key_bytes.as_slice())? {
            Ok(Some(V::decode(val.value())?))
        } else {
            Ok(None)
        }
    }

    pub fn delete(&mut self, key: &K) -> Result<(), crate::store::Error> {
        let key_bytes = K::encode(key)?;
        self.table.remove(key_bytes.as_slice())?;
        Ok(())
    }

    pub fn scan(
        &'txn self
    ) -> Result<impl Iterator<Item = Result<(K, V), crate::store::Error>> + 'txn, crate::store::Error> {
        let iter = self.table.iter()?;
        Ok(iter.map(|entry| {
            let (k, v) = entry?;
            let key = K::decode(k.value())?;
            let val = V::decode(v.value())?;
            Ok((key, val))
        }))
    }
}
