use crate::indexing::HasPrimaryKey;
use crate::indexing::HasTable;
use crate::indexing::IndexLookup;
use crate::indexing::KeySet;
use crate::indexing::ReadableKeySet;
use crate::typed::TableRef;
use crate::typed::transaction::read::RedbReadOnlyTable;
use crate::typed::transaction::read::Transaction;
use crate::{Codec, Error};
use redb::TableDefinition;

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl Transaction {

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
        let primary_table: RedbReadOnlyTable = self.0.open_table(
            TableDefinition::new(V::table_name())
        )?;

        if let Some(value) = primary_table.get(&*PK::serialize(primary_key)?)? {
            Ok(Some(V::deserialize(value.value())?))
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
    /*
    fn get_unique<I>(&self, index_key: &I) -> Result<Option<I::Field>, Error>
    where
        I: IndexableKey,
    {
        let index_table: RedbReadOnlyTable = self.0.open_table(
            TableDefinition::new(index_key.index_name())
        )?;

        match index_table.get(&*index_key.to_bytes()?)? {
            Some(primary_key_bytes) => {
                let primary_table: RedbReadOnlyTable = self.0.open_table(
                    TableDefinition::new(I::table_name())
                )?;

                let result = match primary_table.get(primary_key_bytes.value())? {
                    Some(val) => Some(I::Field::deserialize(val.value())?),
                    None => None,
                };
                Ok(result)
            },
            None => Ok(None),
        }
    } */

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
        let index_table: RedbReadOnlyTable = self.0.open_table(
            TableDefinition::new(index_key.index_name())
        )?;

        // Lookup the serialized index set (the set of primary keys)
        let Some(index_bytes) = index_table.get(&*index_key.to_bytes()?)? else {
            return Ok(None);
        };

        // Deserialize the collection of primary keys
        let keys = KeySet::from_bytes(index_bytes.value())?;

        // Prepare the primary table for fetching actual records
        let redb_primary_table: redb::RedbReadOnlyTable::<&[u8], &[u8]> = self.0.open_table(
            TableDefinition::new(I::table_name())
        )?;

        let primary_table = TableRef::<K, V>::new(redb_primary_table);

        let iterator = primary_table.get_many_by_keys(keys.iter());

        Ok(Some(iterator))
    }
*/

    /// Returns an iterator over all primary keys' bytes in the database.
    ///
    /// This would represent the primary keys for all creatures in the database, including all
    /// species, habitats, diets, etc.
    ///
    /// # Notes
    ///
    /// * The primary keys will be returned in serialized form, as raw bytes. If needed, each key
    ///   can be deserialized into its full form by using `K::deserialize(item)`
    #[inline]
    fn get_primary_keys_bytes_iter<K>(
        &self,
        primary_table_name: &'static str,
    ) -> Result<impl Iterator<Item = Result<Vec<u8>, Error>>, Error>
    where
        K: Codec<K>
    {
        let primary_table: RedbReadOnlyTable =
            self.0.open_table(TableDefinition::new(primary_table_name))?;

        let primary_keys_bytes_iterator = primary_table
            .range::<&[u8]>(..)?
            .map(|result| result
                .map(|(key_guard, _)| key_guard.value().to_vec())
                .map_err(Error::from)
            );

        Ok(primary_keys_bytes_iterator)
    }

    /// Returns all primary keys' in the database as a key set.
    ///
    /// This would represent the primary keys for all creatures in the database, including all
    /// species, habitats, diets, etc.
    ///
    /// # Notes
    ///
    /// * The primary keys will be returned in serialized form, as raw bytes. If needed, each key
    ///   can be deserialized into its full form by using `K::deserialize(item)`
    #[inline]
    fn get_primary_keys<K>(
        &self,
        primary_table_name: &'static str,
    ) -> Result<KeySet, Error>
    where
        K: Codec<K>
    {
        let primary_key_set: KeySet = self.get_primary_keys_bytes_iter::<K>(primary_table_name)?
            .collect::<Result<KeySet, Error>>()?;

        Ok(primary_key_set)
    }

    /// Returns an iterator over all primary keys' bytes in the database excluding the primary keys
    /// listed in the provided `KeySet`.
    ///
    /// These primary keys would represent all creatures in the database, including all species,
    /// habitats, diets, etc. except the ones specified in the `exclusions` `KeySet`.
    ///
    /// This is used for unary `not` and similar operators.
    ///
    /// # Notes
    ///
    /// * The primary keys will be returned in serialized form, as raw bytes. If needed, each key
    ///   can be deserialized into its full form by using `K::deserialize(item)`
    #[inline]
    fn get_primary_keys_bytes_with_exclusions_iter<K>(
        &self,
        primary_table_name: &'static str,
        exclusions: &impl ReadableKeySet
    ) -> Result<impl Iterator<Item = Result<Vec<u8>, Error>>, Error>
    where
        K: Codec<K>
    {
        let primary_table: RedbReadOnlyTable =
            self.0.open_table(TableDefinition::new(primary_table_name))?;

        let primary_key_iterator = primary_table
            .range::<&[u8]>(..)?
            .filter_map(|result| result
                .map(|(key_guard, _)| if !exclusions.contains(key_guard.value()) {
                    Some(key_guard.value().to_vec())
                } else {
                    None
                })
                .map_err(Into::into)
                .transpose()
            );

        Ok(primary_key_iterator)
    }

    /// Returns all primary keys' in the database as a key set.
    ///
    /// This would represent the primary keys for all creatures in the database, including all
    /// species, habitats, diets, etc.
    ///
    /// This is used for unary `not` and similar operators.
    ///
    /// # Notes
    ///
    /// * The primary keys will be returned in serialized form, as raw bytes. If needed, each key
    ///   can be deserialized into its full form by using `K::deserialize(item)`
    #[inline]
    pub(crate) fn get_primary_keys_with_exclusions<K>(
        &self,
        primary_table_name: &'static str,
        exclusions: &impl ReadableKeySet
    ) -> Result<KeySet, Error>
    where
        K: Codec<K>
    {
        let primary_key_set: KeySet = self.get_primary_keys_bytes_with_exclusions_iter::<K>(
                primary_table_name,
                exclusions
            )?
            .collect::<Result<KeySet, Error>>()?;

        Ok(primary_key_set)
    }

    /// Returns an iterator over all primary keys using a secondary index look-up.
    ///
    /// For example, `Habitat("Savanna")` might return an iterator over the primary keys for the
    /// `"African Elephant"`, the `"Zebra"`, and the `"Lion"` creatures.
    #[inline]
    fn get_index_keys_iter<K, V, I>(
        &self,
        index_lookup: Box<I>,
    ) -> Result<impl Iterator<Item = Vec<u8>>, Error>
    where
        K: Codec<K>,
        V: Codec<V>,
        I: IndexLookup + ?Sized
    {
        let index_table: RedbReadOnlyTable =
            self.0.open_table(TableDefinition::new(index_lookup.index_name()))?;

        let keys_iterator = index_table.get(&*index_lookup.index_key_bytes()?)?
            .map(|index_bytes| KeySet::from_bytes(index_bytes.value()))
            .transpose()?
            .unwrap_or_default()
            .0
            .into_iter();

        Ok(keys_iterator)
    }

    /// Returns an iterator over all primary keys for a secondary index look-up.
    ///
    /// For example, `Habitat("Temperate Forest")` might return the primary keys for the `"Black
    /// Bear"`, the `"Deer"`, and the `"Squirrel"` creatures.
    #[inline]
    pub(crate) fn get_index_keys<K, V, I>(
        &self,
        index_lookup: Box<I>,
    ) -> Result<KeySet, Error>
    where
        K: Codec<K>,
        V: Codec<V>,
        I: IndexLookup + ?Sized
    {
        let index_table: RedbReadOnlyTable =
            self.0.open_table(TableDefinition::new(index_lookup.index_name()))?;

        let key_set = index_table.get(&*index_lookup.index_key_bytes()?)?
            .map(|index_bytes| KeySet::from_bytes(index_bytes.value()))
            .transpose()?
            .unwrap_or_default();

        Ok(key_set)
    }

/*
    pub fn get_index_values<K, V, I>(
        &self,
        index_lookup: &I
    ) -> Result<NonUniqueResultIterator<K, V>, Error>
    where
        K: Codec<K>,
        V: Codec<V>,
        I: IndexLookup,
    {
        let keys = self.get_index_keys::<K, V, I>(index_lookup)?;
        let table = self.table::<K, V>(I::table_name())?;

        let value_iterator = NonUniqueResultIterator {
            table,
            keys: keys.collect::<Vec<_>>().into_iter(),
        };

        Ok(value_iterator)
    }
*/




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












