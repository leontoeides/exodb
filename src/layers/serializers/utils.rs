use crate::Codec;

// -------------------------------------------------------------------------------------------------
//
/// Tests that encoded values preserve order. Panics if violated.
pub fn assert_order_preserved<T: Codec<T> + Ord + std::fmt::Debug>(values: &[T]) {
    let mut encoded = values
        .iter()
        .map(|v| T::serialize(v).expect("failed to encode"))
        .collect::<Vec<_>>();

    encoded.sort();

    let decoded = encoded
        .iter()
        .map(|b| T::deserialize(b).expect("failed to decode"))
        .collect::<Vec<_>>();

    assert_eq!(
        decoded, values,
        "Ordering was not preserved during round-trip encode/decode"
    );
}

/* #[cfg(test)]
mod tests {
    #[test]
    fn bincode_u8_ordered() {
        #[cfg(feature = "bincode")]
        assert_ordered_encoding!(u8_ordering, u8, [1, 2, 3, 4, 5]);
    }
} */

#[macro_export]
macro_rules! assert_ordered_encoding {
    ($name:ident, $ty:ty, [$($val:expr),+ $(,)?]) => {
        #[test]
        fn $name() {
            let inputs: &[$ty] = &[$($val),+];
            let mut encoded = inputs
                .iter()
                .map(|x| <$ty as Codec<$ty>>::serialize(x).expect("encoding failed"))
                .collect::<Vec<_>>();

            encoded.sort();

            let decoded = encoded
                .into_iter()
                .map(|b| <$ty as Codec<$ty>>::deserialize(&b).expect("decoding failed"))
                .collect::<Vec<_>>();

            assert_eq!(decoded, inputs, "Encoded sort order did not preserve original ordering");
        }
    };
}

assert_ordered_encoding!(i16_big_endian_sorts_correctly, i16, [-32767, -256, -255, -128, -127, -2, 1, 2, 127, 128, 255, 256, 32767]);
assert_ordered_encoding!(i32_big_endian_sorts_correctly, i32, [-99999, -65536, -65535, -32767, -256, -255, -128, -127 -2, -1, 1, 2, 127, 128, 255, 256, 32767, 65535, 65536, 99999]);
assert_ordered_encoding!(i64_big_endian_sorts_correctly, i64, [-99999, -65536, -65535, -32767, -256, -255, -128, -127 -2, -1, 1, 2, 127, 128, 255, 256, 32767, 65535, 65536, 99999]);
assert_ordered_encoding!(i8_big_endian_sorts_correctly, i8, [-127, -2, -1, 1, 2, 127]);
assert_ordered_encoding!(string_utf8_sorts_correctly, String, ["apple".to_string(), "banana".to_string(), "pear".to_string()]);
assert_ordered_encoding!(u16_big_endian_sorts_correctly, u16, [1, 2, 127, 128, 255, 256, 32767, 65535]);
assert_ordered_encoding!(u32_big_endian_sorts_correctly, u32, [1, 2, 127, 128, 255, 256, 32767, 65535, 65536, 99999]);
assert_ordered_encoding!(u64_big_endian_sorts_correctly, u64, [1, 2, 127, 128, 255, 256, 32767, 65535, 65536, 99999]);
assert_ordered_encoding!(u8_big_endian_sorts_correctly, u8, [1, 2, 127, 128, 255]);