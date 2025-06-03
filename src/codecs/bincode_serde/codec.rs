//! Trait implementations that tell the system how to encode & decode types.

#[cfg(feature = "serde-safety")]
impl<T> crate::Codec<T> for T
where T: serde::de::DeserializeOwned + serde::Serialize + crate::codecs::SafeForBincodeSerde {
    /// Encode a value into its binary representation.
    ///
    /// # Errors
    ///
    /// * To understand the possible errors this encoder may produce, please refer to the official
    ///   documentation: <https://docs.rs/bincode>
    fn encode(&self) -> Result<Vec<u8>, crate::codecs::Error> {
        Ok(bincode::serde::encode_to_vec(self, bincode::config::standard())?)
    }

    /// Decode a value from its binary representation.
    ///
    /// # Errors
    ///
    /// * To understand the possible errors this decoder may produce, please refer to the official
    ///   documentation: <https://docs.rs/bincode>
    fn decode(bytes: &[u8]) -> Result<T, crate::codecs::Error> {
        let (value, _bytes_read) = bincode::serde::decode_from_slice(bytes, bincode::config::standard())?;
        Ok(value)
    }
}

#[cfg(not(feature = "serde-safety"))]
impl<T> crate::Codec<T> for T
where T: serde::de::DeserializeOwned + serde::Serialize {
    /// Encode a value into its binary representation.
    ///
    /// # Errors
    ///
    /// * To understand the possible errors this encoder may produce, please refer to the official
    ///   documentation: <https://docs.rs/bincode>
    fn encode(&self) -> Result<Vec<u8>, crate::codecs::Error> {
        Ok(bincode::serde::encode_to_vec(self, bincode::config::standard())?)
    }

    /// Decode a value from its binary representation.
    ///
    /// # Errors
    ///
    /// * To understand the possible errors this decoder may produce, please refer to the official
    ///   documentation: <https://docs.rs/bincode>
    fn decode(bytes: &[u8]) -> Result<T, crate::codecs::Error> {
        let (value, _bytes_read) = bincode::serde::decode_from_slice(bytes, bincode::config::standard())?;
        Ok(value)
    }
}