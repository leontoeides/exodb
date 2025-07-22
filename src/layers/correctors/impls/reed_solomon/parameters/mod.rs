//! Reed-Solomon ECC error correction layer parameters. This structure provides the information
//! necessary to process a Reed-Solomon error correction layers.

// Exports

pub mod error;
pub use crate::layers::correctors::impls::reed_solomon::parameters::error::Error;

// Imports

use crate::layers::core::Bytes;
use crate::layers::core::tail_readers::TailReaderBytes;

// -------------------------------------------------------------------------------------------------
//
/// Parameters for Reed-Solomon error correction and recovery.
///
/// Stores shard layout, data length, and checksums, appended to an encoded buffer. Used to
/// for error correction:
///
/// # Layer Structure
///
/// | `data`  | `parameters` |
/// |---------|--------------|
/// | `&[u8]` | `Parameters` |
///
/// # Parameters Structure
/// 
/// | `checksums` | `data_len` | `num_data_shards` | `total_num_shards` | `shard_size` |
/// |-------------|------------|-------------------|--------------------|--------------|
/// | `[u32; N]`  | `u32`      | `u32`             | `u32`              | `u32`        |
///
/// Parameters appended to the end of an encoded buffer for error correction and recovery.
///
/// This parameters is used during decoding to reconstruct the original data using Reed-Solomon error
/// correction. It includes shard sizing, data length, checksums for validation, and layout
/// parameters.
#[derive(Debug)]
pub struct Parameters {
    /// The fixed size of each shard in bytes.
    ///
    /// All data and parity shards are padded to this size. Required by the Reed-Solomon algorithm 
    /// to ensure even matrix dimensions.
    pub shard_size: usize,

    /// The total number of shards (data + parity).
    ///
    /// Used to decode the full shard layout. Also determines the number of checksums stored.
    pub total_num_shards: usize,

    /// The number of data shards (excluding parity).
    ///
    /// Only the first `num_data_shards` contain original application data. The remainder are parity 
    /// shards used for error recovery.
    pub num_data_shards: usize,

    /// The original length of the unpadded data block.
    ///
    /// This is used after decoding and reconstruction to truncate any padding and restore the exact 
    /// original payload.
    pub data_len: usize,

