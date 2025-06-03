//! The typed module offers a flexible API for working with redb tables using any types that
//! implement [`crate::Codec`].
//!
//! It is ideal for use cases where key ordering is not required.
//!
//! Range queries and prefix scans are disabled by default and only available when the key type also
//! implements [`crate::codecs::OrderedWhenEncoded`].

mod table_mut;

pub use crate::typed::table_mut::TableMut;
pub use crate::typed::table_mut::RawTable;
pub use crate::typed::table_mut::OrderedTable as OrderedTableMut;

mod table_ref;

pub use crate::typed::table_ref::TableRef;
pub use crate::typed::table_ref::RawReadOnlyTable;
pub use crate::typed::table_ref::OrderedTable as OrderedTableRef;

pub mod database;
pub mod transaction;

// -------------------------------------------------------------------------------------------------
//
/// A type alias for the `redb::Range` iterator used for scanning key-value pairs.
///
/// Used internally by `Range` to decode entries using [`crate::Codec`] implementations.
pub type RedbRange<'r> = redb::Range<
    'r,             // Lifetime
    &'static [u8],  // Key
    &'static [u8]   // Value
>;

// -------------------------------------------------------------------------------------------------
//
/// A result type representing a decoded key-value pair from a table operation.
///
/// This is returned by most iterators and range scans, and encapsulates potential decoding
/// or storage-related failures.
pub type ResultEntry<K, V> = Result<(K, V), crate::Error>;