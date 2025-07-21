#[allow(clippy::needless_pass_by_value)]
#[cfg(test)]
mod quickcheck_tests {
    use crate::layers::{
        core::{Bytes, Direction, Value},
        encryptors::KeyBytes,
        Compressible, Correctable, Encryptable, Serializable,
    };
    use quickcheck::{quickcheck, Arbitrary, Gen};
    use std::collections::HashMap;

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
                metadata.insert(
                    String::arbitrary(g),
                    u32::arbitrary(g),
                );
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
            feature = "serializer-bincode-serde",
            feature = "serializer-bitcode-serde",
            feature = "serializer-messagepack",
            feature = "serializer-postcard-serde"
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
        
        let Ok(buf) = Bytes::apply_write_layers(&data, (*key).into(), None, None) else {
            return false
        };

        let read_value = match Bytes::apply_read_layers::<QuickTestData>(buf, key, None) {
            Ok(value) => value,
            Err(e) => {
                eprintln!("Layer failure: {e:?}");
                return false;
            }
        };

        match read_value.try_into_value() {
            Ok(Value::Borrowed(read_data)) => read_data == &data,
            Ok(Value::Owned(read_data)) => read_data == data,
            Err(_) => false,
        }
    }

    // Property: Multiple round trips should be stable
    fn prop_multiple_round_trips_stable(mut data: QuickTestData, iterations: u8) -> bool {
        let key = test_key();
        let iterations = (iterations % 10) + 1; // 1-10 iterations
        let original = data.clone();

        for _ in 0..iterations {
            let Ok(buf) = Bytes::apply_write_layers(&data, (*key).into(), None, None) else {
                return false
            };

            let read_value = match Bytes::apply_read_layers::<QuickTestData>(buf, (*key).into(), None) {
                Ok(value) => value,
                Err(e) => {
                    eprintln!("Layer failure: {e:?}");
                    return false;
                }
            };

            data = match read_value.try_into_value() {
                Ok(Value::Borrowed(read_data)) => read_data.clone(),
                Ok(Value::Owned(read_data)) => read_data,
                Err(_) => return false,
            };
        }

        data == original
    }

    // Property: Compression should never increase size beyond reasonable bounds
    fn prop_compression_bounded(data: QuickTestData) -> bool {
        let key = test_key();
        
        let Ok(buf) = Bytes::apply_write_layers(&data, (*key).into(), None, None) else {
            return true
        };

        // Compressed size should be reasonable (allowing for encryption overhead, etc.)
        // This is a loose bound - in practice, small data might expand due to overhead
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

        let Ok(buf1) = Bytes::apply_write_layers(&data, (*key1).into(), None, None) else {
            return true
        };

        let Ok(buf2) = Bytes::apply_write_layers(&data, (*key2).into(), None, None) else {
            return true
        };

        // Different keys should produce different encrypted output
        buf1.as_ref() != buf2.as_ref()
    }

    // Property: Error correction should handle single-byte corruption
    fn prop_single_byte_corruption_corrected(data: QuickTestData, corruption_pos: usize) -> bool {
        let key = test_key();
        
        let Ok(mut buf) = Bytes::apply_write_layers(&data, (*key).into(), None, None) else {
            return true
        };

        if buf.len() < 144 { return true; } // Skip tiny buffers where metadata dominates

        // Only corrupt the first 75% of the buffer to avoid Reed-Solomon metadata at the end
        let safe_end = (buf.len() * 70) / 100;
        let pos = corruption_pos % safe_end;

        if let Some(byte) = buf.as_mut().unwrap().get_mut(pos) {
            *byte = byte.wrapping_add(1);
        }

        // Corrupt a single byte
        let pos = corruption_pos % buf.len();
        if let Some(byte) = buf.as_mut().unwrap().get_mut(pos) {
            *byte = byte.wrapping_add(1);
        }

        // ECC should correct single-byte errors
        let read_value = match Bytes::apply_read_layers::<QuickTestData>(buf, key, None) {
            Ok(value) => value,
            Err(e) => {
                eprintln!("Layer failure: {e:?}");
                return false;
            }
        };

        match read_value.try_into_value() {
            Ok(Value::Borrowed(read_data)) => read_data == &data,
            Ok(Value::Owned(read_data)) => read_data == data,
            Err(_) => false,
        }
    }

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
}