//! Error returned from the `exodb` crate. This includes codec errors, storage errors, database
//! errors, and so on.

/// Error returned from the `exodb` crate. This includes codec errors, storage errors, database
/// errors, and so on.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("secondary index `{index}` already maps key to a different primary record")]
    IndexCollision {
        index: &'static str,
        key: Vec<u8>,
    },

    #[error("value not found for the given key in table `{table_name}`")]
    NotFound {
        table_name: String,
        key: Vec<u8>,
    },

    #[error("secondary index `{index}` refers to primary key that does not exist")]
    InvalidIndexReference {
        index: &'static str,
        key: Vec<u8>,
    },

    /// Failed to deserialise or deserialise a value using [`crate::Codec`].
    #[error(transparent)]
    Codec(#[from] crate::codecs::Error),

    /// [redb](https://www.redb.org/)
    /// [transaction error](https://docs.rs/redb/latest/redb/enum.CommitError.html).
    #[error(transparent)]
    RedbCommit(#[from] redb::CommitError),

    /// [redb](https://www.redb.org/)
    /// [database error](https://docs.rs/redb/latest/redb/enum.DatabaseError.html).
    #[error(transparent)]
    RedbDatabase(#[from] redb::DatabaseError),

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

    /// [rkyv](https://crates.io/crates/rkyv) rancor error.
    #[error(transparent)]
    RkyvRancor(#[from] rkyv::rancor::Error),

    /// An external error supplied by the caller.
    #[error("external error: {0}")]
    External(#[from] Box<dyn std::error::Error + Send + Sync + 'static>),
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl Error {
    /// Wraps a user-defined error in a boxed container for use with [`Error::External`].
    ///
    /// This provides an escape hatch for callers who wish to integrate their own custom error types
    /// into the unified [`crate::Error`] type, without requiring generic trait bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use exodb::Error;
    /// use std::io;
    ///
    /// fn validate_user(name: &str) -> Result<(), Error> {
    ///     if name.contains("💀") {
    ///         return Err(Error::wrap_external(io::Error::new(
    ///             io::ErrorKind::InvalidInput,
    ///             "User name contains forbidden rune",
    ///         )));
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// This is especially useful in contexts where indexing functions or database operations return
    /// `Result<T, exodb::Error>`, and you want to surface application-specific error conditions
    /// through the same error channel.
    ///
    /// # Errors
    ///
    /// Returns an [`Error::External`] variant containing the boxed user-defined error.
    pub fn wrap_external<E: std::error::Error + Send + Sync + 'static>(e: E) -> Self {
        Self::External(Box::new(e))
    }

    #[cfg(feature = "anyhow")]
    /// Wraps an [`anyhow::Error`] into an [`Error::External`] variant.
    ///
    /// This is useful when integrating with libraries or application logic that use [`anyhow`] as a
    /// general-purpose error type. It allows those errors to cleanly enter the [`crate::Error`]
    /// pipeline without disrupting fixed error signatures.
    ///
    /// # Errors
    ///
    /// * Returns [`Error::External`] containing the boxed `anyhow::Error`.
    #[must_use] pub fn wrap_anyhow(err: anyhow::Error) -> Self {
        Self::External(err.into_boxed_dyn_error())
    }

    /// Attaches additional context to any existing error variant.
    ///
    /// This is useful for adding human-readable detail to errors without altering their type,
    /// allowing you to trace failure points more clearly while preserving source error data.
    ///
    /// # Notes
    ///
    /// This attaches context to all error variants by wrapping them in [`Error::External`], even if
    /// they were originally internal.
    ///
    /// # Errors
    ///
    /// * Returns [`Error::External`] with attached context.
    #[must_use]
    pub fn with_context(self, context: impl Into<String>) -> Self {
        let ctx = context.into();
        let boxed: Box<dyn std::error::Error + Send + Sync + 'static> = match self {
            Self::External(inner) => {
                let msg = format!("{ctx}: {inner}");
                Box::new(std::io::Error::other(msg))
            },
            err => {
                let msg = format!("{ctx}: {err}");
                Box::new(std::io::Error::other(msg))
            },
        };

        Self::External(boxed)
    }
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

#[cfg(feature = "anyhow")]
impl From<anyhow::Error> for Error {
    fn from(err: anyhow::Error) -> Self {
        Self::External(err.into_boxed_dyn_error())
    }
}