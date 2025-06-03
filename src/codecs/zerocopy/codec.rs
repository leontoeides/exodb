//! Trait implementations that tell the system how to encode & decode types.

impl<T> crate::Codec<T> for T
where
    T: zerocopy::Immutable + zerocopy::IntoBytes + zerocopy::TryFromBytes
{
    /// Encode a value into its binary representation.
    ///
    /// # Errors
    ///
    /// * To understand the possible errors this encoder may produce, please refer to the official
    ///   documentation: <https://docs.rs/zerocopy>
    fn encode(&self) -> Result<Vec<u8>, crate::codecs::Error> {
        Ok(self.as_bytes().to_vec())
    }

    /// Decode a value from its binary representation.
    ///
    /// # Errors
    ///
    /// * To understand the possible errors this decoder may produce, please refer to the official
    ///   documentation: <https://docs.rs/zerocopy>
    fn decode(bytes: &[u8]) -> Result<T, crate::codecs::Error> {
        T::try_read_from_bytes(bytes).map_err(|_error| crate::codecs::Error::Zerocopy)
    }
}