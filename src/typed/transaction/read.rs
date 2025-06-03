use crate::indexing::ArchivedKeySet;
use crate::indexing::KeySet;
use crate::indexing::IndexableKey;
use crate::{Codec, Error, indexing::{HasTable, HasPrimaryKey, IndexKey}, typed::TableRef};
use redb::{ReadTransaction, TableDefinition};

// -------------------------------------------------------------------------------------------------

pub struct Transaction(ReadTransaction);

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl Transaction {
    #[must_use] pub fn new(redb: ReadTransaction) -> Self {
        redb.into()
    }

    /// Opens a read-only typed table by name.
    pub fn table<K, V>(&self, name: &str) -> Result<TableRef<K, V>, Error>
    where
        K: Codec<K>,
        V: Codec<V>,
    {
        let table_definition = TableDefinition::<&[u8], &[u8]>::new(name);
        let redb_table = self.0.open_table(table_definition)?;
        Ok(TableRef::new(redb_table))
    }

    /// Retrieves a value by the specified primary key, if it exists.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    ///
    /// * Encoding the primary key fails,
    /// * Decoding the value fails,
    /// * Opening a table fails, or
    /// * If a storage error occurs.
    pub fn get<'pk, PK, V>(
        &self,
        primary_key: &PK,
    ) -> Result<Option<V>, Error>
    where
        PK: Codec<PK>,
        V: HasTable + HasPrimaryKey<'pk, PK> + Codec<V>
    {
        let primary_table: redb::ReadOnlyTable<&[u8], &[u8]> = self.0.open_table(
            TableDefinition::new(V::table_name())
        )?;

        if let Some(value) = primary_table.get(&*PK::encode(primary_key)?)? {
            Ok(Some(V::decode(value.value())?))
        } else {
            Ok(None)
        }
    }

    /// Retrieves a value by the specified index key, if it exists.
    ///
    /// This `get` implementation is specifically for performing look-ups using unique indicies.
    ///
    /// See also: `get` and `get_non_unique`.
    ///
    /// The lookup proceeds in two steps:
    /// 1. Retrieves the primary key by decoding the result from the index table.
    /// 2. Uses that primary key to load the record from the primary table.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    ///
    /// * Encoding the key fails,
    /// * Decoding the value fails,
    /// * Opening a table fails, or
    /// * If a storage error occurs.
    fn get_unique<I>(&self, index_key: &I) -> Result<Option<I::Field>, Error>
    where
        I: IndexableKey,
    {
        let index_table: redb::ReadOnlyTable<&[u8], &[u8]> = self.0.open_table(
            TableDefinition::new(index_key.index_name())
        )?;

        match index_table.get(&*index_key.to_bytes()?)? {
            Some(primary_key_bytes) => {
                let primary_table: redb::ReadOnlyTable<&[u8], &[u8]> = self.0.open_table(
                    TableDefinition::new(I::table_name())
                )?;

                let result = match primary_table.get(primary_key_bytes.value())? {
                    Some(val) => Some(I::Field::decode(val.value())?),
                    None => None,
                };
                Ok(result)
            },
            None => Ok(None),
        }
    }



