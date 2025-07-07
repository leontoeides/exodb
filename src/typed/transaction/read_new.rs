use crate::indexing::ArchivedKeySet;
use crate::indexing::HasPrimaryKey;
use crate::indexing::HasTable;
use crate::indexing::IndexLookup;
use crate::indexing::IndexMultiLookup;
use crate::indexing::KeySet;
use crate::indexing::PreparedIndexLookup;
use crate::indexing::ReadableKeySet;
use crate::querying::Query;
use crate::typed::TableRef;
use crate::{Codec, Error};
use redb::{ReadTransaction, TableDefinition};

// -------------------------------------------------------------------------------------------------

pub struct Transaction(ReadTransaction);

type RedbReadOnlyTable = redb::ReadOnlyTable<&'static [u8], &'static [u8]>;

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
    fn get_primary_keys_with_exclusions<K>(
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
    fn get_index_keys<K, V, I>(
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

    /// Performs an intersection between a base query and an indexed filter, returning a set of
    /// primary keys.
    ///
    /// For example, this could find animals that live in both a great barrier reef and a coral
    /// cove.
    ///
    /// # Errors
    ///
    /// * The `redb::Table` that contains the index data could not be opened.
    ///
    /// * Storage errors which includes issues such as input/output failures, disk errors,
    ///   permissions errors, data corruption, previously failed operations, or lock poisoning.
    ///
    /// * Deserialization errors when instantiating a `&ArchivedKeySet` from the index entry.
    ///   Invalid key set data.
    #[inline]
    fn handle_and_then<K, V>(
        &self,
        base_query: Query<V>,
        filtering_index: Box<dyn IndexLookup<Record = V>>,
    ) -> Result<KeySet, Error>
    where
        K: Codec<K>,
        V: Codec<V> + HasTable,
    {
        // Open index table. Like accessing a map of the sanctuary’s habitats (for example, a coral
        // reef or savanna).
        let index_table: RedbReadOnlyTable = self.0.open_table(
            TableDefinition::new(filtering_index.index_name())
        )?;

        // Look for the specified index key (or secondary key) from the index table. For example, we
        // might be searching for animals in `Habitat("Coral Cove")`.
        if let Some(key_set_bytes) = index_table.get(&*filtering_index.index_key_bytes()?)? {
            // Evaluate the left-hand set of the `difference` operation. For example, all creatures
            // living in `Habitat("Great Barrier Reef")`.
            let query_result = self.query::<K, V>(base_query)?;

            // Deserialize the key set (or collection of primary keys) from the index entry. For
            // example, it could represent the creatures in the specified feeding ground
            // `Habitat("Coral Cove")`. This is the right-hand set of the `and` operation.
            let filtering_keys = ArchivedKeySet::from_bytes(key_set_bytes.value())?;

            // Perform intersection with the primary keys that: 1. result from the `base_query`
            // (Great Barrier Reef) and 2. that are associated with the specified index key (Coral
            // Cove). Now we would only have creatures that live in both the `Habitat("Rain
            // Forest")` and in the `Habitat("Temperate Forest")`. For example, `"Clownfish"`,
            // `"Parrotfish"`, `"Sea Turtle"`.
            Ok(query_result.intersection(&filtering_keys))
        } else {
            // No index entry was found, however, the left-hand side of the operation still needs
            // to be evaluated. An example scenario would be that the right-hand set
            // `Habitat("Lunar Lagoon")` does not exist, and there would be no index entry for it.
            //
            // Since this place doesn't exist, no known creatures can live there, the intersection
            // would result in nothing. Return an empty set.
            tracing::debug!("index not found, or empty index encountered during operation");
            Ok(KeySet::default())
        }
    }

    /// Performs a union between a base query and an indexed filter, returning a set of primary
    /// keys.
    ///
    /// For example, this could find all animals that live in either a great barrier reef or the
    /// Serengeti plains.
    ///
    /// # Errors
    ///
    /// * The `redb::Table` that contains the index data could not be opened.
    ///
    /// * Storage errors which includes issues such as input/output failures, disk errors,
    ///   permissions errors, data corruption, previously failed operations, or lock poisoning.
    ///
    /// * Deserialization errors when instantiating a `&ArchivedKeySet` from the index entry.
    ///   Invalid key set data.
    #[inline]
    fn handle_or_else<K, V>(
        &self,
        base_query: Query<V>,
        extending_index: Box<dyn IndexLookup<Record = V>>,
    ) -> Result<KeySet, Error>
    where
        K: Codec<K>,
        V: Codec<V> + HasTable,
    {
        // Open index table. Like accessing a map of the sanctuary’s habitats (for example, a coral
        // reef or savanna).
        let index_table: RedbReadOnlyTable = self.0.open_table(
            TableDefinition::new(extending_index.index_name())
        )?;

        // Attempt to get the specified index key (or secondary key) from the index table. For
        // example, we might be looking for creatures that live in a `Habitat("Serengeti Plains")`.
        if let Some(key_set_bytes) = index_table.get(&*extending_index.index_key_bytes()?)? {
            // Evaluate the left-hand set of the `difference` operation. For example: it could
            // produce the result of a `Habitat("Great Barrier Reef")` query.
            let query_result = self.query::<K, V>(base_query)?;

            // Deserialize the key set (or collection of primary keys) from the index entry. This is
            // the right-hand set of the `or` operation. For example it could represent the
            // creatures in `Habitat("Serengeti Plains")`:
            let extending_keys = KeySet::from_bytes(key_set_bytes.value())?;

            // Perform union with the primary keys that: 1. result from the `base_query` (Great
            // Barrier Reef) and 2. that are associated with the specified index key (Serengeti
            // Plains). Now would have all creatures that live in the
            // `Habitat("Great Barrier Reef")` or in the `Habitat("Serengeti Plains")`. For example,
            // `"Clownfish"`, `"Parrotfish"`, `"Cheetah"`, or `"Gazelle"`.
            //
            // This following `len` check helps perform the union with the least amount of work:
            if extending_keys.len() <= query_result.len() {
                Ok(query_result.union(extending_keys))
            } else {
                Ok(extending_keys.union(query_result))
            }
        } else {
            // No index entry was found, however, the left-hand side of the operation still needs
            // to be evaluated. An example scenario would be that the right-hand set
            // `Habitat("Lunar Lagoon")` does not exist, and there would be no index entry
            // for it.
            //
            // However we still want to return the critters for the left-hand side
            // `Habitat("Great Barrier Reef")`, like `"Clownfish"` and `"Sea Turtle"`.
            self.query::<K, V>(base_query)
        }
    }

    /// Performs a set difference between a base query and an indexed filter, returning a set of
    /// primary keys.
    ///
    /// For example, this could find animals living in a coral reef but not in a savanna.
    ///
    /// # Errors
    ///
    /// * The `redb::Table` that contains the index data could not be opened.
    ///
    /// * Storage errors which includes issues such as input/output failures, disk errors,
    ///   permissions errors, data corruption, previously failed operations, or lock poisoning.
    ///
    /// * Deserialization errors when instantiating a `&ArchivedKeySet` from the index entry.
    ///   Invalid key set data.
    #[inline]
    fn handle_difference_of<K, V>(
        &self,
        base_query: Query<V>,
        filtering_index: Box<dyn IndexLookup<Record = V>>,
    ) -> Result<KeySet, Error>
    where
        K: Codec<K>,
        V: Codec<V> + HasTable,
    {
        // Open index table. Like accessing a map of the sanctuary’s habitats (for example, a coral
        // reef or savanna).
        let index_table: RedbReadOnlyTable = self.0.open_table(
            TableDefinition::new(filtering_index.index_name())
        )?;

        // Attempt to get the specified index key (or secondary key) from the index table. For
        // example, we might be looking for creatures that live in a `Habitat("Serengeti Plains")`.
        if let Some(key_set_bytes) = index_table.get(&*filtering_index.index_key_bytes()?)? {
            // Evaluate the left-hand set of the `difference` operation. For example: it could
            // produce the result of a `Habitat("Great Barrier Reef")` query.
            let query_result = self.query::<K, V>(base_query)?;

            // Deserialize the key set (or collection of primary keys) from the index entry. This is
            // the right-hand set of the `difference` operation. For example, it could represent the
            // creatures in `Habitat("Serengeti Plains")`:
            let filtering_keys = ArchivedKeySet::from_bytes(key_set_bytes.value())?;

            // Perform difference with the primary keys that: 1. result from the `base_query` (Great
            // Barrier Reef) and 2. that are associated with the specified index key (Serengeti
            // Plains). Now would have all creatures that live in the
            // `Habitat("Great Barrier Reef")` and not in the `Habitat("Serengeti Plains")`.
            Ok(query_result.difference(&filtering_keys))
        } else {
            // No index entry was found, however, the left-hand side of the operation still needs
            // to be evaluated. An example scenario would be that the right-hand set
            // `Habitat("Lunar Lagoon")` does not exist, and there would be no index entry
            // for it.
            //
            // Since there is no right-hand set to subtract with, return the left-hand set as-is:
            self.query::<K, V>(base_query)
        }
    }

    /// When performing a `not` query (e.g. exclude records from a given index key), there are 3
    /// possible outcomes when the exclusion key does not exist in the index.
    ///
    /// These outcomes are controlled by `Cargo.toml` features:
    ///
    /// # Example
    ///
    /// ```rust
    /// // Exclude all animals in the "Forest" habitat
    /// Query::Not(Box::new(Habitat::new("Forest")))
    /// ```
    ///
    /// Now suppose we query using an invalid or nonexistent key:
    ///
    /// ```rust
    /// // "Sixteenth Moon of Mars" is not a known habitat
    /// Query::Not(Box::new(Habitat::new("Sixteenth Moon of Mars")))
    /// ```
    ///
    /// # Feature Flags and Behavior
    ///
    /// | Feature                    | Behavior                                                                     |
    /// |----------------------------|------------------------------------------------------------------------------|
    /// | `missing-not-return-empty` | Returns no records. Safer default for production                             |
    /// | `missing-not-return-all`   | Returns all records (nothing to exclude). Default for `i-know-what-im-doing` |
    /// | `missing-not-return-error` | Returns a hard error if the exclusion key is missing                         |
    ///
    /// # Recommendations
    /// * Use `not-missing-return-empty` in production to avoid accidental overmatching.
    /// * Use `not-missing-return-all` during development if you're doing exploratory queries.
    /// * Use `not-missing-error` when running critical queries where exclusion failures should
    ///   panic.
    #[inline]
    fn handle_empty_exclusion<K, V>(
        &self,
        index_table: &'static str,
        _secondary_key: Option<Vec<u8>>,
    ) -> Result<KeySet, Error>
    where
        K: Codec<K>,
        V: Codec<V> + HasTable,
    {
        #[cfg(debug_assertions)]
        tracing::warn!("an index key for `{index_table}` was not found or was empty");

        #[cfg(feature = "missing-not-return-empty")]
        return Ok(KeySet::default());

        #[cfg(feature = "missing-not-return-all")]
        return self.collect_all_keys::<K, V>();

        #[cfg(feature = "missing-not-return-error")]
        Err(Error::NotKeyMissing {
            index: index.to_string(),
            _secondary_key,
        })
    }

    /// # Errors
    ///
    /// * Storage errors which includes issues such as input/output failures, disk errors,
    ///   permissions errors, data corruption, previously failed operations, or lock poisoning.
    #[inline]
    fn handle_not<K, V>(
        &self,
        query: Box<dyn IndexLookup<Record = V>>,
    ) -> Result<KeySet, Error>
    where
        K: Codec<K>,
        V: Codec<V> + HasTable,
    {
        // Open the index table. For example, this could be the index that lists all `Habitat`s and
        // the creatures in each habitat.
        let index_table: RedbReadOnlyTable =
            self.0.open_table(TableDefinition::new(query.index_name()))?;

        // Attempt to get the index entry we will exclude. For example, if we're wanting to exclude
        // forest critters, we're trying to get the index entry that lists all creatures in
        // `Habitat("Forest")`.
        if let Some(key_set_bytes) = index_table.get(&*query.index_key_bytes()?)? {
            // At this point we'll have the primary keys for all the forest creatures we'd like to
            // exclude. All of the creatures in `Habitat("Forest")`.
            let primary_keys_to_be_excluded = ArchivedKeySet::from_bytes(key_set_bytes.value())?;

            // If we were searching for all known creatures in `Habitat("Jupiter")`, the index would
            // be either non-existent or empty. In this case, it might not be desirable to return
            // all creatures not on Jupiter, since that would represent the entire database.
            //
            // We'll call `handle_empty_exclusion` to handle the empty or the non-existent set
            // according to the crate's features and settings.
            if primary_keys_to_be_excluded.is_empty() {
                return self.handle_empty_exclusion::<K, V>(
                    query.index_name(),
                    Some(key_set_bytes.value().to_vec())
                );
            }

            // Open the primary table. For example, this could be a table lists all creatures.
            let primary_table: RedbReadOnlyTable =
                self.0.open_table(TableDefinition::new(query.table_name()))?;

            // This will iterate over every single entry in the database and filter out the ones
            // in `primary_keys_to_be_excluded`. For example, if the caller's specified `not` index
            // look-up was `Habitat("Tide Pool")`, all creatures except ones like `"Snail"`,
            // `"Sea Star"` will be returned.
            //
            // The primary keys for these records will be returned to the caller in a `KeySet`
            // collection. These keys can be used to retrieve the actual, full records from the
            // database.
            primary_table
                .range::<&[u8]>(..)?
                .filter_map(|result| result
                    .map(|(key_guard, _)|
                        if !primary_keys_to_be_excluded.contains(key_guard.value()) {
                            Some(key_guard.value().to_vec())
                        } else {
                            None
                        }
                    )
                    .map_err(Into::into)
                    .transpose()
                )
                .collect::<Result<KeySet, Error>>()
        } else {
            // If we were searching for all known creatures in `Habitat("Jupiter")`, the index would
            // be either non-existent or empty. In this case, it might not be desirable to return
            // all creatures not on Jupiter, since that would represent the entire database.
            //
            // We'll call `handle_empty_exclusion` to handle the empty or the non-existent set
            // according to the crate's features and settings.
            self.handle_empty_exclusion::<K, V>(
                query.index_name(),
                query.index_key_bytes().ok()
            )
        }
    }

    #[inline]
    fn handle_not_in<K, V>(
        &self,
        query: Box<dyn IndexMultiLookup<Record = V>>,
    ) -> Result<KeySet, Error>
    where
        K: Codec<K>,
        V: Codec<V> + HasTable,
    {
        // None of these errors should happen. If they do, they represent an empty
        // `IndexMultiLookup` which would likely be the result of a programming error in this
        // crate:
        let table_name = query.table_name().ok_or(Error::MissingPrimaryTableName)?;
        let index_name = query.index_name().ok_or(Error::MissingIndexTableName)?;
        let index_kind = query.index_kind().ok_or(Error::MissingIndexKind)?;

        // Iterate over the index entries we want to exclude. For example, if we're wanting to
        // exclude critters from `Habitat("Temperature Forest")` and `Habitat("Wetlands")`, we'll
        // get the primary keys for them here in this clause:
        let primary_keys_to_be_excluded: KeySet = query
            // Rust doesn't allow iterators from traits (`-> impl Iterator`), and support for
            // iterators from traits isn't great, at time of writing. However, we can iterate over a
            // key-set returned from the `IndexMultiLookup` trait. So, let's do that:
            .to_key_set()?
            .0
            .into_iter()
            .flat_map(|secondary_key_bytes| {
                // The `PreparedIndexLookup` struct implements the `IndexLookup` trait and can be
                // passed to `get_index_keys`. First, we'll repack the data returned from the
                // `IndexMultiLookup` trait methods into a `PreparedIndexLookup` struct for
                // forwarding.
                let index_lookup = PreparedIndexLookup::new(
                    index_name,
                    *index_kind,
                    secondary_key_bytes
                );

                // Get all of the primary keys for a single index entry. For example, this would be
                // one of: `Habitat("Temperature Forest")` or `Habitat("Wetlands")` per iteration.
                self.get_index_keys::<K, V, PreparedIndexLookup<V>>(
                    Box::new(index_lookup)
                )
            })
            .flatten()
            .collect();

        // If we were searching for all known creatures in `Habitat("Jupiter")`, the index would be
        // either non-existent or empty. In this case, it might not be desirable to return all
        // creatures not on Jupiter, since that would represent the entire database.
        //
        // We'll call `handle_empty_exclusion` to handle the empty or the non-existent set according
        // to the crate's features and settings.
        if primary_keys_to_be_excluded.is_empty() {
            return self.handle_empty_exclusion::<K, V>(
                index_name,
                None
            );
        }

        // This will iterate over every single entry in the database and filter out the ones in
        // `primary_keys_to_be_excluded`. For example, if the caller's specified `not` index look-up
        // was `Habitat("Tide Pool")`, all creatures except ones like `"Snail"`, `"Sea Star"` will
        // be returned.
        //
        // The primary keys for these records will be returned to the caller in a `KeySet`
        // collection. These keys can be used to retrieve the actual, full records from the
        // database.
        self.get_primary_keys_with_exclusions::<K>(table_name, &primary_keys_to_be_excluded)
    }

    #[inline]
    fn handle_any_of<K, V>(
        &self,
        query: Box<dyn IndexMultiLookup<Record = V>>,
    ) -> Result<KeySet, Error>
    where
        K: Codec<K>,
        V: Codec<V> + HasTable,
    {
        // None of these errors should happen. If they do, they represent an empty
        // `IndexMultiLookup` which would likely be the result of a programming error in this
        // crate:
        let index_name = query.index_name().ok_or(Error::MissingIndexTableName)?;
        let index_kind = query.index_kind().ok_or(Error::MissingIndexKind)?;

        // Iterate over the index entries we want to include. For example, if we're wanting to
        // include any critters from `Habitat("Temperature Forest")` and `Habitat("Wetlands")`,
        // we'll get the primary keys for them here in this clause:
        let primary_keys_to_be_included = query
            // Rust doesn't allow iterators from traits (`-> impl Iterator`), and support for
            // iterators from traits isn't great, at time of writing. However, we can iterate over a
            // key-set returned from the `IndexMultiLookup` trait. So, let's do that:
            .to_key_set()?
            .0
            .into_iter()
            .flat_map(|secondary_key_bytes| {
                // The `PreparedIndexLookup` struct implements the `IndexLookup` trait and can be
                // passed to `get_index_keys`. First, we'll repack the data returned from the
                // `IndexMultiLookup` trait methods into a `PreparedIndexLookup` struct for
                // forwarding.
                let index_lookup = PreparedIndexLookup::new(
                    index_name,
                    *index_kind,
                    secondary_key_bytes
                );

                // Get all of the primary keys for a single index entry. For example, this would be
                // one of: `Habitat("Temperature Forest")` or `Habitat("Wetlands")` per iteration.
                self.get_index_keys::<K, V, PreparedIndexLookup<V>>(
                    Box::new(index_lookup)
                )
            })
            .flatten()
            .collect();

        Ok(primary_keys_to_be_included)
    }

    pub fn query<K, V>(
        &self,
        query: impl Into<Query<V>>,
    ) -> Result<KeySet, Error>
    where
        K: Codec<K>,
        V: Codec<V> + HasTable,
    {
        let query: Query<V> = query.into();

        let key_set = match query {
            Query::Lookup(index_lookup) =>
                self.get_index_keys::<K, V, dyn IndexLookup<Record = V>>(index_lookup)?,

            Query::AndThen(base_query, filtering_index) =>
                self.handle_and_then::<K, V>(*base_query, filtering_index)?,

            Query::Without(base_query, filtering_index) =>
                self.handle_difference_of::<K, V>(*base_query, filtering_index)?,

            Query::OrElse(base_query, extending_index) =>
                self.handle_or_else::<K, V>(*base_query, extending_index)?,

            Query::Not(index_lookup) =>
                self.handle_not::<K, V>(index_lookup)?,

            Query::AnyOf(index_multi_lookup) =>
                self.handle_any_of::<K, V>(index_multi_lookup)?,

            Query::NotIn(index_multi_lookup) =>
                self.handle_not_in::<K, V>(index_multi_lookup)?,

            Query::Group(inner) => self.query::<K, V>(*inner)?,
        };

        Ok(key_set)
    }

































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