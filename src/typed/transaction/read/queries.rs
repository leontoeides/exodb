use crate::indexing::{
    ArchivedKeySet,
    HasTable,
    IndexLookup,
    IndexMultiLookup,
    KeySet,
    PreparedIndexLookup,
    ReadableKeySet
};
use ::redb::TableDefinition;
use crate::querying::Query;
use crate::typed::transaction::read::RedbReadOnlyTable;
use crate::{Codec, Error};

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl crate::typed::transaction::read::Transaction {
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

            Query::Not(index_lookup) =>
                self.handle_not::<K, V>(index_lookup)?,

            Query::And(base_query, filtering_index) =>
                self.handle_and_then::<K, V>(*base_query, filtering_index)?,

            Query::Difference(base_query, filtering_index) =>
                self.handle_difference_of::<K, V>(*base_query, filtering_index)?,

            Query::Or(base_query, extending_index) =>
                self.handle_or_else::<K, V>(*base_query, extending_index)?,

            Query::Xor(base_query, extending_index) =>
                self.handle_or_else::<K, V>(*base_query, extending_index)?,

            Query::AnyOf(index_multi_lookup) =>
                self.handle_any_of::<K, V>(index_multi_lookup)?,

            Query::NotIn(index_multi_lookup) =>
                self.handle_not_in::<K, V>(index_multi_lookup)?,

            Query::Group(inner) => self.query::<K, V>(*inner)?,
        };

        Ok(key_set)
    }
}