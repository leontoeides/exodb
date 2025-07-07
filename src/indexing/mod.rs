mod key_set;


pub use crate::indexing::key_set::{ArchivedKeySet, KeySet, ReadableKeySet, UpgradableKeySet};




















use crate::{Codec, Error};

// atlatl stuff

pub trait HasTable {
    /// Returns the name of the primary record table associated with this index.
    ///
    /// This is represents a `redb::Table` name that contains the records. Each record in this table
    /// consists of a `K` primary key and a `V` value.
    ///
    /// For example, a primary table could consist of all types creatures on Earth: of varying
    /// `Habitat`s, `Species`, `Diet`s, etc.
    fn table_name() -> &'static str;
}

/// A trait for types that can declare their associated table name and primary key.
pub trait HasPrimaryKey<'pk, PK: Codec<PK>> {
    /// Returns the primary key from this value.
    fn primary_key(&'pk self) -> PrimaryKey<'pk, PK>;
}

/// Primary key produced from a value.
#[derive(Debug)]
pub struct PrimaryKey<'pk, PK: Codec<PK>>(&'pk PK);

impl<'pk, PK: Codec<PK>> PrimaryKey<'pk, PK> {
    /// Creates a new primary key.
    pub const fn new(primary_key: &'pk PK) -> Self {
        Self(primary_key)
    }

    /// Returns a reference to the `PrimaryKey`'s underlying `K` type.
    #[must_use] pub const fn as_ref(&'pk self) -> &'pk PK {
        self.0
    }

    /// Encodes the primary key into bytes for use with a `redb` table.
    ///
    /// # Errors
    ///
    /// * Returns an error if the key cannot be serialized by the active
    ///   `Codec`.
    pub fn to_bytes(&self) -> Result<Vec<u8>, Error> {
        Ok(PK::serialize(self.0)?)
    }
}

impl<'pk, PK: Codec<PK>> From<&'pk PK> for PrimaryKey<'pk, PK> {
    fn from(primary_key: &'pk PK) -> Self {
        Self(primary_key)
    }
}












/// Represents a typed, semantic key used to look up records by a specific indexed field.
///
/// This trait enables type-safe and ergonomic lookups such as:
///
/// ```rust
/// let user = db.get_indexed(Birthday(date))?;
/// ```
///
/// It associates:
/// * A record type (`User`)
/// * A field type (`NaiveDate`)
/// * The index table name, and
/// * The logic to encode the field for lookup
///
/// Implementors are used as keys into secondary index tables.
pub trait IndexLookup {
    /// Type of the parent record.
    type Record: HasTable;

    /// Returns the name of the primary record table associated with this index.
    ///
    /// This is represents a `redb::Table` name that contains the records. Each record in this table
    /// consists of a `K` primary key and a `V` value.
    ///
    /// For example, a primary table could consist of all types creatures on Earth: of varying
    /// `Habitat`s, `Species`, `Diet`s, etc.
    #[must_use] fn table_name(&self) -> &'static str {
        Self::Record::table_name()
    }

    /// Returns the name of the secondary index table being queried.
    ///
    /// This is represents a `redb::Table` name that's used as an index. Each record in this table
    /// consists of a `I` secondary key and a `KeySet` (a collection of serialized primary keys).
    ///
    /// For example, an index table could be used to categorize all the different types creatures by
    /// `Habitat`, by `Diet`, etc.
    ///
    /// The `Habitat` index might contain an entry for `"Temperate Forest"` that references
    /// `"Raccoon"`, `"Skunk"`, and `"Deer"`, etc.
    #[must_use] fn index_name(&self) -> &'static str;

    /// Returns the kind of index: `Unique` or `NonUnique`.
    #[must_use] fn index_kind(&self) -> &IndexKind;

    /// Encodes the index key into bytes to look-up the key-set.
    ///
    /// For example, this secondary key (in raw serialized bytes) could represent a
    /// `Habitat("Savannah")` and the values returned from the index table could be the primary keys
    /// to get a `"Lion"` and a `"Giraffe"`.
    ///
    /// # Notes
    ///
    /// * This key is returned in serialized form, as raw bytes.
    ///
    /// # Errors
    ///
    /// Consult the documentation of the codec backend you are using for more detail on
    /// serialization behavior and potential limitations.
    fn index_key_bytes(&self) -> Result<Vec<u8>, crate::Error>;

    /// Optional method to provide a human-readable representation of the index key.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", std::any::type_name::<Self>())
    }
}





















