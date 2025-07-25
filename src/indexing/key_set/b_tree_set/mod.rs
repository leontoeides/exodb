//! An index `KeySet` help manages non-unique indexes. This implementation is powered by the
//! [Rust Standard Library](https://doc.rust-lang.org/std/)'s
//! [BTreeSet](https://doc.rust-lang.org/std/collections/struct.BTreeSet.html).

mod archived_key_set;
mod readable_key_set;
mod upgradable_key_set;

use crate::indexing::key_set::ReadableKeySet;
use std::collections::BTreeSet;

// -------------------------------------------------------------------------------------------------
//
/// A collection of primary keys (serialized as raw bytes) associated with a given index entry. For
/// example, it could be used to list all `creatures` that have a `Habitat` of `"Cloud Forest"`.
///
/// This implementation is powered by the [Rust Standard Library](https://doc.rust-lang.org/std/)'s
/// [BTreeSet](https://doc.rust-lang.org/std/collections/struct.BTreeSet.html) and
/// [David Koloski](https://crates.io/users/djkoloski)'s [rkyv](https://crates.io/crates/rkyv).
///
/// This set lists all of the primary keys associated with an index entry. Primary keys are in
/// serialized form, represented by bytes. This collection is used internally to manage non-unique
/// indicies (one-to-many index relationships).
///
/// # What is `KeySet`?
///
/// `KeySet` lists all the primary keys (serialized as bytes) associated with a given index entry.
/// It's how `atlatl` manages non-unique indexes, where many records share the same indexed value,
/// like several creatures living in the same habitat.
///
/// Imagine that you're tracking creatures in a global biodiversity database:
///
/// ```rust
/// struct Creature {
///     id: u64,
///     name: String,
///     habitat: String,
/// }
/// ```
///
/// Now suppose you want to look up all the creatures living in the Coral Reef. Under the hood, your
/// index might look like this:
///
/// ```text
/// "Coral Reef" → [12, 48, 301]
/// ```
///
/// Here, `12`, `48`, and `301` are primary keys (IDs) of creatures living in that habitat.
///
/// So what's actually stored?
///
/// * `"Coral Reef"` is the secondary key—the value we index.
/// * `[12, 48, 301]` is the `KeySet`, a serialized set of creature IDs that live there.
///
/// ```text
/// ╭──────────────────────────────╮
/// │        Habitat Index         │
/// ├────────────────┬─────────────┤
/// │ "Tundra"       │ [88]        │
/// │ "Coral Reef"   │ [12,48,301] │ ←───┐
/// │ "Rainforest"   │ [19,204]    │     │
/// ╰────────────────┴─────────────╯     │
///                                      ▼
///                           ┌─────────────────────────────────┐
///                           │          Creature Table         │
///                           ├────┬────────────────────────────┤
///                           │ 12 │ Creature { name: "Goby" }  │
///                           │ 48 │ Creature { name: "Crab" }  │
///                           │301 │ Creature { name: "Shrimp"} │
///                           └────┴────────────────────────────┘
/// ```
///
/// When someone says:
///
/// ```rust
/// let reef_creatures = db.get_by_index(Habitat("Coral Reef"));
/// ```
///
/// `atlatl` will:
///
/// 1. Use the secondary key `"Coral Reef"` to search the habitat index.
/// 2. Retrieve a `KeySet`: `[12, 48, 301]`
/// 3. Visit the primary `Creature` table to fetch each record by ID.
///
/// # Summary
///
/// * `KeySet` is a collection of primary keys in their serialized form.
/// * It's used internally to resolve one-to-many relationships via indexes.
/// * Backed by efficient data structures like `BTreeSet` or deserialized on demand.
/// * Critical for fast index queries like: "Give me everything in this habitat."
#[derive(Debug, Default, Eq, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub struct KeySet(pub(crate) BTreeSet<Vec<u8>>);

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl KeySet {
    // +---------------+
    // | Basic Methods |
    // +---------------+

    /// Creates an empty `KeySet` with at least the specified capacity.
    ///
    /// # Notes
    ///
    /// * This method is provided for compatibility. Currently, `std::collections::BTreeSet` does
    ///   not support capacity and `extend_reserve` is only available in the [nightly-only
    ///   experimental API](https://github.com/rust-lang/rust/issues/72631).
    ///
    ///   This method will return a new `BTreeSet` without any set capacity.
    #[inline]
    pub fn with_capacity(_capacity: usize) -> Self {
        Self(BTreeSet::<Vec<u8>>::new())
    }

    /// Inserts the given primary key into the set.
    ///
    /// # Notes
    ///
    /// * The primary key must be represented in serialized form, as bytes.
    #[inline]
    pub fn insert(&mut self, primary_key_bytes: Vec<u8>) {
        self.0.insert(primary_key_bytes);
    }

    /// Removes the given primary key from the set.
    ///
    /// # Notes
    ///
    /// * The primary key must be represented in serialized form, as a slice of bytes.
    #[inline]
    pub fn remove(&mut self, primary_key_bytes: &[u8]) {
        self.0.remove(primary_key_bytes);
    }

    /// Returns the owned, inner collection of primary keys.
    #[inline]
    #[must_use]
    pub fn into_inner(self) -> BTreeSet<Vec<u8>> {
        self.0
    }

    /// Deserializes a `KeySet` from its binary representation.
    ///
    /// # Errors
    ///
    /// * This method will return an error if deserialization of the `KeySet`
    ///   fails.
    #[inline]
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, crate::Error> {
        let archived = rkyv::access::<ArchivedKeySet, rkyv::rancor::Error>(bytes)?;
        let deserialized = rkyv::deserialize::<Self, rkyv::rancor::Error>(archived)?;
        Ok(deserialized)
    }

    /// Serializes the `KeySet` to its binary representation.
    ///
    /// # Errors
    ///
    /// * This method will return an error if serialization of the `KeySet`
    ///   fails.
    #[inline]
    pub fn to_bytes(&self) -> Result<Vec<u8>, crate::Error> {
        Ok(rkyv::to_bytes::<rkyv::rancor::Error>(self)?.to_vec())
    }

    // +----------------+
    // | Set Operations |
    // +----------------+

    /// Returns the intersection of this set and another.
    ///
    /// Primary keys that are present in both sets will be included in the result.
    ///
    /// # Notes
    ///
    /// * The primary keys will be returned in serialized form, as bytes. If needed, each key can be
    ///   deserialized into its full form by using `K::deserialize(item)`
    ///
    /// * Primary keys are used to get actual records from the database.
    #[must_use]
    pub fn intersection(self, other: &impl ReadableKeySet) -> Self {
        let intersection: BTreeSet<Vec<u8>> = self.0
            .into_iter()
            .filter(|member| other.contains(member))
            .collect();

        Self(intersection)
    }

    /// Returns the union of this set and another.
    ///
    /// Primary keys that are present in either set will be included in the result.
    ///
    /// # Notes
    ///
    /// * The primary keys will be returned in serialized form, as bytes. If needed, each key can be
    ///   deserialized into its full form by using `K::deserialize(item)`
    ///
    /// * Primary keys are used to get actual records from the database.
    pub fn union(self, other: KeySet) -> Self {
        let union_result: BTreeSet<Vec<u8>> = self.0
            .into_iter()
            .chain(other.0)
            .collect();

        Self(union_result)
    }

    /// Returns the difference between this set and another.
    ///
    /// Primary keys that are present in `self` but not in `other` will be included in the result.
    ///
    /// # Notes
    ///
    /// * The primary keys will be returned in serialized form, as bytes. If needed, each key can be
    ///   deserialized into its full form by using `K::deserialize(item)`
    ///
    /// * Primary keys are used to get actual records from the database.
    #[must_use]
    pub fn difference(self, other: &impl ReadableKeySet) -> Self {
        let difference: BTreeSet<Vec<u8>> = self.0
            .into_iter()
            .filter(|member| !other.contains(member))
            .collect();

        Self(difference)
    }

    /// Returns the symmetric difference of this set and another.
    ///
    /// Primary keys that are present in either set but not both will be included in the result.
    ///
    /// # Notes
    ///
    /// * The primary keys will be returned in serialized form, as bytes. If needed, each key can be
    ///   deserialized into its full form by using `K::deserialize(item)`
    ///
    /// * Primary keys are used to get actual records from the database.
    pub fn symmetric_difference(self, other: KeySet) -> Self {
        // Start with all keys from `self`.
        let mut result = self.0;

        // Walk each key in `other` exactly once.
        for key in other.0 {
            // If the key was already present, remove it (they cancel out):
            if !result.remove(&key) {
                // Otherwise insert it (new unique key):
                result.insert(key);
            }
        }

        Self(result)
    }
}

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl std::ops::Deref for KeySet {
    type Target = BTreeSet<Vec<u8>>;

    /// Dereferences a `KeySet` into its underlying `BTreeSet<Vec<u8>>` collection.
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for KeySet {
    /// Dereferences a `KeySet` into its underlying `BTreeSet<Vec<u8>>` collection.
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl IntoIterator for KeySet {
    type Item = Vec<u8>;
    type IntoIter = std::collections::btree_set::IntoIter<Vec<u8>>;

    /// Returns an owned iterator over the primary keys in the index set.
    ///
    /// # Notes
    ///
    /// * The primary keys will be returned in serialized form, as raw bytes. If needed, each key
    ///   can be deserialized into its full form by using `K::deserialize(item)`
    ///
    /// * Primary keys are used to get actual records from the database.
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'i> IntoIterator for &'i KeySet {
    type Item = &'i Vec<u8>;
    type IntoIter = std::collections::btree_set::Iter<'i, Vec<u8>>;

    /// Returns an borrowed iterator over the primary keys in the index set.
    ///
    /// # Notes
    ///
    /// * The primary keys will be returned in serialized form, as raw bytes. If needed, each key
    ///   can be deserialized into its full form by using `K::deserialize(item)`
    ///
    /// * Primary keys are used to get actual records from the database.
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl FromIterator<Vec<u8>> for KeySet {
    /// Builds an `KeySet` collection from an iterator over primary keys.
    ///
    /// # Notes
    ///
    /// * The primary key must be represented in serialized form, as a slice of bytes.
    fn from_iter<I: IntoIterator<Item = Vec<u8>>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl FromIterator<KeySet> for KeySet {
    /// Builds an `KeySet` collection from an iterator over other key sets.
    fn from_iter<I: IntoIterator<Item = KeySet>>(iter: I) -> Self {
        let mut dest_key_set = KeySet::default();

        iter
            .into_iter()
            .for_each(|src_key_set| dest_key_set.extend(src_key_set.0));

        dest_key_set
    }
}

impl Extend<Vec<u8>> for KeySet {
    /// Extends a `KeySet` collection using an iterator over primary keys.
    ///
    /// # Notes
    ///
    /// * The primary keys will be returned in serialized form, as raw bytes. If needed, each key
    ///   can be deserialized into its full form by using `K::deserialize(item)`
    fn extend<T: IntoIterator<Item=Vec<u8>>>(&mut self, iter: T) {
       self.0.extend(iter)
    }
}

// -------------------------------------------------------------------------------------------------
//
// Tests

#[test]
fn set_operations() {
    let a = KeySet::from_iter(vec![vec![1], vec![2], vec![3]]);
    let b = KeySet::from_iter(vec![vec![3], vec![4]]);
    let c = KeySet::from_iter(vec![vec![2]]);

    let result = a
        .intersection(&b)     // [3]
        .union(c)             // [2, 3]
        .difference(&b);      // [2]

    let expected = KeySet::from_iter(vec![vec![2]]);
    assert_eq!(result, expected);
}