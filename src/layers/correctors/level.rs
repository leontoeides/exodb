//! An enumeration that allows the protection-level for each type to be set individually.

// -------------------------------------------------------------------------------------------------
//
/// Error correction protection levels.
///
/// While error correction can be used with smaller types (< 512 bytes), the protection offered
/// might be overkill. This setting allows configuring protection levels based on data importance
/// and storage constraints.
#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub enum Level {
    /// Minimal protection against single shard corruption.
    ///
    /// * Parity shards: 1
    /// * Storage overhead: ~6-12% (depends on data size)
    /// * Recoverable errors: 1 shard
    #[default]
    Basic,

    /// Moderate protection using data_shards >> 2 parity shards.
    ///
    /// * Parity shards: data_shards / 4
    /// * Storage overhead: ~25%
    /// * Recoverable errors: Up to 25% of shards
    Standard,

    /// High protection using data_shards >> 1 parity shards.
    ///
    /// * Parity shards: data_shards / 2
    /// * Storage overhead: ~50%
    /// * Recoverable errors: Up to 50% of shards
    Maximum,

    /// Custom protection with an exact number of parity shards.
    ///
    /// * Parity shards: As specified
    /// * Storage overhead: Varies
    /// * Recoverable errors: Up to the specified number of shards
    Exact(usize),
}