    /// CRC32 checksums for each shard (data + parity), in order.
    ///
    /// Used to detect corruption in individual shards before attempting reconstruction. Must 
    /// contain exactly `total_num_shards` entries.
    pub checksums: Vec<u32>,
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl Parameters {
    /// Instantiates a new `Parameters` structure using the given information.
    ///
    /// # Layer Structure
    ///
    /// | `data`  | `parameters` |
    /// |---------|--------------|
    /// | `&[u8]` | `Parameters` |
    ///
    /// # Serialized Parameters Structure
    ///
    /// | `checksums` | `data_len` | `num_data_shards` | `total_num_shards` | `shard_size` |
    /// |-------------|------------|-------------------|--------------------|--------------|
    /// | `[u32; N]`  | `u32`      | `u32`             | `u32`              | `u32`        |
    ///
    /// # Errors
    ///
    /// This method will return an error if:
    ///
    /// * Shard size is invalid
    /// * Total number of shards is zero
    /// * Number of data shards is zero or greater than the total number of shards, or
    /// * The number of checksums does not match the total number of shards
    pub fn new(
        shard_size: usize,
        total_num_shards: usize,
        num_data_shards: usize,
        data_len: usize,
        checksums: Vec<u32>,
    ) -> Result<Self, Error> {
        if shard_size == 0 {
            Err(Error::InvalidShardSize { shard_size })
        } else if total_num_shards < 2 {
            Err(Error::InvalidTotalShards {
                shard_count: total_num_shards,
                minimum_shards: 2
            })
        } else if num_data_shards == total_num_shards {
            Err(Error::NoPurityShards {
                data_shards: num_data_shards,
                total_shards: total_num_shards
            })
        } else if num_data_shards == 0 || num_data_shards > total_num_shards {
            Err(Error::InvalidDataShards {
                shard_count: num_data_shards,
                minimum_shards: 1,
                maximum_shards: total_num_shards
            })
        } else if data_len > shard_size * num_data_shards {
            Err(Error::DataLenTooLarge {
                data_len,
                max_capacity: shard_size * num_data_shards,
                shard_size,
                num_data_shards
            })
        } else if checksums.len() != total_num_shards {
            Err(Error::InvalidChecksumCount {
                found_checksums: checksums.len(),
                expected_checksums: total_num_shards
            })
        } else if shard_size.checked_mul(num_data_shards).is_none() {
            Err(Error::CapacityOverflow {
                shard_size,
                num_data_shards
            })
        } else {
            Ok(Self {
                shard_size,
                total_num_shards,
                num_data_shards,
                data_len,
                checksums,
            })
        }
    }

    /// Deserializes parameters from the end of a buffer.
    ///
    /// Reads in reverse order, from the end of the buffer toward to beginning of the buffer.
    ///
    /// # Layer Structure
    ///
    /// | `data`  | `parameters` |
    /// |---------|--------------|
    /// | `&[u8]` | `Parameters` |
    ///
    /// # Serialized Parameters Structure
    ///
    /// | `checksums` | `data_len` | `num_data_shards` | `total_num_shards` | `shard_size` |
    /// |-------------|------------|-------------------|--------------------|--------------|
    /// | `[u32; N]`  | `u32`      | `u32`             | `u32`              | `u32`        |
    ///
    /// # Errors
    ///
    /// This method will return an error if:
    ///
    /// * Shard size is invalid
    /// * Total number of shards is zero
    /// * Number of data shards is zero or greater than the total number of shards, or
    /// * The number of checksums does not match the total number of shards
    pub fn from_data_buffer(
        data: &mut Bytes<'_>
    ) -> Result<Self, Error> {
        let mut reader = TailReaderBytes::from_bytes(data);

        let shard_size: usize = reader.read_u32_le()
            .map_err(|error| Error::InsufficientData { parameter: "shard_size", error })?
            .try_into()
            .map_err(|_| Error::InvalidParameter("shard_size"))?;

        let total_num_shards: usize = reader.read_u32_le()
            .map_err(|error| Error::InsufficientData { parameter: "total_num_shards", error })?
            .try_into()
            .map_err(|_| Error::InvalidParameter("total_num_shards"))?;

        let num_data_shards: usize = reader.read_u32_le()
            .map_err(|error| Error::InsufficientData { parameter: "num_data_shards", error })?
            .try_into()
            .map_err(|_| Error::InvalidParameter("num_data_shards"))?;

        let data_len: usize = reader.read_u32_le()
            .map_err(|error| Error::InsufficientData { parameter: "data_len", error })?
            .try_into()
            .map_err(|_| Error::InvalidParameter("data_len"))?;

        let checksums: Vec<u32> = reader.read_u32_le_vec(total_num_shards)
            .map_err(|error| Error::InsufficientData { parameter: "checksums", error })?;

        reader.close();

        Self::new(shard_size, total_num_shards, num_data_shards, data_len, checksums)
    }

    /// Converts a `usize` integer that's used internally in-memory to a `u32` integer that's used
    /// for more compact storage.
    fn usize_to_u32(integer: usize, parameter: &'static str) -> Result<u32, Error> {
        u32::try_from(integer).map_err(|_error| Error::InvalidInteger {
            parameter,
            provided_value: integer,
            maximum_value: &u32::MAX
        })
    }

    /// Appends parameters to the buffer. Note: space for the parameters must have already been reserved
    /// in the data buffer when this method is called.
    ///
    /// # Layer Structure
    ///
    /// | `data`  | `parameters` |
    /// |---------|--------------|
    /// | `&[u8]` | `Parameters` |
    ///
    /// # Serialized Parameters Structure
    ///
    /// | `checksums` | `data_len` | `num_data_shards` | `total_num_shards` | `shard_size` |
    /// |-------------|------------|-------------------|--------------------|--------------|
    /// | `[u32; N]`  | `u32`      | `u32`             | `u32`              | `u32`        |
    pub fn into_data_buffer(self, data: &mut Vec<u8>) -> Result<(), Error> {
        self.checksums
            .into_iter()
            .for_each(|checksum| data.extend_from_slice(&checksum.to_le_bytes()));

        data.extend_from_slice(&Self::usize_to_u32(self.data_len, "data_len")?.to_le_bytes());
        data.extend_from_slice(&Self::usize_to_u32(self.num_data_shards, "num_data_shards")?.to_le_bytes());
        data.extend_from_slice(&Self::usize_to_u32(self.total_num_shards, "total_num_shards")?.to_le_bytes());
        data.extend_from_slice(&Self::usize_to_u32(self.shard_size, "shard_size")?.to_le_bytes());

        Ok(())
    }

    /// Verifies all shards against their checksums.
    ///
    /// * This method will return the shard index numbers of all shards that fail their checksum
    ///   verification in the returned `Vec`.
    ///
    ///   Start positions of corrupted shards can be calculated by `shard index num * shard size`.
    ///
    /// * If all shards pass their checksum validations, this method method will return an
    ///   empty `Vec`.
    pub fn check_shards(
        &self,
        data: &[u8],
    ) -> tinyvec::TinyVec::<[usize; 16]> {
        data
            .chunks(self.shard_size)
            .enumerate()
            .filter_map(|(index, shard)| self.checksums
                .get(index)
                .map_or(Some(index), |expected| {
                    let calculated = crc32fast::hash(shard);
                    if calculated == *expected {
                        None
                    } else {
                        tracing::error!(
                            "shard #{index} failed CRC-32 check, \
                            expected: {expected:#04x}, \
                            found: {calculated:#04x}"
                        );
                        Some(index)
                    }
                })
            )
            .collect()
    }

    /// Converts a `&[u8]` slice into a `Vec<Option<Vec<u8>>>`. This will also zero out any
    /// corrupted shards by replacing them with `None`.
    ///
    /// Prepares the shard layout for the Reed-Solomon `reconstruct` operation.
    ///
    /// Given the following input buffer of 10 bytes, zero padding of 2 bytes, a shard
    /// size of 3, 4 data shards, and 1 parity shard:
    ///
    /// ```nocheck
    /// &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 0, 10, 11, 12]
    /// ```
    ///
    /// We will end up with something like this: (not byte-accurate, for visual only)
    ///
    /// ```nocheck
    /// vec![
    ///      Some(vec![ 0,  1,  2]), // Data shard
    ///      None,                   // Corrupted data shard
    ///      Some(vec![ 6,  7,  8]), // Data shard
    ///      Some(vec![ 9,  0,  0]), // Data shard (with padding)
    ///      Some(vec![10, 11, 12])  // Parity shard
    /// ]
    /// ```
    pub fn prepare_shards(
        &self,
        data: &[u8],
        corrupted_shards: &[usize],
    ) -> Vec<Option<Vec<u8>>> {
        data
            .chunks(self.shard_size)
            .take(self.total_num_shards)
            .enumerate()
            .map(|(index, shard)| if corrupted_shards.contains(&index) {
                tracing::debug!("preparing shard #{index} for recovery");
                None
            } else {
                Some(shard.to_vec())
            })
            .collect()
    }

    /// Flattens the first `num_data_shards` from a `Vec<Option<Vec<u8>>>` into a single `Vec<u8>`.
    ///
    /// This method is used to reformat the data shards back into a `Vec<u8>` collection of bytes
    /// that's usable by the caller, after:
    ///
    /// * The data has been prepared for reconstruction, and
    /// * Reconstructed by the Reed-Solomon correction system.
    ///
    /// Ignores any parity shards beyond `num_data_shards`.
    ///
    /// # Errors
    ///
    /// This method will return an error if:
    ///
    /// * A shard is missing. This could be expected to happen if: the Reed-Solomon correction
    ///   failed because data is too corrupted and unrecoverable, or the data is not formatted as
    ///   expected.
    pub fn flatten_data_shards(
        &self,
        all_prepared_shards: Vec<Option<Vec<u8>>>,
    ) -> Result<Vec<u8>, Error> {
        let mut data = Vec::with_capacity(self.num_data_shards * self.shard_size);

        all_prepared_shards
            .into_iter()
            .take(self.num_data_shards)
            .enumerate()
            .try_for_each(|(index, shard_opt)|
                shard_opt.map_or(Err(Error::MissingShard { missing_shard: index }), |shard| {
                    data.extend_from_slice(&shard);
                    Ok(())
                })
            )?;

        data.truncate(self.data_len);

        Ok(data)
    }
}