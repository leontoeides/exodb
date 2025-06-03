//! Trait implementations that tell the system how to encode & decode types.

impl<T> crate::Codec<T> for T
where T: borsh::BorshDeserialize + borsh::BorshSerialize {
    /// Encode a value into its binary representation.
    ///
    /// # Errors
    ///
    /// * To understand the possible errors this encoder may produce, please refer to the official
    ///   documentation: <https://docs.rs/borsh>
    fn encode(&self) -> Result<Vec<u8>, crate::codecs::Error> {
        Ok(borsh::to_vec(self)?)
    }

    /// Decode a value from its binary representation.
    ///
    /// # Errors
    ///
    /// * To understand the possible errors this decoder may produce, please refer to the official
    ///   documentation: <https://docs.rs/borsh>
    fn decode(bytes: &[u8]) -> Result<T, crate::codecs::Error> {
        Ok(borsh::from_slice(bytes)?)
    }
}