use crate::layers::core::Bytes;
use crate::layers::correctors::impls::reed_solomon::{DATA_LEN_MIN, DATA_LEN_MAX, Error, Parameters};
use crate::layers::correctors::{Correctable, Level};
use reed_solomon_erasure::Field;

// -------------------------------------------------------------------------------------------------
//
/// Reed-Solomon encoder using [Darren Li](https://github.com/darrenldl), [Michael
/// Vines](https://github.com/mvines), and [Nazar Mokrynskyi](https://github.com/nazar-pc)'s
/// [reed-solomon-erasure](https://crates.io/crates/reed-solomon-erasure) crate.
///
/// Implements Reed-Solomon error correction using Galois Fields. This is a technique first
/// developed in 1960 by Irving S. Reed and Gustave Solomon for reliable data transmission. By
/// operating over finite fields (courtesy of Évariste Galois, 1830), this method protects data
/// against shard loss or corruption with mathematically guaranteed recoverability.
#[allow(clippy::too_long_first_doc_paragraph, reason = "it's complete and concise")]
pub struct ReedSolomon<V> {
    /// A marker to tie this `ReedSolomon` structure to a specific type `V` without storing any
    /// actual data.
    phantom_data: std::marker::PhantomData<V>,
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl<V: Correctable> ReedSolomon<V> {
    #[inline]
    #[must_use]
    pub fn is_protectable(data: &Bytes<'_>) -> bool {
        // Remember the size of the data block before padding, parity data, or the data block
        // length is added:
        let data_len = data.len();

        // Only attempt to encode data buffers if they meet the size constraints:
        if data_len < DATA_LEN_MIN {
            tracing::debug!(
                "data will not be protected, \
                data size of {data_len} bytes is too small"
            );
            false
        } else if data_len > DATA_LEN_MAX {
            tracing::debug!(
                "data will not be protected, \
                data size of {data_len} bytes is too large"
            );
            false
        } else {
            true
        }
    }

    /// Encodes Reed-Solomon parity shards in-place, appending them to the data buffer.
    ///
    /// # Data Structure
    ///
    /// | Data Block             | Parity Block           | Parameters Block                        |
    /// |------------------------|------------------------|---------------------------------------|
    /// | `[u8; N * shard size]` | `[u8; N * shard size]` | `[u32; (4 + total number of shards)]` |
    ///
    /// # Errors
    ///
    /// * If encoding fails due to an internal error.
    pub fn add_parity(data: Bytes<'_>) -> Result<Bytes<'_>, Error> {
        // Remember the size of the data block before padding, parity data, or the data block
        // length is added:
        let data_len = data.len();

        // Only attempt to encode data buffers if they meet the size constraints:
        if data_len < DATA_LEN_MIN {
            tracing::debug!(
                "data will not be protected, \
                data size of {data_len} bytes is too small"
            );
            Ok(data)
        } else if data_len > DATA_LEN_MAX {
            tracing::debug!(
                "data will not be protected, \
                data size of {data_len} bytes is too large"
            );
            Ok(data)
        } else {
            // Make several measurements based on the data buffer's length:
            let (
                shard_size,           // System determined size for shard.
                num_data_shards,      // Number of shards required to encompass the data buffer.
                total_num_shards,     // Total shards to encompass data and parity blocks.
                data_and_parity_size, // Total size of data & parity blocks in bytes.
                total_size            // Total size of data, parity, & metadata blocks in bytes.
            ) = Self::buffer_measurements(data_len);

            // Initialize the Reed-Solomon engine. This is what does the math to create and
            // repair shards using Galois Fields.
            let reed_solomon =
                reed_solomon_erasure::ReedSolomon::<reed_solomon_erasure::galois_8::Field>::new(
                    num_data_shards,
                    total_num_shards - num_data_shards
                )?;

            // Pad the data block, append space for parity data and metadata.
            //
            // Before:
            // [data buffer]
            //
            // After:
            // [data buffer][data padding][space for parity shards][space for metadata]
            let mut data = data.to_vec();
            data.reserve(total_size.saturating_sub(data_len));
            data.resize(data_and_parity_size, 0); // No need to zero-out the metadata block.

            // Builds the structure that will hold the data + parity shards.
            //
            // Given the following input buffer of 10 bytes, zero padding of 2 bytes, a shard size
            // of 3, 4 data shards, and 1 parity shard:
            //
            // &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 0, 0, 0, 0]
            //
            // We will end up with this:
            //
            // vec![
            //      &[0, 1, 2], // Data shard
            //      &[3, 4, 5], // Data shard
            //      &[6, 7, 8], // Data shard
            //      &[9, 0, 0], // Data shard (with padding)
            //      &[0, 0, 0]  // Parity shard
            // ]
            let mut shards: Vec<&mut [u8]> = data.chunks_mut(shard_size).collect();

            // Encode the parity shards using the `reed_solomon_erasure` crate:
            //
            // Before:
            // [data buffer][data padding][space for parity shards][space for metadata]
            //
            // After:
            // [data buffer][data padding][parity shards][space for metadata]
            reed_solomon.encode(&mut shards)?;

            // Instantiate the `Parameters`:
            let metadata = Parameters::new(
                shard_size,
                total_num_shards,
                num_data_shards,
                data_len,
                Self::compute_checksums(&data, shard_size)
            )?;

            eprintln!("protection scheme: {metadata:#?}");

            // Commit metadata structure to data.
            //
            // Before:
            // [data buffer][data padding][parity shards][space for metadata]
            //
            // After:
            // [data buffer][data padding][parity shards][metadata]
            metadata.into_data_buffer(&mut data)?;

            Ok(data.into())
        }
    }

    /// Verifies and decodes Reed-Solomon encoded data, returning either borrowed or recovered data.
    ///
    /// # Process
    ///
    /// 1. Parse trailing metadata to determine shard structure
    /// 2. Verify shard integrity using embedded checksums
    /// 3. If all shards are intact, return borrowed slice of original data
    /// 4. If corruption detected, reconstruct missing or damaged shards and return owned data
    ///
    /// # Data Structure
    ///
    /// | Data Block             | Parity Block           | Parameters Block                        |
    /// |------------------------|------------------------|---------------------------------------|
    /// | `[u8; N * shard size]` | `[u8; N * shard size]` | `[u32; (4 + total number of shards)]` |
    ///
    /// # Errors
    ///
    /// * If the data is corrupted but not recoverable, or
    /// * If decoding fails due to an internal error.
    pub fn check_and_recover(data: Bytes<'_>) -> Result<Bytes<'_>, Error> {
        // Remember the size of the data block before padding, parity data, or the data block
        // length is added:
        let data_len = data.len();

        // Only attempt to decode data buffers if they meet the size constraints:
        if data_len < DATA_LEN_MIN {
            tracing::debug!("skipping error correction, data size of {data_len} bytes is too small");
            Ok(data)
        } else if data_len > DATA_LEN_MAX - std::mem::size_of::<Parameters>() {
            tracing::debug!("skipping error correction, data size of {data_len} bytes is too large");
            Ok(data)
        } else {
            // Parse metadata from the metadata block of bytes at the end of the data buffer:
            let metadata = Parameters::from_data_buffer(data.as_slice())?;

            eprintln!("checking & recovering with: {metadata:#?}");

            // Check integrity of all data and parity shards using CRC-32. This function will return
            // the byte indices of corrupted shards.
            let corrupted_shards = metadata.check_shards(data.as_slice());

            if corrupted_shards.is_empty() {
                // Fast path. All shards intact, return borrowed slice:
                Ok(data)
            } else {
                // Slow path. Reconstruct corrupted shards:
                tracing::info!("attempting to recover corrupted data");

                // Initialize the Reed-Solomon engine. This is what does the math to create and
                // reconstruct shards using Galois Fields.
                let reed_solomon =
                    reed_solomon_erasure::ReedSolomon::<reed_solomon_erasure::galois_8::Field>::new(
                        metadata.num_data_shards,
                        metadata.total_num_shards - metadata.num_data_shards
                    )?;

                // Reconfigures data buffer into shards for reconstruction.
                //
                // Given the following input buffer of 10 bytes, zero padding of 2 bytes, a shard
                // size of 3, 4 data shards, and 1 parity shard:
                //
                // &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 0, 10, 11, 12]
                //
                // We will end up with something like this: (not byte-accurate, for visual only)
                //
                // vec![
                //      Some(vec![ 0,  1,  2]), // Data shard
                //      None,                   // Corrupted data shard
                //      Some(vec![ 6,  7,  8]), // Data shard
                //      Some(vec![ 9,  0,  0]), // Data shard (with padding)
                //      Some(vec![10, 11, 12])  // Parity shard
                // ]
                let mut prepared_shards: Vec<Option<Vec<u8>>> =
                    metadata.prepare_shards(&data, &corrupted_shards);

                // Attempts to reconstruct the corrupted, and now missing, shards:
                reed_solomon.reconstruct(&mut prepared_shards)?;

                // Data recovery appeared to succeed. We will now reconfigure the data from the
                // sharded arrangement into a flat buffer of bytes:
                let recovered_data: Vec<u8> =
                    metadata.flatten_data_shards(prepared_shards)?;

                // Mark the data was "recovered" so that it may be later recommitted to the
                // database:
                Ok(Bytes::from_recovered_data(recovered_data))
            }
        }
    }

    /// Returns the number of parity shards that should be used to protect the value.
    #[inline]
    fn num_parity_shards(num_data_shards: usize) -> usize {
        // Note: `max(1)` ensures that at least on parity shard will be used, regardless of the
        // math.
        match V::LEVEL {
            Level::Minimum  => 1,
            Level::Medium   => (num_data_shards >> 2).max(1), // 25%
            Level::Maximum  => (num_data_shards >> 1).max(1), // 50%
            Level::Exact(n) => n,
        }
    }

    /// Makes several measurements and calculates metadata values based on the data buffer's length,
    /// in bytes.
    ///
    /// These values are returned as a tuple, in order of:
    /// 0. `shard_size` · Size of both data shards and parity shards, in bytes.
    /// 1. `num_data_shards` · Number of data shards, the number of blocks or chunks that will
    ///    encompass the entire data buffer.
    /// 2. `total_num_shards` · Total number of shards. This includes both data shards and parity
    ///    shards.
    /// 3. `data_and_parity_size` · Total size of the of both the data block (all data shards) and
    ///    the parity block (all parity shards) in bytes.
    /// 4. `total_size` · The total size of the data, parity, and metadata blocks in bytes.
    #[inline]
    fn buffer_measurements(data_len: usize) -> (usize, usize, usize, usize, usize) {
        // Figure out the size of a shard:
        let shard_size = Self::shard_size(data_len);

        tracing::trace!(
            "data size is {data_len} bytes, \
            selected shard size is {shard_size} bytes",
        );

        // Now, figure out how many shards we need to contain the data, the parity data, and
        // both blocks together:
        let num_data_shards = Self::num_data_shards(shard_size, data_len);
        let num_parity_shards = Self::num_parity_shards(num_data_shards);
        let total_num_shards = num_data_shards + num_parity_shards;

        tracing::trace!(
            "wrapping {data_len} bytes of data \
            with {num_data_shards} data shards and {num_parity_shards} parity shards",
        );

        // Pre-calculate total size to avoid multiple reallocations:
        let data_and_parity_size = total_num_shards * shard_size;
        let total_size = data_and_parity_size + std::mem::size_of::<Parameters>();

        tracing::trace!("total encoded data size with metadata will be {total_size} bytes");

        (
            shard_size,
            num_data_shards,
            total_num_shards,
            data_and_parity_size,
            total_size
        )
    }
}

impl<V> ReedSolomon<V> {
    /// Returns a recommended data shard size based on the provided data length (in bytes).
    ///
    /// This function typically produces shard sizes that are 1/4 to 1/8 the size of the data
    /// buffer length, resulting in 4 to 8 shards per record.
    ///
    /// Recommended shard sizes are clamped between 16 and 65,536 bytes for better cache alignment.
    #[inline]
    fn recommended_shard_size(data_len: usize) -> usize {
        if data_len == 0 {
            16
        } else {
            // Find the highest bit position (essentially log2)
            let log2_size = usize::BITS - 1 - data_len.leading_zeros();
            // 1. Shard size is roughly data_len/4 to data_len/8, rounded to next power of 2
            let shard_log2 = log2_size.saturating_sub(2).max(4); // min 16 (2^4)
            // 2. Subtract 2-3 from log2 to divide by 4-8
            (1 << shard_log2).min(65_536)
        }
    }

    /// Returns a data shard size based on the provided data length (in bytes).
    ///
    /// This function typically produces shard sizes that are 1/4 to 1/8 the size of the data type,
    /// resulting in 4 to 8 shards per record.
    ///
    /// Recommended shard sizes are clamped between 16 and 65,536 bytes for better cache alignment.
    ///
    /// Additionally, this function ensures that the number of data shards:
    /// * Aren't too few, and
    /// * Don't exceed the Galois field limit, currently `256` shards.
    #[inline]
    fn shard_size(data_len: usize) -> usize {
        // Shard count constraints to be encorced:
        const MIN_SHARDS: usize = 2;
        const MAX_SHARDS: usize = reed_solomon_erasure::galois_8::Field::ORDER;

        // Get the recommended shard sized based on the data length:
        let mut shard_size = Self::recommended_shard_size(data_len);

        // Ensure that the recommended shard size doesn't cause us to exceed the maximum number of
        // shards (Galois field limit):
        let min_shard_size_for_max = data_len.div_ceil(MAX_SHARDS);
        if shard_size < min_shard_size_for_max {
            // Round down to previous power of 2:
            shard_size = if min_shard_size_for_max.is_power_of_two() {
                min_shard_size_for_max
            } else {
                1 << (usize::BITS - min_shard_size_for_max.leading_zeros())
            };
        }

        // Likewise, ensure that the recommended shard size results in at least `MIN_SHARDS` shards
        // for the data:
        let max_shard_size_for_min = data_len / MIN_SHARDS;
        if shard_size > max_shard_size_for_min {
            // Round down to previous power of 2:
            shard_size = if max_shard_size_for_min.is_power_of_two() {
                max_shard_size_for_min
            } else {
                1 << (usize::BITS - 1 - max_shard_size_for_min.leading_zeros())
            };
        }

        shard_size
    }

    /// Returns the number of data shards required to contain the value.
    #[inline]
    const fn num_data_shards(shard_size: usize, data_len: usize) -> usize {
        data_len.div_ceil(shard_size)
    }

    /// Computes the checksums for the provided shards.
    #[inline]
    fn compute_checksums(
        data: &[u8],
        shard_size: usize
    ) -> Vec<u32> {
        data
            .chunks(shard_size)
            .map(crc32fast::hash)
            .collect()
    }
}

// -------------------------------------------------------------------------------------------------
//
// Shard Size Tests

#[cfg(test)]
mod shard_sizes {
    use super::*;

    /// Test that we respect both mininum and maximum shard constraints.
    #[test]
    fn shard_count_constraints() {
        let test_cases = [
            // Tuples: (data_len, expected_min_shards, expected_max_shards)
            (32, 1, 256),           // Very small data - no min constraint
            (64, 1, 256),           // At threshold - no min constraint yet
            (128, 4, 256),          // Above threshold - min constraint applies
            (1000, 4, 256),         // Medium data
            (100_000, 4, 256),      // Large data
            (10_000_000, 4, 256),   // Very large data
            (100_000_000, 4, 256),  // Extremely large data
        ];

        for &(data_len, min_shards, max_shards) in &test_cases {
            let shard_size = ReedSolomon::<&[u8]>::shard_size(data_len);
            let actual_shards = data_len.div_ceil(shard_size);

            tracing::trace!(
                "data: {data_len} bytes, \
                shard size: {shard_size} bytes, \
                shard count: {actual_shards}"
            );

            assert!(
                actual_shards >= min_shards,
                "shard count {actual_shards} below minimum \
                of {min_shards} for data size of {data_len} bytes"
            );

            assert!(
                actual_shards <= max_shards,
                "shard count {actual_shards} exceeds maximum \
                of {max_shards} for data size of {data_len} bytes"
            );
        }
    }

    /// Verify shard sizes are powers of 2.
    #[test]
    fn power_of_two_sizes() {
        let test_sizes = [100, 1000, 10_000, 100_000, 1_000_000];

        for &size in &test_sizes {
            let shard_size = ReedSolomon::<&[u8]>::shard_size(size);

            assert!(
                shard_size.is_power_of_two(),
                "shard size {shard_size} is not a power of 2 \
                for a data buffer size of {size} bytes"
            );

            assert!(
                (16..=65_536).contains(&shard_size),
                "shard size of {shard_size} bytes is outside bounds \
                for a data buffer size of {size} bytes"
            );
        }
    }
}