/*
    /// Retrieves all values associated with a non-unique index key.
    ///
    /// This method supports one-to-many relationships, where a single secondary key maps to
    /// multiple primary records.
    ///
    /// See also: [`get`], [`get_unique`].
    ///
    /// The lookup proceeds in two steps:
    /// 1. Retrieves the set of primary keys using [`KeySet::from_bytes`].
    /// 2. Uses each primary key to load the corresponding record from the primary table.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// * Encoding the index key fails,
    /// * Decoding the index set fails,
    /// * Any record fails to decode,
    /// * Opening a table fails, or
    /// * If a storage error occurs.
    pub fn get_non_unique<K, V, I>(
        &self,
        index_key: &I
    ) -> Result<Option<impl Iterator<Item = Result<V, Error>>>, Error>
    where
        K: Codec<K>,
        V: Codec<V>,
        I: IndexableKey,
    {
        let index_table: redb::ReadOnlyTable<&[u8], &[u8]> = self.0.open_table(
            TableDefinition::new(index_key.index_name())
        )?;

        // Lookup the serialized index set (the set of primary keys)
        let Some(index_bytes) = index_table.get(&*index_key.to_bytes()?)? else {
            return Ok(None);
        };

        // Deserialize the collection of primary keys
        let keys = KeySet::from_bytes(index_bytes.value())?;

        // Prepare the primary table for fetching actual records
        let redb_primary_table: redb::ReadOnlyTable::<&[u8], &[u8]> = self.0.open_table(
            TableDefinition::new(I::table_name())
        )?;

        let primary_table = TableRef::<K, V>::new(redb_primary_table);

        let iterator = primary_table.get_many_by_keys(keys.iter());

        Ok(Some(iterator))
    }
*/




    pub(crate) fn get_non_unique_keys<K, V, I>(
        &self,
        index_key: &I,
    ) -> Result<impl Iterator<Item = Vec<u8>>, Error>
    where
        K: Codec<K>,
        V: Codec<V>,
        I: IndexableKey
    {
        let index_table: redb::ReadOnlyTable<&[u8], &[u8]> = self.0.open_table(
            TableDefinition::new(index_key.index_name())
        )?;

        let key_iterator = index_table.get(&*index_key.to_bytes()?)?
            .map(|index_bytes| KeySet::from_bytes(index_bytes.value()))
            .transpose()?
            .unwrap_or_default()
            .into_iter();

        Ok(key_iterator)
    }

    pub(crate) fn get_index_set<K, V, I>(
        &self,
        index_key: &I,
    ) -> Result<KeySet, Error>
    where
        K: Codec<K>,
        V: Codec<V>,
        I: IndexableKey
    {
        let index_set: KeySet = self.get_non_unique_keys::<K, V, I>(index_key)?
            .collect();

        Ok(index_set)
    }

    pub(crate) fn index_set_op<K, V, I>(
        &self,
        index_key: &I,
        base_set: KeySet,
    ) -> Result<KeySet, Error>
    where
        K: Codec<K>,
        V: Codec<V>,
        I: IndexableKey
    {
        let index_table: redb::ReadOnlyTable<&[u8], &[u8]> = self.0.open_table(
            TableDefinition::new(index_key.index_name())
        )?;

        if let Some(index_bytes) = index_table.get(&*index_key.to_bytes()?)? {
            let filter_set = ArchivedKeySet::from_bytes(index_bytes.value())?;
        }

        Ok(base_set)
    }

    pub fn get_non_unique_values<K, V, I>(
        &self,
        index_key: &I
    ) -> Result<NonUniqueResultIterator<K, V>, Error>
    where
        K: Codec<K>,
        V: Codec<V>,
        I: IndexableKey,
    {
        let keys = self.get_non_unique_keys::<K, V, I>(index_key)?;
        let table = self.table::<K, V>(I::table_name())?;

        let value_iterator = NonUniqueResultIterator {
            table,
            keys: keys.collect::<Vec<_>>().into_iter(),
        };

        Ok(value_iterator)
    }




}



enum QueryOp {
    And,
    Or,
    Not,
}





/*
pub struct NonUniqueKeyIterator<'i, K>
where
    K: Codec<K>,
{
    index_set_iter: crate::indexing::SetPhaseIter<'i>,
    _phantom_data: std::marker::PhantomData<K>
}

impl<'i, K> Iterator for NonUniqueKeyIterator<'i, K>
where
    K: Codec<K>,
{
    type Item = &'i [u8];

    fn next(&mut self) -> Option<Self::Item> {
        self.index_set_iter.next()
    }
}
*/







pub struct NonUniqueResultIterator<K, V>
where
    K: Codec<K>,
    V: Codec<V>,
{
    table: TableRef<K, V>,
    keys: std::vec::IntoIter<Vec<u8>>,
}


impl<K, V> Iterator for NonUniqueResultIterator<K, V>
where
    K: Codec<K>,
    V: Codec<V>,
{
    type Item = Result<V, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let key_bytes = self.keys.next()?;
        Some(self.table.get_by_key_bytes(&key_bytes))
    }
}

















// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl From<ReadTransaction> for Transaction {
    fn from(redb: ReadTransaction) -> Self {
        Self(redb)
    }
}