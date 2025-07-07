//! Provides errors for error-correction layer parameters.

// -------------------------------------------------------------------------------------------------
//
/// Represents an error from the error-correction layer's parameters module. Parameters provide the
/// information necessary to process error-correction layers.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Not enough data for parameter.
    ///
    /// Expected a certain number of bytes for a parameter but not enough data was available.
    /// Indicates corrupted data or an incomplete buffer.
    #[error("not enough data to read error-correction layer parameter {parameter:?}")]
    InsufficientData {
        parameter: &'static str,
        error: crate::layers::tail_reader::Error,
    },

    /// Invalid parameter.
    ///
    /// Failed to parse parameter from the buffer. This may indicate corrupted data in the database
    /// or may also indicate cross-platform integer-size incompatibilities.
    #[error("invalid data for error-correction layer parameter {0:?}")]
    InvalidParameter(&'static str),

    /// Invalid shard size.
    ///
    /// Shard size must be non-zero for Reed-Solomon error correction. A shard size of `0` is
    /// invalid and prevents sharding.
    #[error("invalid shard size: {shard_size} bytes, must be greater than zero")]
    InvalidShardSize {
        shard_size: usize
    },

    /// Invalid total shard count.
    ///
    /// Total number of shards (data + parity) must be non-zero for Reed-Solomon error correction.
    #[error(
        "invalid total shard count of {shard_count}, \
        minimum of {minimum_shards} shards required"
    )]
    InvalidTotalShards {
        shard_count: usize,
        minimum_shards: usize,
    },

    /// Invalid data shard count.
    ///
    /// Number of data shards must be non-zero and not exceed total shards.
    #[error(
        "invalid data shard count of {shard_count}, \
        must be between {minimum_shards} to {maximum_shards} (total number of shards)"
    )]
    InvalidDataShards {
        shard_count: usize,
        minimum_shards: usize,
        maximum_shards: usize
    },

    /// Invalid checksum count.
    ///
    /// Number of checksums must equal total shards.
    #[error(
        "invalid checksum count: found {found_checksums} checksums, \
        expected {expected_checksums}
    ")]
    InvalidChecksumCount {
        found_checksums: usize,
        expected_checksums: usize
    },

    /// Missing shard after Reed-Solomon reconstruction. This data has been damaged beyond recovery.
    ///
    /// Shard could not be reconstructed during Reed-Solomon decoding. This indicates that the
    /// Reed-Solomon correction failed because data is too corrupted and it's unrecoverable.
    #[error("missing shard #{missing_shard} after reconstruction")]
    MissingShard { missing_shard: usize },
}