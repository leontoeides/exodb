//! Trait implementations that tell the system how to encode & decode types.

#[cfg(feature = "serde-safety")]
impl<T> crate::Codec<T> for T
where T: serde::de::DeserializeOwned + serde::Serialize + crate::codecs::SafeForRmpSerde {
    /// Encode a value into its binary representation.
    ///
    /// # Errors
    ///
    /// * To understand the possible errors this encoder may produce, please refer to the official
    ///   documentation: <https://docs.rs/rmp-serde>
    fn encode(&self) -> Result<Vec<u8>, crate::codecs::Error> {
        Ok(rmp_serde::to_vec(self)?)
    }

    /// Decode a value from its binary representation.
    ///
    /// # Errors
    ///
    /// * To understand the possible errors this decoder may produce, please refer to the official
    ///   documentation: <https://docs.rs/rmp-serde>
    fn decode(bytes: &[u8]) -> Result<T, crate::codecs::Error> {
        Ok(rmp_serde::from_slice(bytes)?)
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
    ///   documentation: <https://docs.rs/rmp-serde>
    fn encode(&self) -> Result<Vec<u8>, crate::codecs::Error> {
        Ok(rmp_serde::to_vec(self)?)
    }

    /// Decode a value from its binary representation.
    ///
    /// # Errors
    ///
    /// * To understand the possible errors this decoder may produce, please refer to the official
    ///   documentation: <https://docs.rs/rmp-serde>
    fn decode(bytes: &[u8]) -> Result<T, crate::codecs::Error> {
        Ok(rmp_serde::from_slice(bytes)?)
    }
}