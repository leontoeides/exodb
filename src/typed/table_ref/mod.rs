//! A typed wrapper around a read-only `redb` table for a specific key/value type pair.

mod ordered_table;
mod range;
mod redb;

pub use crate::typed::table_ref::ordered_table::OrderedTable;

use crate::{Codec, Error, typed::table_ref::range::Range};
use ::redb::TableHandle;

// -------------------------------------------------------------------------------------------------
//
// Type Aliases

/// A type alias for a low-level read-only `redb` table with raw byte slice keys and values.
///
/// This is the untyped foundation beneath [`TableRef`], used for decoding data within read-only
/// transactions.
pub type RawReadOnlyTable = ::redb::ReadOnlyTable<&'static [u8], &'static [u8]>;

// -------------------------------------------------------------------------------------------------
//
/// A typed wrapper around a read-only `redb` table for a specific key/value type pair.
///
/// The key and value types must implement the `Codec` trait to handle serialization and
/// deserialization. This allows for pluggable, safe, and ergonomic interaction with `redb` using
/// your own data types.
///
/// # Errors
///
/// All methods may return a [`crate::Error`] variant originating from underlying storage issues.
///
/// | Variant         | Cause                                             | Resolution                            |
/// |-----------------|---------------------------------------------------|---------------------------------------|
/// | `Corrupted`     | Internal table structure was damaged or invalid   | Backup and restore or recreate table  |
/// | `ValueTooLarge` | Attempted to store a value that exceeded max size | Consider breaking into smaller values |
/// | `Io`            | I/O failure (disk error, permission issue), etc.  | Check file permissions and storage    |
/// | `PreviousIo`    | Prior I/O failure poisoned the database           | Reopen or recover the environment     |
/// | `LockPoisoned`  | A panic occurred while holding a database lock    | Restart process or retry operation    |
#[derive(Debug)]
pub struct TableRef<K, V>
where
    K: Codec<K>,
    V: Codec<V>
{
    redb_table: RawReadOnlyTable,
    _phantom: std::marker::PhantomData<(K, V)>,
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl<K, V> TableRef<K, V>
where
    K: Codec<K>,
    V: Codec<V>
{
    /// Creates a new [`TableRef`] wrapper around a raw `redb` table with byte slice keys and
    /// values.
    ///
    /// This method initializes the typed interface for inserting, retrieving, and modifying
    /// structured data using [`Codec`] implementations for the key and value types.
    ///
    /// Internally, this wraps the raw `redb` table into a type-safe exoskeleton.
    ///
    /// # Notes
    ///
    /// * This method call is passed-through to the `redb` Rust embedded database.
    #[inline]
    #[must_use]
    pub fn new(table: ::redb::ReadOnlyTable<&[u8], &[u8]>) -> Self {
        table.into()
    }


    pub(crate) fn get_by_key_bytes(
        &self,
        key_bytes: &[u8],
    ) -> Result<V, Error> {
        self.redb_table
            .get(key_bytes)
            .map_err(Error::from)
            .and_then(|result| result
                .ok_or_else(|| Error::NotFound {
                    table_name: self.redb_table.name().to_string(),
                    key: key_bytes.to_vec(),
                })
                .and_then(|serialized| V::deserialize(serialized.value()).map_err(Error::from))
            )
    }




    pub(crate) fn get_many_by_key_bytes(
        &self,
        keys: impl Iterator<Item = Vec<u8>>,
    ) -> impl Iterator<Item = Result<V, Error>> {
        keys
            .map(|key_bytes| {
                self.redb_table
                    .get(key_bytes.as_slice())
                    .map_err(Error::from)
                    .and_then(|result| result
                        .ok_or_else(|| Error::NotFound {
                            table_name: self.redb_table.name().to_string(),
                            key: key_bytes.clone(),
                        })
                        .and_then(|serialized| V::deserialize(serialized.value()).map_err(Error::from))
                    )
            })
    }
}