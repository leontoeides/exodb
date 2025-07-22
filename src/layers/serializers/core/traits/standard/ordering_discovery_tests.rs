//! These tests are meant for development purposes only. They're used for discovering which
//! `OrderedWhenSerialized` marker traits should be implemented for serializers.
//!
//! This module may not compile and many or most of the tests will fail, depending on the serializer
//! selected at compile time. This is normal. It's why this module is commented-out but kept in the
//! source base.
//!
//! This test unit is not meant to be used outside of setting up new serializers.

use crate::layers::core::Bytes;
use crate::layers::Serializer;

/// Verifies that serialized values maintain their original ordering when sorted by encoded bytes.
///
/// This function tests the fundamental property that for any two values A and B where A < B,
/// the serialized form of A should also be lexicographically less than the serialized form of B.
/// This property is crucial for maintaining correct ordering in binary indexes and sorted storage.
pub fn assert_order_preserved<'a, T>(values: &[T])
where
    T: Serializer<'a, T> + Clone + Ord + std::fmt::Debug + 'a,
{
    // Serialize all input values
    let mut encoded: Vec<Bytes> = values
        .iter()
        .cloned()
        .map(|v| T::serialize(v).expect("Serialization should not fail"))
        .collect();

    // Sort by encoded byte representation
    encoded.sort();

    // Deserialize back to original type
    let decoded: Vec<T> = encoded
        .into_iter()
        .map(|buf| T::deserialize(buf).expect("Deserialization should not fail"))
        .map(|value| value.into_owned().expect("Value conversion should not fail"))
        .collect();

    assert_eq!(
        decoded, values,
        "Ordering was not preserved: serialized byte order differs from logical value order"
    );
}

