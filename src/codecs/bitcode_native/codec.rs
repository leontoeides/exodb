//! Trait implementations that tell the system how to encode & decode types.

impl<T> crate::Codec<T> for T
where T: bitcode::DecodeOwned + bitcode::Encode {
    /// Encode a value into its binary representation.
    ///
    /// # Errors
    ///
    /// * To understand the possible errors this encoder may produce, please refer to the official
    ///   documentation: <https://docs.rs/bitcode>
    fn encode(&self) -> Result<Vec<u8>, crate::codecs::Error> {
        Ok(bitcode::encode(self))
    }

    /// Decode a value from its binary representation.
    ///
    /// # Errors
    ///
    /// * To understand the possible errors this decoder may produce, please refer to the official
    ///   documentation: <https://docs.rs/bitcode>
    fn decode(bytes: &[u8]) -> Result<T, crate::codecs::Error> {
        Ok(bitcode::decode(bytes)?)
    }
}