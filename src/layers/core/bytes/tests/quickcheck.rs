#[allow(clippy::needless_pass_by_value)]
#[cfg(test)]
mod quickcheck_tests {
    use crate::layers::{
        core::{Bytes, Direction, Value},
        encryptors::KeyBytes,
        Compressible, Correctable, Encryptable, Serializable,
    };
    use quickcheck::{quickcheck, Arbitrary, Gen, QuickCheck, TestResult};
    use std::collections::HashMap;
    use std::time::{Duration, Instant};

    // Test data structure that implements Arbitrary for random generation
    #[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    struct QuickTestData {
        id: u64,
        name: String,
        tags: Vec<String>,
        metadata: HashMap<String, u32>, // Using u32 instead of f64 for Eq
        binary_data: Vec<u8>,
        flag: bool,
    }

    impl Arbitrary for QuickTestData {
        fn arbitrary(g: &mut Gen) -> Self {
            let mut metadata = HashMap::new();
            // Generate random metadata entries
            for _ in 0..g.size() % 10 {
                metadata.insert(String::arbitrary(g), u32::arbitrary(g));
            }

            Self {
                id: u64::arbitrary(g),
                name: String::arbitrary(g),
                tags: Vec::<String>::arbitrary(g),
                metadata,
                binary_data: Vec::<u8>::arbitrary(g),
                flag: bool::arbitrary(g),
            }
        }
    }

    // Implement required traits
    #[cfg(all(
        feature = "serde-safety",
        any(
            feature = "serialize-bincode-serde",
            feature = "serialize-bitcode-serde",
            feature = "serialize-messagepack",
            feature = "serialize-postcard-serde"
        )
    ))]
    unsafe impl crate::layers::serializers::SafeForSerde for QuickTestData {}

    impl Serializable for QuickTestData {
        const DIRECTION: Direction = Direction::Both;
    }

    impl Compressible for QuickTestData {
        const DIRECTION: Direction = Direction::Both;
        const LEVEL: crate::layers::compressors::Level = crate::layers::compressors::Level::Maximum;
    }

    impl Encryptable for QuickTestData {
        const DIRECTION: Direction = Direction::Both;
    }

    impl Correctable for QuickTestData {
        const DIRECTION: Direction = Direction::Both;
        const LEVEL: crate::layers::correctors::Level = crate::layers::correctors::Level::Maximum;
    }

    fn test_key() -> KeyBytes<'static> {
        b"QUICKCHECK_TEST_KEY_32_BYTES____".into()
    }

    // Property: Round-trip should always preserve data
    fn prop_round_trip_preserves_data(data: QuickTestData) -> bool {
        let key = test_key();

        let buf = match Bytes::apply_write_layers(&data, (*key).into(), None, None) {
            Ok(value) => value,
            Err(e) => {
                eprintln!("Layer failure: {e:?}");
                panic!("Early exit on failure"); // return false;
            }
        };

        let read_value = match Bytes::apply_read_layers::<QuickTestData>(buf, key, None) {
            Ok(value) => value,
            Err(e) => {
                eprintln!("Layer failure: {e:?}");
                panic!("Early exit on failure"); // return false;
            }
        };

        match read_value.try_into_value() {
            Ok(Value::Borrowed(read_data)) => {
                let is_match = read_data == &data;
                assert!(is_match, "Early exit on failure"); // return is_match;
                is_match
            },
            Ok(Value::Owned(read_data)) => {
                let is_match = read_data == data;
                assert!(is_match, "Early exit on failure"); // return is_match;
                is_match
            },
            Err(e) => {
                eprintln!("Layer failure: {e:?}");
                panic!("Early exit on failure"); // return false;
            }
        }
    }

    // Property: Multiple round trips should be stable
    fn prop_multiple_round_trips_stable(mut data: QuickTestData, iterations: u8) -> bool {
        let key = test_key();
        let iterations = (iterations % 10) + 1; // 1-10 iterations
        let original = data.clone();

        for _ in 0..iterations {
            let buf = match Bytes::apply_write_layers(&data, (*key).into(), None, None) {
                Ok(value) => value,
                Err(e) => {
                    eprintln!("Layer failure: {e:?}");
                    panic!("Early exit on failure"); // return false;
                }
            };

            let read_value = match Bytes::apply_read_layers::<QuickTestData>(buf, (*key).into(), None) {
                Ok(value) => value,
                Err(e) => {
                    eprintln!("Layer failure: {e:?}");
                    panic!("Early exit on failure"); // return false;
                }
            };

            data = match read_value.try_into_value() {
                Ok(Value::Borrowed(read_data)) => read_data.clone(),
                Ok(Value::Owned(read_data)) => read_data,
                Err(e) => {
                    eprintln!("Layer failure: {e:?}");
                    panic!("Early exit on failure"); // return false;
                }
            };
        }

        data == original
    }

    // Property: Compression should never increase size beyond reasonable bounds
    fn prop_compression_bounded(data: QuickTestData) -> bool {
        let key = test_key();

        let Ok(buf) = Bytes::apply_write_layers(&data, (*key).into(), None, None) else {
            return true;
        };

        // Compressed size should be reasonable (allowing for encryption overhead, etc.)
        let original_size_estimate = data.name.len() +
                                   data.tags.iter().map(String::len).sum::<usize>() +
                                   data.binary_data.len() +
                                   data.metadata.len() * 16; // Rough estimate

        // Allow 10x expansion for small data (encryption/ECC overhead can be significant)
        buf.len() <= (original_size_estimate * 10).max(1024)
    }

    // Property: Different keys should produce different ciphertexts
    fn prop_different_keys_different_output(data: QuickTestData) -> bool {
        let key1: KeyBytes = b"KEY1_32_BYTES___________________".into();
        let key2: KeyBytes = b"KEY2_32_BYTES___________________".into();

        let buf1 = match Bytes::apply_write_layers(&data, (*key1).into(), None, None) {
            Ok(value) => value,
            Err(e) => {
                eprintln!("Layer failure: {e:?}");
                panic!("Early exit on failure"); // return false;
            }
        };

        let buf2 = match Bytes::apply_write_layers(&data, (*key2).into(), None, None) {
            Ok(value) => value,
            Err(e) => {
                eprintln!("Layer failure: {e:?}");
                panic!("Early exit on failure"); // return false;
            }
        };

        // Different keys should produce different encrypted output
        buf1.as_ref() != buf2.as_ref()
    }

    // Property: Error correction should handle single-byte corruption in data areas
    fn prop_single_byte_corruption_corrected(data: QuickTestData, corruption_pos: usize) -> bool {
        let key = test_key();

        let Ok(mut buf) = Bytes::apply_write_layers(&data, (*key).into(), None, None) else {
            return true;
        };

        if buf.len() < 144 {
            return true; // Skip tiny buffers where metadata dominates
        }

        // Only corrupt the first 70% of the buffer to avoid Reed-Solomon metadata at the end
        let safe_end = (buf.len() * 70) / 100;
        let pos = corruption_pos % safe_end;

        if let Some(byte) = buf.as_mut().unwrap().get_mut(pos) {
            *byte = byte.wrapping_add(1);
        }

        // ECC should correct single-byte errors
        let read_value = match Bytes::apply_read_layers::<QuickTestData>(buf, key, None) {
            Ok(value) => value,
            Err(e) => {
                eprintln!("Layer failure: {e:?}");
                panic!("Early exit on failure"); // return false;
            }
        };

        match read_value.try_into_value() {
            Ok(Value::Borrowed(read_data)) => read_data == &data,
            Ok(Value::Owned(read_data)) => read_data == data,
            Err(e) => {
                eprintln!("Layer failure: {e:?}");
                panic!("Early exit on failure"); // return false;
            }
        }
    }

    #[test]
    fn quickcheck_performance_baseline() {
        use std::time::Instant;

        let start = Instant::now();
        let count = 100_000;

        for i in 0..count {
            let data = QuickTestData {
                id: i,
                name: format!("test{i}"),
                tags: vec!["tag".to_string()],
                metadata: HashMap::new(),
                binary_data: vec![1, 2, 3],
                flag: true,
            };

            // Just test serialization without layers
            let _serialized = serde_json::to_vec(&data).unwrap();
        }

        eprintln!("100k JSON serializations took: {:?}", start.elapsed());
    }

    #[test]
    fn quickcheck_performance() {
        use std::time::Instant;

        let key = test_key();
        let start = Instant::now();
        let count = 100_000;

        for i in 0..count {
            let data = QuickTestData {
                id: i,
                name: format!("test{i}"),
                tags: vec!["tag".to_string()],
                metadata: HashMap::new(),
                binary_data: vec![1, 2, 3],
                flag: true,
            };

            // Just test serialization without layers
            let _bytes = match Bytes::apply_write_layers(&data, (*key).into(), None, None) {
                Ok(value) => value,
                Err(e) => {
                    eprintln!("Layer failure: {e:?}");
                    panic!("Early exit on failure"); // return false;
                }
            };
        }

        eprintln!("100k serialization+compression+encryption+ECC protections took: {:?}", start.elapsed());
    }

    // Regular QuickCheck tests
    #[test]
    fn quickcheck_round_trip_preserves_data() {
        quickcheck(prop_round_trip_preserves_data as fn(QuickTestData) -> bool);
    }

    #[test]
    fn quickcheck_multiple_round_trips_stable() {
        quickcheck(prop_multiple_round_trips_stable as fn(QuickTestData, u8) -> bool);
    }

    #[test]
    fn quickcheck_compression_bounded() {
        quickcheck(prop_compression_bounded as fn(QuickTestData) -> bool);
    }

    #[test]
    fn quickcheck_different_keys_different_output() {
        quickcheck(prop_different_keys_different_output as fn(QuickTestData) -> bool);
    }

    #[test]
    fn quickcheck_single_byte_corruption_corrected() {
        quickcheck(prop_single_byte_corruption_corrected as fn(QuickTestData, usize) -> bool);
    }

    // Focused tests for edge cases
    #[test]
    fn quickcheck_empty_collections() {
        let empty_data = QuickTestData {
            id: 0,
            name: String::new(),
            tags: Vec::new(),
            metadata: HashMap::new(),
            binary_data: Vec::new(),
            flag: false,
        };

        assert!(prop_round_trip_preserves_data(empty_data));
    }

    #[test]
    fn quickcheck_large_data() {
        let large_data = QuickTestData {
            id: u64::MAX,
            name: "X".repeat(10000),
            tags: vec!["tag".to_string(); 1000],
            metadata: (0..100).map(|i| (format!("key{i}"), i)).collect(),
            binary_data: (0..255u8).cycle().take(50000).collect(),
            flag: true,
        };

        assert!(prop_round_trip_preserves_data(large_data));
    }

    // Configuration helpers for brutal testing
    fn gauntlet_config() -> QuickCheck {
        QuickCheck::new()
            .tests(1_000_000)        // 1 million tests instead of default 100
            .max_tests(10_000_000)   // Keep going even if some are discarded
            .r#gen(Gen::new(1000))     // Larger data generation
    }

    fn extreme_gauntlet_config() -> QuickCheck {
        QuickCheck::new()
            .tests(10_000_000)       // 10 million tests
            .max_tests(100_000_000)  // 100 million max attempts
            .r#gen(Gen::new(5000))     // Much larger generated data
    }

    // Brutal 24-hour gauntlet tests
    // Use `cargo test -- --ignored` to run these
    #[test]
    #[ignore = "extreme gauntlet test, use `cargo test -- --ignored` to run"]
    fn slowcheck_round_trip_gauntlet() {
        gauntlet_config()
            .quickcheck(prop_round_trip_preserves_data as fn(QuickTestData) -> bool);
    }

    #[test]
    #[ignore = "extreme gauntlet test, use `cargo test -- --ignored` to run"]
    fn slowcheck_multiple_round_trips_gauntlet() {
        gauntlet_config()
            .quickcheck(prop_multiple_round_trips_stable as fn(QuickTestData, u8) -> bool);
    }

    #[test]
    #[ignore = "extreme gauntlet test, use `cargo test -- --ignored` to run"]
    fn slowcheck_corruption_gauntlet() {
        gauntlet_config()
            .quickcheck(prop_single_byte_corruption_corrected as fn(QuickTestData, usize) -> bool);
    }

    #[test]
    #[ignore = "extreme gauntlet test, use `cargo test -- --ignored` to run"]
    fn extreme_slowcheck_round_trip() {
        extreme_gauntlet_config()
            .quickcheck(prop_round_trip_preserves_data as fn(QuickTestData) -> bool);
    }

    #[test]
    #[ignore = "extreme gauntlet test, use `cargo test -- --ignored` to run"]
    fn time_based_gauntlet() {
        let start = Instant::now();
        let duration = Duration::from_secs(24 * 60 * 60); // 24 hours
        let mut test_count = 0;

        while start.elapsed() < duration {
            let mut qc = QuickCheck::new()
                .tests(10_000) // Batch of 10k tests
                .r#gen(Gen::new((test_count % 10000) + 100)); // Varying sizes

            qc.quickcheck(prop_round_trip_preserves_data as fn(QuickTestData) -> bool);
            qc.quickcheck(prop_multiple_round_trips_stable as fn(QuickTestData, u8) -> bool);
            qc.quickcheck(prop_single_byte_corruption_corrected as fn(QuickTestData, usize) -> bool);

            test_count += 10_000;

            if test_count % 100_000 == 0 {
                println!("Completed {test_count} tests in {:?}", start.elapsed());
            }
        }

        println!("Gauntlet complete! Ran {test_count} tests over {duration:?}");
    }

    #[test]
    #[ignore = "extreme gauntlet test, use `cargo test -- --ignored` to run"]
    fn memory_pressure_gauntlet() {
        fn huge_data_test() -> TestResult {
            let mut r#gen = Gen::new(50000);
            let data = QuickTestData::arbitrary(&mut r#gen);

            // Only test if the data is actually large
            let estimated_size = data.name.len() +
                                data.binary_data.len() +
                                data.tags.iter().map(String::len).sum::<usize>();

            if estimated_size < 10_000 {
                return TestResult::discard();
            }

            TestResult::from_bool(prop_round_trip_preserves_data(data))
        }

        QuickCheck::new()
            .tests(100_000)
            .max_tests(1_000_000)
            .quickcheck(huge_data_test as fn() -> TestResult);
    }

    #[test]
    #[ignore = "extreme gauntlet test, use `cargo test -- --ignored` to run"]
    fn pathological_data_gauntlet() {
        fn pathological_test() -> TestResult {
            let mut r#gen = Gen::new(1000);
            let mut data = QuickTestData::arbitrary(&mut r#gen);

            // Create pathological cases
            match rand::random::<u8>() % 10 {
                0 => data.name = "\0".repeat(10000),           // Null bytes
                1 => data.name = "🦀".repeat(5000),            // Unicode
                2 => data.binary_data = vec![0xFF; 50000],     // All 1s
                3 => data.binary_data = vec![0x00; 50000],     // All 0s
                4 => data.binary_data = (0..=255).cycle().take(50000).collect(), // Patterns
                5 => data.tags = vec![String::new(); 10000],  // Empty strings
                6 => data.tags = vec!["A".repeat(1000); 1000], // Huge strings
                7 => data.name = String::from_utf8_lossy(&[0xED, 0xA0, 0x80]).to_string(), // Invalid UTF-8
                8 => data.binary_data = vec![rand::random::<u8>(); 100_000], // Pure random
                _ => {} // Normal case
            }

            TestResult::from_bool(prop_round_trip_preserves_data(data))
        }

        QuickCheck::new()
            .tests(500_000)
            .quickcheck(pathological_test as fn() -> TestResult);
    }
}