/// Generates a test function that verifies order preservation for a specific type and value set.
///
/// This macro creates a unit test that serializes the given values, sorts them by their byte
/// representation, deserializes them back, and verifies the result matches the original ordering.
/// The test will fail if the serialization format doesn't preserve lexicographic ordering, which
/// would break assumptions needed for binary search trees and ordered indexes.
#[macro_export]
macro_rules! assert_ordered_encoding {
    ($test_name:ident, $type:ty, [$($value:expr),+ $(,)?]) => {
        #[test]
        fn $test_name() {
            let original_values: &[$type] = &[$($value),+];

            // Serialize each value to bytes
            let mut encoded_bytes: Vec<Vec<u8>> = original_values
                .iter()
                .map(|val| {
                    <$type as Serializer<$type>>::serialize(val.clone())
                        .expect("Serialization failed")
                        .to_vec()
                })
                .collect();

            // Sort by lexicographic byte order
            encoded_bytes.sort();

            // Deserialize back to original type
            let decoded_values: Vec<$type> = encoded_bytes
                .into_iter()
                .map(|bytes| Bytes::from(bytes))
                .map(|buf| {
                    <$type as Serializer<$type>>::deserialize(buf)
                        .expect("Deserialization failed")
                })
                .map(|value| value.into_owned().expect("Value conversion failed"))
                .collect();

            assert_eq!(
                decoded_values, original_values,
                "Byte-sorted order does not match logical value order"
            );
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test `bool` types across full range
    assert_ordered_encoding!(
        bool_order_preservation,
        bool,
        [false, true]
    );

    // Test `char` types across full range
    assert_ordered_encoding!(
        char_order_preservation,
        char,
        [
            "\0".chars().next().unwrap(),
            r"\1".chars().next().unwrap(),
            r"\127".chars().next().unwrap(),
            r"\128".chars().next().unwrap(),
            r"\254".chars().next().unwrap(),
            char::MAX
        ]
    );

    // Test boundary values and edge cases for signed 8-bit integers
    assert_ordered_encoding!(
        i8_order_preservation,
        i8,
        [i8::MIN, -1, 0, 1, i8::MAX]
    );

    // Test comprehensive range for signed 16-bit integers including boundary crossings
    assert_ordered_encoding!(
        i16_order_preservation,
        i16,
        [i16::MIN, -32768, -256, -255, -128, -1, 0, 1, 127, 128, 255, 256, 32767, i16::MAX]
    );

    // Test signed 32-bit integers with focus on byte boundary transitions
    assert_ordered_encoding!(
        i32_order_preservation,
        i32,
        [
            i32::MIN, -100_000, -65536, -32768, -256, -128, -1, 0,
            1, 127, 128, 255, 256, 32767, 65535, 65536, 100_000, i32::MAX
        ]
    );

    // Test signed 64-bit integers across multiple byte boundaries
    assert_ordered_encoding!(
        i64_order_preservation,
        i64,
        [
            i64::MIN, -1_000_000_000, -4_294_967_296, -65536, -256, -1, 0,
            1, 255, 256, 65535, 65536, 4_294_967_295, 4_294_967_296, 1_000_000_000, i64::MAX
        ]
    );

    // Test `isize` integers across multiple byte boundaries
    assert_ordered_encoding!(
        isize_order_preservation,
        isize,
        [
            isize::MIN, -1_000_000_000, -4_294_967_296, -65536, -256, -1, 0,
            1, 255, 256, 65535, 65536, 4_294_967_295, 4_294_967_296, 1_000_000_000, isize::MAX
        ]
    );

    // Test signed 128-bit integers across multiple byte boundaries
    assert_ordered_encoding!(
        i128_order_preservation,
        i128,
        [
            i128::MIN, -1_000_000_000, -4_294_967_296, -65536, -256, -1, 0,
            1, 255, 256, 65535, 65536, 4_294_967_295, 4_294_967_296, 1_000_000_000, i128::MAX
        ]
    );

    // Test unsigned 8-bit integers across full range
    assert_ordered_encoding!(
        u8_order_preservation,
        u8,
        [0, 1, 127, 128, 254, u8::MAX]
    );

    // Test unsigned 16-bit integers with byte boundary focus
    assert_ordered_encoding!(
        u16_order_preservation,
        u16,
        [0, 1, 127, 128, 255, 256, 32767, 32768, 65534, u16::MAX]
    );

    // Test unsigned 32-bit integers across significant boundaries
    assert_ordered_encoding!(
        u32_order_preservation,
        u32,
        [
            0, 1, 255, 256, 65535, 65536, 4_294_967_294, u32::MAX
        ]
    );

    // Test unsigned 64-bit integers with large value transitions
    assert_ordered_encoding!(
        u64_order_preservation,
        u64,
        [
            0, 1, 255, 65535, 4_294_967_295, 4_294_967_296,
            18_446_744_073_709_551_614, u64::MAX
        ]
    );

    // Test `usize` integers with large value transitions
    assert_ordered_encoding!(
        usize_order_preservation,
        usize,
        [
            0, 1, 255, 65535, 4_294_967_295, 4_294_967_296,
            18_446_744_073_709_551_614, usize::MAX
        ]
    );

    // Test unsigned 128-bit integers with large value transitions
    assert_ordered_encoding!(
        u128_order_preservation,
        u128,
        [
            0, 1, 255, 65535, 4_294_967_295, 4_294_967_296,
            18_446_744_073_709_551_614, u128::MAX
        ]
    );

    // Test NonZeroU8 - should behave identically to u8 for non-zero values
    /* assert_ordered_encoding!(
        nonzero_u8_order_preservation,
        std::num::NonZeroU8,
        [
            std::num::NonZeroU8::new(1).unwrap(),
            std::num::NonZeroU8::new(127).unwrap(),
            std::num::NonZeroU8::new(128).unwrap(),
            std::num::NonZeroU8::new(254).unwrap(),
            std::num::NonZeroU8::new(255).unwrap()
        ]
    ); */

    // Test string ordering - should preserve lexicographic UTF-8 byte order
    assert_ordered_encoding!(
        string_order_preservation,
        String,
        [
            String::new(),
            "a".to_string(),
            "aa".to_string(),
            "ab".to_string(),
            "apple".to_string(),
            "banana".to_string(),
            "pear".to_string(),
            "zebra".to_string()
        ]
    );

    // Test strings with special characters and Unicode
    assert_ordered_encoding!(
        string_unicode_order_preservation,
        String,
        [
            "café".to_string(),
            "naïve".to_string(),
            "résumé".to_string(),
            "日本".to_string(),
            "한국".to_string()
        ]
    );

    #[test]
    fn empty_slice_handling() {
        let empty: &[i32] = &[];
        assert_order_preserved(empty); // Should not panic
    }

    #[test]
    fn single_value_handling() {
        assert_order_preserved(&[42i32]);
    }

    #[test]
    fn duplicate_values() {
        assert_order_preserved(&[1i32, 1, 2, 2, 3, 3]);
    }
}