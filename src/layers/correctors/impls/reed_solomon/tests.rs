// -------------------------------------------------------------------------------------------------
//
// Encoder/Decoder Tests

#[cfg(test)]
mod round_trip {
	use crate::layers::core::{Bytes, Direction};
	use crate::layers::correctors::impls::reed_solomon::*;
	use crate::layers::correctors::{Correctable, Level};

    // Mock type implementing `Correctable` for testing
    struct TestValue;

    impl Correctable for TestValue {
		const DIRECTION: Direction = Direction::Both;
        const LEVEL: Level = Level::Maximum;
    }

    #[test]
    fn corrector_round_trip() {
        // Original data: 12 bytes to fit 3 shards of 4 bytes
        let data = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let original_data = data.clone();

        // Encode data with Reed-Solomon:
        let protected_data = ReedSolomon::<TestValue>::add_parity((&data).into()).unwrap();

        // Decode data:
        let recovered_data = ReedSolomon::<TestValue>::check_and_recover(protected_data).unwrap();

        // Verify round-trip:
		assert_eq!(recovered_data.as_slice(), original_data.as_slice());
    }

	#[test]
	fn corrector_round_trip_with_missing_shard() {
	    // Original data: 12 bytes for 2 shards:
	    let data = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
	    let original_data = data.clone();

	    // Encode data:
	    let protected_data = ReedSolomon::<TestValue>::add_parity((&data).into()).unwrap();

	    // Parse metadata to get shard layout:
	    let metadata = Parameters::from_data_buffer(&mut protected_data.clone()).unwrap();
	    let shard_size = metadata.shard_size;

	    // Simulate missing data shard (index 0) by zeroing it:
	    let shard_start = 0;
	    let shard_end = shard_size;

	    let mut protected_data = protected_data.to_vec();
	    protected_data[shard_start..shard_end].fill(0);

	    // Decode and reconstruct:
	    let recovered_data = ReedSolomon::<TestValue>::check_and_recover(protected_data.into()).unwrap();

	    // Verify round-trip:
	    assert_eq!(recovered_data.as_slice(), original_data.as_slice());
	}

    /// A parity shard is not expected to be generated for this data. The protectr and recoverr should
    /// skip error corrrection.
    #[test]
    fn corrector_skip_tiny() {
        // Original data: 2 bytes
        let data = vec![0, 1];
        let original_data = data.clone();

        // Encode data with Reed-Solomon:
        let protected_data = ReedSolomon::<TestValue>::add_parity((&data).into()).unwrap();

        // Decode data:
        let recovered_data = ReedSolomon::<TestValue>::check_and_recover(protected_data).unwrap();

        // Verify round-trip:
	   	assert_eq!(recovered_data.as_slice(), original_data.as_slice());
    }

    /// A parity shard is expected to be generated for this data.
    #[test]
    fn corrector_tiny() {
        // Original data: 3 bytes
        let data = vec![0, 1, 3];
        let original_data = data.clone();

        // Encode data with Reed-Solomon:
        let protected_data = ReedSolomon::<TestValue>::add_parity((&data).into()).unwrap();

        // Decode data:
        let recovered_data = ReedSolomon::<TestValue>::check_and_recover(protected_data).unwrap();

        // Verify round-trip:
	   	assert_eq!(recovered_data.as_slice(), original_data.as_slice());
    }

	#[test]
	fn round_trip_no_protection() {
		struct NoProtection;
		impl Correctable for NoProtection {
			const DIRECTION: Direction = Direction::None;
	        const LEVEL: Level = Level::Maximum;
		}

	    let data = b"no protections, no problems".to_vec();
	    let original_data = data.clone();

	    let protected_data = ReedSolomon::<NoProtection>::add_parity((&data).into()).unwrap();
	    let recovered_data = ReedSolomon::<NoProtection>::check_and_recover(protected_data).unwrap();

	    assert_eq!(recovered_data.as_slice(), original_data.as_slice());
	}

	#[test]
	fn encoding_data_too_small() {
	    struct Lowball;
	    impl Correctable for Lowball {
			const DIRECTION: Direction = Direction::Both;
	        const LEVEL: Level = Level::Maximum;
	    }

	    let data = vec![2u8];
	    let original_data = data.clone();

	    let protected_data = ReedSolomon::<Lowball>::add_parity((&data).into()).unwrap();

	    assert_eq!(protected_data.as_slice(), original_data.as_slice());
	}

	#[test]
	fn parity_shard_corruption_triggers_recovery() {
	    struct TestValue;
	    impl Correctable for TestValue {
			const DIRECTION: Direction = Direction::Both;
	        const LEVEL: Level = Level::Maximum;
	    }

	    let data = b"parity corruption check".to_vec();
	    let original_data = data.clone();

	    // Encode the data
	    let protected_data = ReedSolomon::<TestValue>::add_parity((&data).into()).unwrap();

	    // Parse metadata to access shard info
	    let metadata = Parameters::from_data_buffer(&mut protected_data.clone()).unwrap();
	    let shard_size = metadata.shard_size;

	    // Corrupt a parity shard (not data shard)
	    let parity_index = metadata.num_data_shards; // first parity shard
	    let start = parity_index * shard_size;
	    let end = start + shard_size;

	    let mut protected_data = protected_data.to_vec();
	    protected_data[start..end].fill(0);

	    // Decode the data
	    let recovered_data = ReedSolomon::<TestValue>::check_and_recover(protected_data.into()).unwrap();

	    // Assert that recovery path was triggered (even though data is intact)
	    assert_eq!(recovered_data.as_slice(), original_data.as_slice());
	}

	#[test]
	fn multiple_corruptions_within_limit() {
	    struct HighProtect;
	    impl Correctable for HighProtect {
			const DIRECTION: Direction = Direction::Both;
	        const LEVEL: Level = Level::Exact(2); // 2 parity shards, allows 2 corruptions
	    }

	    let data = vec![99u8; 64]; // data_len = 64
	    let original_data = data.clone();

	    let protected_data = ReedSolomon::<HighProtect>::add_parity((&data).into()).unwrap();

	    let metadata = Parameters::from_data_buffer(&mut protected_data.clone()).unwrap();
	    let shard_size = metadata.shard_size;
	    let mut protected_data = protected_data.to_vec();

	    // Corrupt two data shards, which is OK with 2 parity shards
	    for &index in &[0, 2] {
	        let start = index * shard_size;
	        let end = start + shard_size;
	        protected_data[start..end].fill(0);
	    }

	    let recovered_data = ReedSolomon::<HighProtect>::check_and_recover(protected_data.into()).unwrap();

	    assert_eq!(recovered_data.as_slice(), original_data.as_slice());
	}

	#[test]
	fn invalid_metadata_should_fail() {
	    let data = vec![42u8; 32];
	    let protected_data = ReedSolomon::<TestValue>::add_parity((&data).into()).unwrap();

	    // Chop off the metadata
	    let mut protected_data = protected_data.to_vec();
	    protected_data.truncate(protected_data.len() - std::mem::size_of::<Parameters>());
	    let mut protected_data: Bytes = protected_data.into();

	    let result = Parameters::from_data_buffer(&mut protected_data);
	    assert!(matches!(
	    	result,
	    	Err(crate::layers::correctors::impls::reed_solomon::parameters::Error::InsufficientData {
	    		parameter: _,
	    		error: _
	    	})
	    ));
	}
}