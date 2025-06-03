mod key_set;


pub use crate::indexing::key_set::{ArchivedKeySet, KeySet};




















use crate::{Codec, Error};

// exodb stuff

pub trait HasTable {
    /// Returns the table name where values of this type are stored.
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
        Ok(PK::encode(self.0)?)
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
pub trait IndexableKey {
    /// Type of the parent record.
    type Record: HasTable;

    /// Type of the child field.
    type Field: Codec<Self::Field>;

    /// Returns the table name where the record is stored.
    #[must_use] fn table_name() -> &'static str { Self::Record::table_name() }

    /// Returns the table name where the field index is store.
    #[must_use] fn index_name(&self) -> &'static str;

    /// Returns the kind of index: `Unique` or `NonUnique`.
    #[must_use] fn index_kind(&self) -> &IndexKind;

    /// Encodes the secondary key into bytes for lookup.
    ///
    /// # Errors
    ///
    /// Consult the documentation of the codec backend you are using for more detail on
    /// serialization behavior and potential limitations.
    fn to_bytes(&self) -> Result<Vec<u8>, crate::Error>;
}


#[derive(Debug)]
pub enum IndexKind {
    Unique,
    NonUnique,
}

/// Represents an encoded index key for a specific index table.
#[derive(Debug)]
pub struct IndexKey<'i, K: Codec<K>> {
    /// The name of the index table this key belongs to.
    index_name: &'static str,

    index_kind: &'i IndexKind,

    /// The user-defined key value.
    secondary_key: &'i K,
}


pub trait Indexable<'i> {
    type Index: IndexableKey;
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
    fn encode(collection_bytes: Vec<u8>) ->
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
    pub index_name: &'static str,
    pub index_kind: &'sk IndexKind,
    pub secondary_key: &'sk K,
    pub primary_key_bytes: &'pk [u8],
}

impl<'pk, 'sk, K> IndexEntry<'pk, 'sk, K>
where
    K: Codec<K> + IndexableKey
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
use chrono::NaiveDate;



unsafe impl crate::codecs::SafeForRmpSerde for User {}
unsafe impl crate::codecs::SafeForRmpSerde for chrono::NaiveDate {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    id: u64,
    name: String,
    sex: Sex,
    birth_date: NaiveDate,
    password: Option<String>,
    notes: String,
}

unsafe impl crate::codecs::SafeForRmpSerde for Sex {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Sex {
    Female,
    Male
}

impl HasTable for User {
    fn table_name() -> &'static str { "users" }
}

impl HasPrimaryKey<'_, u64> for User {
    fn primary_key(&self) -> PrimaryKey<u64> {
        PrimaryKey::new(&self.id)
    }
}

pub struct Birthday(pub NaiveDate);

impl IndexableKey for Birthday {
    type Record = User;
    type Field = NaiveDate;

    /// Returns the name of the index table this key is associated with.
    fn index_name(&self) -> &'static str {
        "user_by_birth_date"
    }

    /// Returns the kind of index: `Unique` or `NonUnique`.
    fn index_kind(&self) -> &IndexKind {
        &IndexKind::NonUnique
    }

    fn to_bytes(&self) -> Result<Vec<u8>, crate::Error> {
        Ok(Codec::<NaiveDate>::encode(&self.0)?)
    }
}

pub enum UserIndex<'i> {
    Name(IndexKey<'i, String>),
    Birthday(IndexKey<'i, NaiveDate>),
}