/// The `PreparedIndexLookup` tells `atlatl` how to look-up the primary records (`"Scorpion"`,
/// `"Snail"`, `"Jerboa"`) from an index (`Habitat("Desert")`, `Habitat("Tide Pool")`).
///
/// This `struct` implements `IndexLookup` and is used to translate a `IndexMultiLookup` into a
/// collection or iterator over `IndexLookup` types. This is the type that allows this to happen.
///
/// This tells `atlatl` that:
///
/// * `index_name` · We want to look up creatures by `Habitat`. This is the name of the table that
///   contains the list of habitats and creatures.
///
/// * `index_kind` · That `Habitat` is a `NonUnique` index, and that more than one species can be
///   associated with a single habitat.
///
/// * `index_key_bytes` · And that we would like to see the critters listed in the
///   `Habitat("Rain Forest")` index.
pub struct PreparedIndexLookup<V> {
    /// Returns the name of the secondary index table being queried.
    ///
    /// This is represents a `redb::Table` name that's used as an index. Each record in this table
    /// consists of a `I` secondary key and a `KeySet` (a collection of serialized primary keys).
    ///
    /// For example, an index table could be used to categorize all the different types creatures by
    /// `Habitat`, by `Diet`, etc.
    ///
    /// The `Habitat` index might contain an entry for `"Temperate Forest"` that references
    /// `"Raccoon"`, `"Skunk"`, and `"Deer"`, etc.
    pub index_name: &'static str,

    /// Indicates whether the index is `Unique` or `NonUnique`.
    pub index_kind: IndexKind,

    /// The key used for look-up the key-set from the index.
    ///
    /// For example, this secondary key could represent a `Habitat("Savannah")` and the values
    /// returned from the index table could be the primary keys to get a `"Lion"` and a `"Giraffe"`.
    ///
    /// # Notes
    ///
    /// * This key is in serialized form, as raw bytes.
    pub index_key_bytes: Vec<u8>,

    /// Used to track the type of the primary record `V` value.
    pub phantom_data: std::marker::PhantomData<V>,
}

impl<V> PreparedIndexLookup<V> {
    /// Instantiates a new `PreparedIndexLookup`.
    ///
    /// The `PreparedIndexLookup` tells `atlatl` how to look-up the primary records (`"Scorpion"`,
    /// `"Snail"`, `"Jerboa"`) from an index (`Habitat("Desert")`, `Habitat("Tide Pool")`).
    ///
    /// This structure tells `atlatl` that:
    ///
    /// * `index_name` · We want to look up creatures by `Habitat`. This is the name of the table
    ///   that contains the list of habitats and creatures.
    ///
    /// * `index_kind` · That `Habitat` is a `NonUnique` index, and that more than one species can
    ///   be associated with a single habitat.
    ///
    /// * `index_key_bytes` · And that we would like to see the critters listed in the
    ///   `Habitat("Rain Forest")` index.
    pub fn new(
        index_name: &'static str,
        index_kind: IndexKind,
        index_key_bytes: Vec<u8>,
    ) -> Self {
        Self {
            index_name,
            index_kind,
            index_key_bytes,
            phantom_data: std::marker::PhantomData::<V>,
        }
    }
}

impl<V: HasTable> IndexLookup for PreparedIndexLookup<V> {
    type Record = V;

    /// Returns the name of the secondary index table being queried.
    fn index_name(&self) -> &'static str {
        self.index_name
    }

    /// Returns whether the index is `Unique` or `NonUnique`.
    fn index_kind(&self) -> &IndexKind {
        &self.index_kind
    }

    /// Returns the serialized bytes of the secondary key. For example: `"Forest"`.
    fn index_key_bytes(&self) -> Result<Vec<u8>, crate::Error> {
        Ok(self.index_key_bytes.clone())
    }
}

















/// A trait for types that represent multiple index lookups under the same index.
///
/// This is used for `NOT IN`-style queries, where several secondary keys need to be resolved to
/// their corresponding primary keys. For example: exclude all animals with habitats in {`Forest`,
/// `Desert`, `Tundra`}.
///
/// Types like `Vec<I>` and `TinyVec<[I; N]>` can implement this as long as `I: IndexLookup`.
pub trait IndexMultiLookup {
    type Record: HasTable;

