//! Contains the error type returned from the error-correction layer's parameters module.

// -------------------------------------------------------------------------------------------------
//
/// Represents an error from the error-correction layer's parameters module. Parameters provide the
/// information necessary to process error-correction layers.
#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum Error {
    /// Not enough data for parameter.
    ///
    /// Expected a certain number of bytes for a parameter but not enough data was available. This
    /// indicates that the data has likely been corrupted.
    #[error("not enough data to read error-correction layer parameter {parameter:?}")]
    InsufficientData {
        parameter: &'static str,
        error: crate::layers::core::tail_readers::Error,
    },

    /// Invalid parameter.
    ///
    /// Failed to parse parameter from the buffer. This may indicate corrupted data in the database
    /// or may also indicate cross-platform integer-size incompatibilities.
    #[error("invalid data for error-correction layer parameter {0:?}")]
    InvalidParameter(&'static str),

    /// Reed-Solomon parameter too large. Maximum size is `u32::MAX`.
    ///
    /// * This could happen due to enormous values. The theoretical maximum is 65,535 bytes (maximum
    ///   shard size) × 4,294,967,295 (`u32::MAX` shards) = *281.4 terabytes*.
    ///
    /// * If you're receiving this error it's likely due to data corruption.
    #[error(
        "integer too large for {parameter:?}: given an value of {provided_value:?} \
        but the maximum value is {maximum_value:?}"
    )]
    InvalidInteger {
        parameter: &'static str,
        provided_value: usize,
        maximum_value: &'static u32,
    },

    /// Invalid shard size.
    ///
    /// Shard size must be non-zero for Reed-Solomon error correction. A shard size of `0` is
    /// invalid and prevents sharding. This indicates that the data has likely been corrupted.
    #[error("invalid shard size: {shard_size} bytes, must be greater than zero")]
    InvalidShardSize {
        shard_size: usize
    },

    /// Invalid total shard count.
    ///
    /// Total number of shards (data + parity) must be non-zero for Reed-Solomon error correction.
    /// This indicates that the data has likely been corrupted.
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
    /// Number of data shards must be non-zero and not exceed total shards. This indicates that the
    /// data has likely been corrupted.
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
    /// Number of checksums must equal total shards. This indicates that the data has likely been
    /// corrupted.
    #[error(
        "invalid checksum count: found {found_checksums} checksums, \
        expected {expected_checksums}"
    )]
    InvalidChecksumCount {
        found_checksums: usize,
        expected_checksums: usize
    },

    /// Data length exceeds shard capacity.
    ///
    /// The provided data length is larger than what can fit in the available data shards.
    /// This indicates that the data has likely been corrupted or parameters are misconfigured.
    #[error(
        "data length {data_len} bytes exceeds maximum capacity of {max_capacity} bytes \
        ({shard_size} bytes per shard × {num_data_shards} data shards)"
    )]
    DataLenTooLarge {
        data_len: usize,
        max_capacity: usize,
        shard_size: usize,
        num_data_shards: usize,
    },

    /// No parity shards available for error correction.
    ///
    /// Reed-Solomon error correction requires at least one parity shard. When the number of
    /// data shards equals the total number of shards, no error correction is possible.
    /// This indicates that the data has likely been corrupted.
    #[error(
        "no parity shards available: {data_shards} data shards out of {total_shards} total shards \
        (need at least 1 parity shard for error correction)"
    )]
    NoPurityShards {
        data_shards: usize,
        total_shards: usize,
    },

    /// Shard capacity calculation overflow.
    ///
    /// The multiplication of shard size and number of data shards would overflow. This indicates
    /// that the parameters are too large or the data has been corrupted.
    #[error(
        "shard capacity calculation overflow: {shard_size} bytes × {num_data_shards} shards \
        exceeds maximum representable value"
    )]
    CapacityOverflow {
        shard_size: usize,
        num_data_shards: usize,
    },

    /// Missing shard after Reed-Solomon reconstruction. This data has been damaged beyond recovery.
    ///
    /// Shard could not be reconstructed during Reed-Solomon decoding. This indicates that the
    /// Reed-Solomon correction failed because data is too corrupted and it's unrecoverable.
    #[error("missing shard #{missing_shard} after reconstruction")]
    MissingShard { missing_shard: usize },
}