// Re-exports for [Christopher Berner](https://github.com/cberner)'s
// [redb](https://crates.io/crates/redb) crate.

// mod databases;
// mod tables;
mod transactions;

// -------------------------------------------------------------------------------------------------
//
// These re-exports are provided so that the correct `redb` types are always available to the host
// crate without having to explicitly define it as a dependency in the `Cargo.toml` file.

pub use redb::AccessGuard;
pub use redb::AccessGuardMut;
pub use redb::Builder;
pub use redb::CacheStats;
pub use redb::CommitError;
pub use redb::CompactionError;
pub use redb::Database;
pub use redb::DatabaseError;
pub use redb::DatabaseStats;
pub use redb::Durability;
pub use redb::Error;
pub use redb::ExtractIf;
pub use redb::Key;
pub use redb::MultimapRange;
pub use redb::MultimapTable;
pub use redb::MultimapTableDefinition;
pub use redb::MultimapTableHandle;
pub use redb::MultimapValue;
pub use redb::MutInPlaceValue;
pub use redb::Range;
pub use redb::ReadableMultimapTable;
pub use redb::ReadableTable;
pub use redb::ReadableTableMetadata;
pub use redb::ReadOnlyMultimapTable;
pub use redb::ReadOnlyTable;
pub use redb::ReadOnlyUntypedMultimapTable;
pub use redb::ReadOnlyUntypedTable;
pub use redb::ReadTransaction;
pub use redb::RepairSession;
pub use redb::Result;
pub use redb::Savepoint;
pub use redb::SavepointError;
pub use redb::StorageBackend;
pub use redb::StorageError;
pub use redb::Table;
pub use redb::TableDefinition;
pub use redb::TableError;
pub use redb::TableHandle;
pub use redb::TableStats;
pub use redb::TransactionError;
pub use redb::TypeName;
pub use redb::UntypedMultimapTableHandle;
pub use redb::UntypedTableHandle;
pub use redb::UpgradeError;
pub use redb::Value;
pub use redb::WriteTransaction;