    /// Returns the name of the primary record table associated with this index.
    ///
    /// This is represents a `redb::Table` name that contains the records. Each record in this table
    /// consists of a `K` primary key and a `V` value.
    ///
    /// For example, a primary table could consist of all types creatures on Earth: of varying
    /// `Habitat`s, `Species`, `Diet`s, etc.
    fn table_name(&self) -> Option<&'static str>;

    /// Returns the name of the secondary index table being queried.
    ///
    /// This is represents a `redb::Table` name that's used as an index. Each record in this table
    /// consists of a `I` secondary key and a `KeySet` (a collection of serialized primary keys).
    ///
    /// For example, an index table could be used to categorize all the different types creatures by
    /// `Habitat`, by `Diet`, etc.
    ///
    /// The `Habitat` index might contain an entry for `"Temperate Forest"` that references
    /// `"Raccoon"`, `"Skunk"`, and `"Deer"`, etc.
    fn index_name(&self) -> Option<&'static str>;

    /// Returns whether the index is `Unique` or `NonUnique`.
    fn index_kind(&self) -> Option<&IndexKind>;

    /// Returns the set of keys to look-up from the secondary index table.
    ///
    /// This method is meant to allow for iterating over index-lookup contained in the collection
    /// that implements this trait.
    ///
    /// Rust doesn't allow iterators from traits (`-> impl Iterator`), and support for iterators
    /// from traits isn't great, at time of writing. However, we can iterate over a key-set returned
    /// from this `IndexMultiLookup` trait.
    ///
    /// This method might return index look-up instructions to look-up all animals for these keys:
    /// `Habitat("Forest")`, `Habitat("Savannah")`, and `Habitat("Desert")`
    ///
    /// The returned `KeySet` collection contains a list of secondary keys (serialized as raw bytes)
    /// associated with a given index entry. For example, it could be used to list all `creatures`
    /// that have a `Habitat` of `"Forest"`, `"Savannah"` and `"Desert"`.
    ///
    /// These are later used to fetch associated primary keys for exclusion. For example, if the
    /// caller wanted to exclude `Habitat`s of `"Forest"`, `"Savannah"` and `"Desert"`.
    fn to_key_set(&self) -> Result<KeySet, crate::Error>;
}

impl<I> IndexMultiLookup for I
where
    I: IndexLookup
{
    type Record = I::Record;

    /// Returns the name of the primary record table associated with this index.
    fn table_name(&self) -> Option<&'static str> {
        Some(self.table_name())
    }

    /// Returns the name of the secondary index table being queried.
    fn index_name(&self) -> Option<&'static str> {
        Some(self.index_name())
    }

    /// Returns whether the index is `Unique` or `NonUnique`.
    fn index_kind(&self) -> Option<&IndexKind> {
        Some(self.index_kind())
    }

    /// Returns the set of keys to look-up from the secondary index table.
    fn to_key_set(&self) -> Result<KeySet, Error> {
        let mut key_set = KeySet::with_capacity(1);
        key_set.insert(self.index_key_bytes()?);
        Ok(key_set)
    }
}

/// Implements `IndexMultiLookup` for `TinyVec` of `IndexLookup`s.
///
/// This is memory-efficient and ideal for small sets of values (≤ 4).
impl<I> IndexMultiLookup for tinyvec::TinyVec<[I; 4]>
where
    I: IndexLookup + Default
{
    type Record = I::Record;

    /// Returns the name of the primary record table associated with this index.
    fn table_name(&self) -> Option<&'static str> {
        self.first().map(|item| item.table_name())
    }

    /// Returns the name of the secondary index table being queried.
    fn index_name(&self) -> Option<&'static str> {
        self.first().map(|item| item.index_name())
    }

    /// Returns whether the index is `Unique` or `NonUnique`.
    fn index_kind(&self) -> Option<&IndexKind> {
        self.first().map(|item| item.index_kind())
    }

    /// Returns the set of keys to look-up from the secondary index table.
    fn to_key_set(&self) -> Result<KeySet, Error> {
        self.iter().map(|item| item.index_key_bytes()).collect()
    }
}

