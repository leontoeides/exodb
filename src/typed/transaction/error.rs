//! Error returned from the `atlatl` crate. This includes codec errors, storage errors, database
//! errors, and so on.

/// Error returned from the `atlatl` crate. This includes codec errors, storage errors, database
/// errors, and so on.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// [redb](https://www.redb.org/)
    /// [transaction error](https://docs.rs/redb/latest/redb/enum.CommitError.html).
    #[error(transparent)]
    RedbCommit(#[from] redb::CommitError),

    // [redb](https://www.redb.org/)
    // [database error](https://docs.rs/redb/latest/redb/enum.DatabaseError.html).
    // #[error(transparent)]
    // RedbDatabase(#[from] redb::DatabaseError),

    /// [redb](https://www.redb.org/)
    /// [savepoint error](https://docs.rs/redb/latest/redb/enum.SavepointError.html).
    #[error(transparent)]
    RedbSavepoint(#[from] redb::SavepointError),

    /// [redb](https://www.redb.org/)
    /// [storage error](https://docs.rs/redb/latest/redb/enum.StorageError.html).
    #[error(transparent)]
    RedbStorage(#[from] redb::StorageError),

    /// [redb](https://www.redb.org/)
    /// [table error](https://docs.rs/redb/latest/redb/enum.TableError.html).
    #[error(transparent)]
    RedbTable(#[from] redb::TableError),

    /// [redb](https://www.redb.org/)
    /// [transaction error](https://docs.rs/redb/latest/redb/enum.TransactionError.html).
    #[error(transparent)]
    RedbTransaction(#[from] Box<redb::TransactionError>),
}