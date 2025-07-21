//! Write transaction methods that are routed directly to `redb`.

// use crate::typed::transaction::Error;

// -------------------------------------------------------------------------------------------------
//
/// A wrapper around a `redb` write transaction.
///
/// A read/write transaction
///
/// Only a single write [`Transaction`] may exist at a time
pub struct Transaction(redb::WriteTransaction);
/*
// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl Transaction {
    /// Wraps a `redb` write transaction into an `atlatl` one.
    #[inline]
    #[must_use]
    pub fn new(redb: redb::WriteTransaction) -> Self {
        redb.into()
    }

    /// Creates a snapshot of the current database state, which can be used to rollback the
    /// database. This savepoint will exist until it is deleted with `[delete_savepoint()]`.
    ///
    /// Note that while a savepoint exists, pages that become unused after it was created are not
    /// freed. Therefore, the lifetime of a savepoint should be minimized.
    ///
    /// Returns `[SavepointError::InvalidSavepoint]`, if the transaction is “dirty” (any tables have
    /// been opened) or if the transaction’s durability is less than `[Durability::Immediate]`
    ///
    /// # Notes
    ///
    /// * This method call is passed-through to the `redb` Rust embedded database.
    #[cfg(feature = "redb-pass-through")]
    #[inline]
    pub fn persistent_savepoint(&self) -> Result<u64, Error> {
    	Ok(self.0.persistent_savepoint()?)
    }

    /// Get a persistent savepoint given its id
    ///
    /// # Notes
    ///
    /// * This method call is passed-through to the `redb` Rust embedded database.
    #[cfg(feature = "redb-pass-through")]
    #[inline]
    pub fn get_persistent_savepoint(
	    &self,
	    id: u64
	) -> Result<redb::Savepoint, Error> {
    	Ok(self.0.get_persistent_savepoint(id)?)
    }

    /// Delete the given persistent savepoint.
    ///
    /// Note that if the transaction is `abort()`’ed this deletion will be rolled back.
    ///
    /// Returns `true` if the savepoint existed Returns `[SavepointError::InvalidSavepoint]` if the
    /// transaction’s durability is less than `[Durability::Immediate]`
    ///
    /// # Notes
    ///
    /// * This method call is passed-through to the `redb` Rust embedded database.
    #[cfg(feature = "redb-pass-through")]
    #[inline]
	pub fn delete_persistent_savepoint(
	    &self,
	    id: u64,
	) -> Result<bool, Error> {
    	Ok(self.0.delete_persistent_savepoint(id)?)
    }

    /// List all persistent savepoints
    ///
    /// # Notes
    ///
    /// * This method call is passed-through to the `redb` Rust embedded database.
    #[cfg(feature = "redb-pass-through")]
    #[inline]
	pub fn list_persistent_savepoints(&self) -> Result<impl Iterator<Item = u64>, Error> {
		Ok(self.0.list_persistent_savepoints()?)
	}

    /// Creates a snapshot of the current database state, which can be used to rollback the database
    ///
    /// This savepoint will be freed as soon as the returned `[Savepoint]` is dropped.
    ///
    /// Returns `[SavepointError::InvalidSavepoint]`, if the transaction is “dirty” (any tables have
    /// been opened)
    ///
    /// # Notes
    ///
    /// * This method call is passed-through to the `redb` Rust embedded database.
    #[cfg(feature = "redb-pass-through")]
    #[inline]
	pub fn ephemeral_savepoint(&self) -> Result<redb::Savepoint, Error> {
		Ok(self.0.ephemeral_savepoint()?)
	}

	/// Restore the state of the database to the given
	/// [Savepoint](https://docs.rs/redb/latest/redb/struct.Savepoint.html)
	///
	/// Calling this method invalidates all
	/// [Savepoint](https://docs.rs/redb/latest/redb/struct.Savepoint.html)s created after savepoint
    ///
    /// # Notes
    ///
    /// * This method call is passed-through to the `redb` Rust embedded database.
    #[cfg(feature = "redb-pass-through")]
    #[inline]
	pub fn restore_savepoint(
	    &mut self,
	    savepoint: &redb::Savepoint
	) -> Result<(), Error> {
		Ok(self.0.restore_savepoint(savepoint)?)
	}

	/// Set the desired durability level for writes made in this transaction Defaults to
	/// [Durability::Immediate](https://docs.rs/redb/latest/redb/enum.Durability.html#variant.Immediate)
	///
	/// Will panic if the durability is reduced below [Durability::Immediate] after a persistent
	/// savepoint has been created or deleted.
    ///
    /// # Notes
    ///
    /// * This method call is passed-through to the `redb` Rust embedded database.
    #[cfg(feature = "redb-pass-through")]
    #[inline]
	pub fn set_durability(&mut self, durability: redb::Durability) {
		self.0.set_durability(durability)
	}


    /// Enable or disable 2-phase commit (defaults to disabled)
    ///
    /// By default, data is written using the following 1-phase commit algorithm:
    ///
    /// 1. Update the inactive commit slot with the new database state
    /// 2. Flip the god byte primary bit to activate the newly updated commit slot
    /// 3. Call `fsync` to ensure all writes have been persisted to disk
    ///
    /// All data is written with checksums. When opening the database after a crash, the most recent
    /// of the two commit slots with a valid checksum is used.
    ///
    /// Security considerations: The checksum used is xxhash, a fast, non-cryptographic hash
    /// function with close to perfect collision resistance when used with non-malicious input. An
    /// attacker with an extremely high degree of control over the database's workload, including
    /// the ability to cause the database process to crash, can cause invalid data to be written
    /// with a valid checksum, leaving the database in an invalid, attacker-controlled state.
    ///
    /// Alternatively, you can enable 2-phase commit, which writes data like this:
    ///
    /// 1. Update the inactive commit slot with the new database state
    /// 2. Call `fsync` to ensure the database slate and commit slot update have been persisted
    /// 3. Flip the god byte primary bit to activate the newly updated commit slot
    /// 4. Call `fsync` to ensure the write to the god byte has been persisted
    ///
    /// This mitigates a theoretical attack where an attacker who
    /// 1. can control the order in which pages are flushed to disk
    /// 2. can introduce crashes during `fsync`,
    /// 3. has knowledge of the database file contents, and
    /// 4. can include arbitrary data in a write transaction
    ///
    /// could cause a transaction to partially commit (some but not all of the data is written).
    /// This is described in the design doc in futher detail.
    ///
    /// Security considerations: Many hard disk drives and SSDs do not actually guarantee that data
    /// has been persisted to disk after calling `fsync`. Even with 2-phase commit, an attacker with
    /// a high degree of control over the database's workload, including the ability to cause the
    /// database process to crash, can cause the database to crash with the god byte primary bit
    /// pointing to an invalid commit slot, leaving the database in an invalid, potentially
    /// attacker-controlled state.
    ///
    /// # Notes
    ///
    /// * This method call is passed-through to the `redb` Rust embedded database.
    #[cfg(feature = "redb-pass-through")]
    #[inline]
    pub fn set_two_phase_commit(&mut self, enabled: bool) {
        self.0.set_two_phase_commit(enabled)
    }

    /// Enable or disable quick-repair (defaults to disabled)
    ///
    /// By default, when reopening the database after a crash, redb needs to do a full repair. This
    /// involves walking the entire database to verify the checksums and reconstruct the allocator
    /// state, so it can be very slow if the database is large.
    ///
    /// Alternatively, you can enable quick-repair. In this mode, redb saves the allocator state
    /// as part of each commit (so it doesn't need to be reconstructed), and enables 2-phase commit
    /// (which guarantees that the primary commit slot is valid without needing to look at the
    /// checksums). This means commits are slower, but recovery after a crash is almost instant.
    ///
    /// # Notes
    ///
    /// * This method call is passed-through to the `redb` Rust embedded database.
    #[cfg(feature = "redb-pass-through")]
    #[inline]
    pub fn set_quick_repair(&mut self, enabled: bool) {
        self.0.set_quick_repair(enabled)
    }

    /// Open the given table
    ///
    /// The table will be created if it does not exist
    ///
    /// # Notes
    ///
    /// * This method call is passed-through to the `redb` Rust embedded database.
    #[cfg(feature = "redb-pass-through")]
    #[inline]
    pub fn open_redb_table<'txn, K, V>(
        &'txn self,
        definition: redb::TableDefinition<K, V>,
    ) -> Result<redb::Table<'txn, K, V>, Error>
    where
    	K: redb::Key + 'static,
    	V: redb::Value + 'static
    {
        Ok(self.0.open_table(definition)?)
    }

    /// Open the given table
    ///
    /// The table will be created if it does not exist
    ///
    /// # Notes
    ///
    /// * This method call is passed-through to the `redb` Rust embedded database.
    #[cfg(feature = "redb-pass-through")]
    #[inline]
    pub fn open_multimap_table<'txn, K, V>(
        &'txn self,
        definition: redb::MultimapTableDefinition<K, V>
    ) -> Result<redb::MultimapTable<'txn, K, V>, Error>
    where
    	K: redb::Key + 'static,
    	V: redb::Key + 'static
    {
        Ok(self.0.open_multimap_table(definition)?)
    }

    /// Rename the given table
    ///
    /// # Notes
    ///
    /// * This method call is passed-through to the `redb` Rust embedded database.
    #[cfg(feature = "redb-pass-through")]
    #[inline]
    pub fn rename_table(
        &self,
        definition: impl redb::TableHandle,
        new_name: impl redb::TableHandle
    ) -> Result<(), Error> {
    	Ok(self.0.rename_table(definition, new_name)?)
    }

    /// Rename the given multimap table
    ///
    /// # Notes
    ///
    /// * This method call is passed-through to the `redb` Rust embedded database.
    #[cfg(feature = "redb-pass-through")]
    #[inline]
	pub fn rename_multimap_table(
	    &self,
	    definition: impl redb::MultimapTableHandle,
	    new_name: impl redb::MultimapTableHandle
	) -> Result<(), Error> {
		Ok(self.0.rename_multimap_table(definition, new_name)?)
	}

    /// Delete the given table
    ///
	/// Returns a bool indicating whether the table existed
    ///
    /// # Notes
    ///
    /// * This method call is passed-through to the `redb` Rust embedded database.
    #[cfg(feature = "redb-pass-through")]
    #[inline]
	pub fn delete_table(
	    &self,
	    definition: impl redb::TableHandle
	) -> Result<bool, Error> {
		Ok(self.0.delete_table(definition)?)
	}

    /// Delete the given table
    ///
	/// Returns a bool indicating whether the table existed
    ///
    /// # Notes
    ///
    /// * This method call is passed-through to the `redb` Rust embedded database.
    #[cfg(feature = "redb-pass-through")]
    #[inline]
	pub fn delete_multimap_table(
	    &self,
	    definition: impl redb::MultimapTableHandle
	) -> Result<bool, Error> {
		Ok(self.0.delete_multimap_table(definition)?)
	}

    /// List all the tables
    ///
    /// # Notes
    ///
    /// * This method call is passed-through to the `redb` Rust embedded database.
    #[cfg(feature = "redb-pass-through")]
    #[inline]
	pub fn list_tables(
	    &self
	) -> Result<impl Iterator<Item = redb::UntypedTableHandle> + '_, Error> {
		Ok(self.0.list_tables()?)
	}

    /// List all the multimap tables
    ///
    /// # Notes
    ///
    /// * This method call is passed-through to the `redb` Rust embedded database.
    #[cfg(feature = "redb-pass-through")]
    #[inline]
	pub fn list_multimap_tables(
	    &self
	) -> Result<impl Iterator<Item = redb::UntypedMultimapTableHandle> + '_, Error> {
		Ok(self.0.list_multimap_tables()?)
	}

    /// Commit the transaction
    ///
    /// All writes performed in this transaction will be visible to future transactions, and are
    /// durable as consistent with the [Durability](https://docs.rs/redb/latest/redb/enum.Durability.html)
    /// level set by [Self::set_durability](https://docs.rs/redb/latest/redb/struct.WriteTransaction.html#method.set_durability)
    ///
    /// # Notes
    ///
    /// * This method call is passed-through to the `redb` Rust embedded database.
    #[inline]
	pub fn commit(self) -> Result<(), Error> {
		Ok(self.0.commit()?)
	}

    /// Abort the transaction
    ///
	/// All writes performed in this transaction will be rolled back
    ///
    /// # Notes
    ///
    /// * This method call is passed-through to the `redb` Rust embedded database.
    #[inline]
	pub fn abort(self) -> Result<(), Error> {
		Ok(self.0.abort()?)
	}

    /// Retrieves information about storage usage in the database
    ///
    /// # Notes
    ///
    /// * This method call is passed-through to the `redb` Rust embedded database.
    #[cfg(feature = "redb-pass-through")]
    #[inline]
	pub fn stats(self) -> Result<redb::DatabaseStats, Error> {
		Ok(self.0.stats()?)
	}
}

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl From<redb::WriteTransaction> for Transaction {
    /// Converts a `redb` write transaction into an `atlatl` write transaction.
    fn from(redb: redb::WriteTransaction) -> Self {
        Self(redb)
    }
} */