/// Implements `IndexMultiLookup` for `Vec` of `IndexLookup`s.
///
/// Use this when you're working with a dynamic or larger number of keys.
impl<I> IndexMultiLookup for Vec<I>
where
    I: IndexLookup,
{
    type Record = I::Record;

    /// Returns the name of the primary record table associated with this index.
    fn table_name(&self) -> Option<&'static str> {
        self.first().map(|item| item.table_name())
    }

    /// Returns the name of the secondary index table being queried.
    fn index_name(&self) -> Option<&'static str> {
        self.first().map(|item| item.index_name())
    }

    /// Returns whether the index is `Unique` or `NonUnique`.
    fn index_kind(&self) -> Option<&IndexKind> {
        self.first().map(|item| item.index_kind())
    }

    /// Returns the set of keys to look-up from the secondary index table.
    fn to_key_set(&self) -> Result<KeySet, Error> {
        self.iter().map(|item| item.index_key_bytes()).collect()
    }
}




















/*

pub trait ToPreparedIndexLookup<V: HasTable> {
    fn to_prepared_index_lookup(&self) -> Result<PreparedIndexLookup<V>, Error>;
}

impl<I, V: HasTable> ToPreparedIndexLookup<V> for I
where
    I: IndexLookup<Record = V>
{
    fn to_prepared_index_lookup(&self) -> Result<PreparedIndexLookup<V>, Error> {
        Ok(PreparedIndexLookup {
            index_name: self.index_name(),
            index_kind: *self.index_kind(),
            index_key_bytes: self.to_bytes()?,
            phantom_data: std::marker::PhantomData,
        })
    }
}
*/




























































#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum IndexKind {
    Unique = 0,
    NonUnique = 1,
}

/// Represents an encoded index key for a specific index table.
#[derive(Debug)]
pub struct IndexKey<'i, K: Codec<K>> {
    /// Returns the name of the secondary index table being queried.
    ///
    /// This is represents an `redb::Table` name that contains the secondary index. Each record is
    /// compried of a `I` secondary key and a `KeySet` (collection of serialized `K` primary keys).
    ///
    /// For example, an index table could be used to categorize all the different types creatures by
    /// `Habitat`, by `Diet`, etc.
    ///
    /// The `Habitat` index might contain an entry for `"Temperate Forest"` that references
    /// `"Raccoon"`, `"Skunk"`, and `"Deer"`, etc.
    index_name: &'static str,

    /// Indicates whether the index is `Unique` or `NonUnique`.
    index_kind: &'i IndexKind,

    /// The user-defined key value.
    secondary_key: &'i K,
}


pub trait Indexable<'i> {
    type Index: IndexLookup;
    type Indexes: IntoIterator<Item = Self::Index>;

    fn indexes(&'i self) -> Result<Self::Indexes, Error>;
}
































/*



pub trait Set {
    fn contains(&self, primary_key_bytes: &[u8]) -> bool;
    fn insert(&mut self, primary_key_bytes: Vec<u8>);
    fn remove(&mut self, primary_key_bytes: &[u8]);

    fn as_bytes(&self) -> &[u8]; // Cheap serialization for redb insert
    fn from_bytes(collection_bytes: &[u8]) -> Result<Self, Error> where Self: Sized;
}

#[derive(Debug)]
pub struct SetZerocopy<'i> {
    bytes: &'i [u8],
}

#[derive(Debug)]
pub struct Set {
    entries: HashSet<Vec<u8>>,
}

#[derive(Debug)]
pub enum SetImpl {
    Zerocopy(SetZerocopy<'static>),
    Owned(SetOwned),
}

impl Set for SetZerocopy<'i> {

}
*/



/*
pub trait Set {
    fn contains(&self, primary_key_bytes: &[u8]) -> bool;
    fn insert(&mut self, primary_key_bytes: Vec<u8>);
    fn remove(&mut self, primary_key_bytes: &[u8]);

    fn as_bytes(&self) -> &[u8]; // Cheap serialization for redb insert
    fn from_bytes(collection_bytes: &[u8]) -> Result<Self, Error> where Self: Sized;
}

*/




