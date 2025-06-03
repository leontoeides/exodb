use crate::indexing::IndexableKey;
use redb::ReadableTable;
use crate::indexing::HasPrimaryKey;
use crate::indexing::HasTable;
use crate::indexing::Indexable;
use crate::indexing::IndexEntry;
use crate::indexing::IndexKey;
use crate::{Codec, Error, typed::TableMut};
use redb::{TableDefinition, WriteTransaction};

// -------------------------------------------------------------------------------------------------
//
/// ## Index Safety Feature
///
/// If you're missing methods like `table` on this `Transaction`, it's likely due to the default
/// `index-safety` feature.
///
/// `index-safety` prevents direct table mutation (through `TableMut`) to ensure all inserts and
/// deletes are coordinated with secondary indexes.
///
/// ### Options:
/// * Safe default: use high-level methods like `insert` and `insert_indexed`
/// * Manual mode: disable the `index-safety` feature and enable `i-know-what-im-doing`
///
/// In `Cargo.toml`:
/// ```toml
/// [features]
/// default = ["index-safety"]
/// i-know-what-im-doing = []
/// ```
///
/// This ensures `exodb` is safe by default but fully controllable by experts.
pub struct Transaction(WriteTransaction);

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl Transaction {
    #[must_use] pub fn new(redb: WriteTransaction) -> Self {
        redb.into()
    }

    /// Opens a mutable typed table by name.
    #[cfg(not(feature = "index-safety"))]
    pub fn table<K, V>(&self, name: &str) -> Result<TableMut<K, V>, Error>
    where
        K: Codec<K>,
        V: Codec<V>,
    {
        let table_definition = TableDefinition::<&[u8], &[u8]>::new(name);
        let redb_table = self.0.open_table(table_definition)?;
        Ok(TableMut::new(redb_table))
    }

    /// Commits the transaction and makes all changes permanent.
    pub fn commit(self) -> Result<(), Error> {
        self.0.commit()?;
        Ok(())
    }

    pub fn insert<'v, K, V>(&mut self, value: &'v V) -> Result<Option<V>, Error>
    where
        V: HasTable + HasPrimaryKey<'v, K> + Codec<V>,
        K: Codec<K> + 'v,
    {
        let mut primary_table: redb::Table<&[u8], &[u8]> = self.0.open_table(
            TableDefinition::new(V::table_name())
        )?;

        let primary_key_bytes: Vec<u8> =
            value.primary_key().to_bytes()?;

        let value_bytes: Vec<u8> =
            V::encode(value)?;

        let result = match primary_table.insert(&*primary_key_bytes, &*value_bytes)? {
            Some(old) => Some(V::decode(old.value())?),
            None => None,
        };

        Ok(result)
    }







    fn insert_unique_index<K>(&self, entry: &IndexEntry<K>) -> Result<(), Error>
    where K: IndexableKey {
        let mut table = self.0.open_table::<&[u8], &[u8]>(
            TableDefinition::new(entry.index_name)
        )?;

        let secondary_key_bytes: Vec<u8> =
            entry.secondary_key.to_bytes()?;

        if let Some(existing) = table.get(&*secondary_key_bytes)? {
            if existing.value() == entry.primary_key_bytes {
                // Already exists and is correct — no-op
                Ok(())
            } else {
                // Collision — different primary key already mapped to this secondary key
                Err(Error::IndexCollision {
                    index: entry.index_name,
                    key: secondary_key_bytes.clone(),
                })
            }
        } else {
            // New entry
            table.insert(&*secondary_key_bytes, entry.primary_key_bytes)?;
            Ok(())
        }
    }


    fn insert_non_unique_index<K>(&self, entry: &IndexEntry<K>) -> Result<(), Error>
    where K: IndexableKey {
        let mut table = self.0.open_table::<&[u8], &[u8]>(
            TableDefinition::new(entry.index_name)
        )?;

        let secondary_key_bytes: Vec<u8> =
            entry.secondary_key.to_bytes()?;

        if let Some(existing) = table.get(&*secondary_key_bytes)? {
            if existing.value() == entry.primary_key_bytes {
                // Already exists and is correct — no-op
                Ok(())
            } else {
                // Collision — different primary key already mapped to this secondary key
                Err(Error::IndexCollision {
                    index: entry.index_name,
                    key: secondary_key_bytes.clone(),
                })
            }
        } else {
            // New entry
            table.insert(&*secondary_key_bytes, entry.primary_key_bytes)?;
            Ok(())
        }
    }







/*
    fn insert_unique_index<K>(&self, entry: &IndexEntry<K>) -> Result<(), Error> {
        let mut table = self.0.open_table::<&[u8], &[u8]>(
            TableDefinition::new(entry.index_name)
        )?;

        if let Some(existing) = table.get(&*entry.secondary_key_bytes)? {
            if existing.value() == entry.primary_key_bytes {
                // Already exists and is correct — no-op
                Ok(())
            } else {
                // Collision — different primary key already mapped to this secondary key
                Err(Error::IndexCollision {
                    index: entry.index_name,
                    key: entry.secondary_key_bytes.clone(),
                })
            }
        } else {
            // New entry
            table.insert(&*entry.secondary_key_bytes, entry.primary_key_bytes)?;
            Ok(())
        }
    }
*/





/*
    fn insert_indexed<'v, K, V>(
        &mut self,
        value: &'v V,
    ) -> Result<(), Error>
    where
        V: HasPrimaryKey<'v, K> + Indexable<'v>,
        K: Codec<K> + 'v,
    {
        let primary_key_bytes: Vec<u8> =
            value.primary_key().to_bytes()?;

        for index in value.indexes()? {
            let mut table = self.0.open_table(
                TableDefinition::<&[u8], &[u8]>::new(index.index_name())
            )?;

            let secondary_key_bytes: Vec<u8> =
                index.to_bytes()?;

            table.insert(&*secondary_key_bytes, &*primary_key_bytes)?; // TODO: enforce uniqueness
        }

        Ok(())
    }
*/




/*
    pub fn insert_indexed<'v, K, V>(
        &mut self,
        value: &'v V,
    ) -> Result<(), Error>
    where
        V: Indexable<'v> + HasPrimaryKey<'v, K>,
        K: Codec<K> + 'v,
    {
        let primary_key = value.primary_key().to_bytes()?;

        for index in value.indexes()? {
            let table = self.0.open_table(
                TableDefinition::<&[u8], &[u8]>::new(index.index_name())
            )?;

            let secondary_key = index.to_bytes()?;
            table.insert(&secondary_key, &primary_key)?; // TODO: enforce uniqueness
        }

        Ok(())
    } */
}

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl From<WriteTransaction> for Transaction {
    fn from(redb: WriteTransaction) -> Self {
        Self(redb)
    }
}