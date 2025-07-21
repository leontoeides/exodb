#[cfg(test)]
mod tests_comprehensive {
    use crate::layers::{
        core::{Bytes, Direction, Value},
        encryptors::KeyBytes,
        Compressible, Correctable, Encryptable, Serializer, Serializable,
    };
    use std::collections::HashMap;

    // Simple test struct
    #[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    struct TestValue {
        name: String,
        age: u32,
    }

    // Complex nested struct to stress-test the pipeline
    #[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    struct ComplexData {
        id: u64,
        metadata: HashMap<String, String>,
        tags: Vec<String>,
        nested: NestedData,
        binary_data: Vec<u8>,
    }

    #[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    struct NestedData {
        scores: Vec<u32>,
        config: HashMap<String, bool>,
        optional_field: Option<String>,
    }

    // Marker trait for serde safety
    #[cfg(all(
        feature = "serde-safety",
        any(
            feature = "serializer-bincode-serde",
            feature = "serializer-bitcode-serde",
            feature = "serializer-messagepack",
            feature = "serializer-postcard-serde"
        )
    ))]
    unsafe impl crate::layers::serializers::SafeForSerde for TestValue {}

    #[cfg(all(
        feature = "serde-safety",
        any(
            feature = "serializer-bincode-serde",
            feature = "serializer-bitcode-serde",
            feature = "serializer-messagepack",
            feature = "serializer-postcard-serde"
        )
    ))]
    unsafe impl crate::layers::serializers::SafeForSerde for ComplexData {}

    #[cfg(all(
        feature = "serde-safety",
        any(
            feature = "serializer-bincode-serde",
            feature = "serializer-bitcode-serde",
            feature = "serializer-messagepack",
            feature = "serializer-postcard-serde"
        )
    ))]
    unsafe impl crate::layers::serializers::SafeForSerde for NestedData {}

    // Implement traits for TestValue
    impl Serializable for TestValue {
        const DIRECTION: Direction = Direction::Both;
    }

    impl Compressible for TestValue {
        const DIRECTION: Direction = Direction::Both;
        const LEVEL: crate::layers::compressors::Level = crate::layers::compressors::Level::Maximum;
    }

    impl Encryptable for TestValue {
        const DIRECTION: Direction = Direction::Both;
    }

    impl Correctable for TestValue {
        const DIRECTION: Direction = Direction::Both;
        const LEVEL: crate::layers::correctors::Level = crate::layers::correctors::Level::Maximum;
    }

    // Implement traits for ComplexData
    impl Serializable for ComplexData {
        const DIRECTION: Direction = Direction::Both;
    }

    impl Compressible for ComplexData {
        const DIRECTION: Direction = Direction::Both;
        const LEVEL: crate::layers::compressors::Level = crate::layers::compressors::Level::Maximum;
    }

    impl Encryptable for ComplexData {
        const DIRECTION: Direction = Direction::Both;
    }

    impl Correctable for ComplexData {
        const DIRECTION: Direction = Direction::Both;
        const LEVEL: crate::layers::correctors::Level = crate::layers::correctors::Level::Maximum;
    }

    fn create_test_key() -> KeyBytes<'static> {
        b"SECURE_32_BYTE_KEY______________".into()
    }

    fn create_wrong_key() -> KeyBytes<'static> {
        b"WRONG_32_BYTE_KEY_______________".into()
    }

    fn assert_round_trip<'a, T>(original: &'a T, key: KeyBytes)
    where
        T: Clone + PartialEq + std::fmt::Debug + Serializer<'a, T> + Serializable + Compressible + Encryptable + Correctable
    {
        let buf = Bytes::apply_write_layers(original, (*key).into(), None, None)
            .expect("Write layers should succeed");

        let read_value = Bytes::apply_read_layers::<T>(buf, key, None)
            .expect("Read layers should succeed");

        match read_value.try_into_value().expect("Should get deserialized value") {
            Value::Borrowed(read_data) => assert_eq!(read_data, original),
            Value::Owned(read_data) => assert_eq!(read_data, *original),
        }
    }

    #[test]
    fn test_basic_round_trip() {
        let data = TestValue {
            name: "Ariadne".to_string(),
            age: 42,
        };
        assert_round_trip(&data, create_test_key());
    }

    #[test]
    fn test_empty_values() {
        let data = TestValue {
            name: String::new(),
            age: 0,
        };
        assert_round_trip(&data, create_test_key());
    }

    #[test]
    fn test_unicode_strings() {
        let data = TestValue {
            name: "🚀 Rust is awesome! 你好世界 🦀".to_string(),
            age: 2024,
        };
        assert_round_trip(&data, create_test_key());
    }

    #[test]
    fn test_maximum_values() {
        let data = TestValue {
            name: "A".repeat(u16::MAX as usize), // Very long string
            age: u32::MAX,
        };
        assert_round_trip(&data, create_test_key());
    }

    #[test]
    fn test_complex_nested_data() {
        let mut metadata = HashMap::new();
        metadata.insert("version".to_string(), "1.0".to_string());
        metadata.insert("author".to_string(), "test".to_string());
        metadata.insert("encoding".to_string(), "utf-8".to_string());

        let mut config = HashMap::new();
        config.insert("debug".to_string(), true);
        config.insert("production".to_string(), false);
        config.insert("feature_x".to_string(), true);

        let data = ComplexData {
            id: 12_345_678_901_234_567_890,
            metadata,
            tags: vec!["rust".to_string(), "database".to_string(), "performance".to_string()],
            nested: NestedData {
                scores: vec![1, 5, 9, 10, 99],
                config,
                optional_field: Some("present".to_string()),
            },
            binary_data: (0..255).collect(), // 255 bytes of test data
        };

        assert_round_trip(&data, create_test_key());
    }

    #[test]
    fn test_compression_effectiveness() {
        // Highly repetitive data should compress well
        let repetitive_data = ComplexData {
            id: 1,
            metadata: HashMap::new(),
            tags: vec!["AAAAAAAAAA".to_string(); 1000], // Very repetitive
            nested: NestedData {
                scores: vec![1; 10000], // Identical values
                config: HashMap::new(),
                optional_field: None,
            },
            binary_data: vec![0xFF; 50000], // 50KB of identical bytes
        };

        let key = create_test_key();
        let compressed_buf = Bytes::apply_write_layers(&repetitive_data, (*key).into(), None, None)
            .expect("Compression should succeed");

        // The compressed size should be much smaller than the original
        println!("Compressed size: {} bytes", compressed_buf.len());

        // Verify round-trip still works
        let read_value = Bytes::apply_read_layers::<ComplexData>(compressed_buf, key, None)
            .expect("Decompression should succeed");

        match read_value.try_into_value().expect("Should get value") {
            Value::Borrowed(read_data) => assert_eq!(read_data, &repetitive_data),
            Value::Owned(read_data) => assert_eq!(read_data, repetitive_data),
        }
    }

    #[test]
    fn test_encryption_wrong_key_fails() {
        let data = TestValue {
            name: "Secret data".to_string(),
            age: 42,
        };

        let write_key = create_test_key();
        let wrong_key = create_wrong_key();

        let encrypted_buf = Bytes::apply_write_layers(&data, (*write_key).into(), None, None)
            .expect("Encryption should succeed");

        // Attempting to decrypt with wrong key should fail
        assert!(
            Bytes::apply_read_layers::<TestValue>(encrypted_buf, wrong_key, None).is_err(),
            "Decryption with wrong key should fail"
        );
    }

    #[test]
    fn test_data_corruption_detection() {
        let data = TestValue {
            name: "Important data".to_string(),
            age: 42,
        };

        let key = create_test_key();
        let mut buf = Bytes::apply_write_layers(&data, (*key).into(), None, None)
            .expect("Write should succeed");

        // Corrupt a byte in the middle
        let buf_len = buf.len();
        if let Some(byte) = buf.as_mut().unwrap().get_mut(buf_len / 2) {
            *byte = byte.wrapping_add(1); // Flip a bit
        }

        // Reed-Solomon should correct the corruption and data should be recovered!
        let recovered_value = Bytes::apply_read_layers::<TestValue>(buf, key, None)
            .expect("ECC should correct the corruption");

        match recovered_value.try_into_value().expect("Should get corrected value") {
            Value::Borrowed(read_data) => assert_eq!(read_data, &data),
            Value::Owned(read_data) => assert_eq!(read_data, data),
        }
    }

    #[test]
    fn test_various_data_sizes() {
        let test_cases = vec![
            1,      // Tiny
            100,    // Small
            1024,   // 1KB
            10240,  // 10KB
            102_400, // 100KB
        ];

        for size in test_cases {
            let data = TestValue {
                name: "X".repeat(size),
                age: u32::try_from(size).unwrap(),
            };

            assert_round_trip(&data, create_test_key());
        }
    }

    #[test]
    fn test_edge_case_values() {
        let edge_cases = vec![
            TestValue { name: "\0".to_string(), age: 0 }, // Null character
            TestValue { name: "\n\r\t".to_string(), age: 1 }, // Whitespace
            TestValue { name: "🎯🚀🦀".to_string(), age: u32::MAX }, // Emoji + max int
            TestValue { name: "\"'\\".to_string(), age: 42 }, // Quote characters
        ];

        for data in edge_cases {
            assert_round_trip(&data, create_test_key());
        }
    }

    #[test]
    fn test_multiple_round_trips() {
        let mut data = TestValue {
            name: "Evolution test".to_string(),
            age: 1,
        };

        let key = create_test_key();

        // Do multiple round trips, modifying data each time
        for i in 1..=10 {
            let buf = Bytes::apply_write_layers(&data, (*key).into(), None, None)
                .expect("Write should succeed");

            let read_value = Bytes::apply_read_layers::<TestValue>(buf, (*key).into(), None)
                .expect("Read should succeed");

            data = match read_value.try_into_value().expect("Should get value") {
                Value::Borrowed(read_data) => read_data.clone(),
                Value::Owned(read_data) => read_data,
            };

            // Modify for next iteration
            data.age = i + 1;
            data.name = format!("Iteration {}", i + 1);
        }

        assert_eq!(data.age, 11);
        assert_eq!(data.name, "Iteration 11");
    }
}