/*
pub trait Set {
    fn serialize(collection_bytes: Vec<u8>) ->
    fn contains(&self, secondary_key_bytes: &[u8]) -> bool;
    fn insert(&mut self, secondary_key_bytes: Vec<u8>);
    fn remove(&mut self, secondary_key_bytes: &[u8]);
}

impl Set for std::collections::BTreeSet<Vec<u8>> {

    fn contains(&self, secondary_key_bytes: &[u8]) -> bool {
        self.contains(secondary_key_bytes)
    }

    fn insert(&mut self, secondary_key_bytes: Vec<u8>) {
        self.insert(secondary_key_bytes);
    }

    fn remove(&mut self, secondary_key_bytes: &[u8]) {
        self.remove(secondary_key_bytes);
    }
}
/*
impl<S: std::hash::BuildHasher> Set for std::collections::HashSet<Vec<u8>, S> {
    fn contains(&self, secondary_key_bytes: &[u8]) -> bool {
        self.contains(secondary_key_bytes)
    }

    fn insert(&mut self, secondary_key_bytes: Vec<u8>) {
        self.insert(secondary_key_bytes);
    }

    fn remove(&mut self, secondary_key_bytes: &[u8]) {
        self.remove(secondary_key_bytes);
    }
}

impl Set for Vec<Vec<u8>> {
    fn contains(&self, secondary_key_bytes: &[u8]) -> bool {
        self.iter().any(|key| key == secondary_key_bytes)
    }

    fn insert(&mut self, secondary_key_bytes: Vec<u8>) {
        self.push(secondary_key_bytes);
    }

    fn remove(&mut self, secondary_key_bytes: &[u8]) {
        self.retain(|key| key != secondary_key_bytes);
    }
}
*/

























*/








pub struct IndexEntry<'pk, 'sk, K> {
    /// Returns the name of the secondary index table being queried.
    ///
    /// This is represents an `redb::Table` name that contains the secondary index. Each record is
    /// compried of a `I` secondary key and a `KeySet` (collection of serialized `K` primary keys).
    ///
    /// For example, an index table could be used to categorize all the different types creatures by
    /// `Habitat`, by `Diet`, etc.
    ///
    /// The `Habitat` index might contain an entry for `"Temperate Forest"` that references
    /// `"Raccoon"`, `"Skunk"`, and `"Deer"`, etc.
    pub index_name: &'static str,

    /// Indicates whether the index is `Unique` or `NonUnique`.
    pub index_kind: &'sk IndexKind,

    pub secondary_key: &'sk K,
    pub primary_key_bytes: &'pk [u8],
}

impl<'pk, 'sk, K> IndexEntry<'pk, 'sk, K>
where
    K: Codec<K> + IndexLookup
{
    pub const fn new(
        key: &'sk IndexKey<K>,
        primary_key_bytes: &'pk [u8],
    ) -> Result<Self, Error> {
        Ok(Self {
            index_name: key.index_name,
            index_kind: key.index_kind,
            secondary_key: key.secondary_key,
            primary_key_bytes,
        })
    }
}












// User Example stuff



use serde::{Deserialize, Serialize};



unsafe impl crate::layers::serializers::SafeForRmpSerde for Creature {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Creature {
    id: u64,
    species: String,
    habitat: String,
    diet: String,
}

impl HasTable for Creature {
    /// Returns the name of the primary record table associated with this index.
    fn table_name() -> &'static str { "creatures" }
}

impl HasPrimaryKey<'_, u64> for Creature {
    fn primary_key(&self) -> PrimaryKey<u64> {
        PrimaryKey::new(&self.id)
    }
}

pub struct Habitat(pub String);

impl IndexLookup for Habitat {
    type Record = Creature;

    /// Returns the name of the secondary index table being queried.
    fn index_name(&self) -> &'static str {
        "creatures_by_habitat"
    }

    /// Returns the kind of index: `Unique` or `NonUnique`.
    fn index_kind(&self) -> &IndexKind {
        &IndexKind::NonUnique
    }

    fn index_key_bytes(&self) -> Result<Vec<u8>, crate::Error> {
        Ok(Codec::<String>::serialize(&self.0)?)
    }
}

pub struct Species(pub String);

impl IndexLookup for Species {
    type Record = Creature;
    // type Field = String;

    /// Returns the name of the index table this key is associated with.
    fn index_name(&self) -> &'static str {
        "creatures_by_species"
    }

    /// Returns the kind of index: `Unique` or `NonUnique`.
    fn index_kind(&self) -> &IndexKind {
        &IndexKind::NonUnique
    }

    fn index_key_bytes(&self) -> Result<Vec<u8>, crate::Error> {
        Ok(Codec::<String>::serialize(&self.0)?)
    }
}