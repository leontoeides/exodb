//! Trait implementations that tell the system how to encode & decode types.

#[cfg(feature = "serde-safety")]
impl<T> crate::Codec<T> for T
where T: serde::de::DeserializeOwned + serde::Serialize + crate::codecs::SafeForPostcardSerde {
    /// Encode a value into its binary representation.
    ///
    /// # Errors
    ///
    /// * To understand the possible errors this encoder may produce, please refer to the official
    ///   documentation: <https://docs.rs/postcard>
    fn encode(&self) -> Result<Vec<u8>, crate::codecs::Error> {
        Ok(postcard::to_stdvec(self)?)
    }

    /// Decode a value from its binary representation.
    ///
    /// # Errors
    ///
    /// * To understand the possible errors this decoder may produce, please refer to the official
    ///   documentation: <https://docs.rs/postcard>
    fn decode(bytes: &[u8]) -> Result<T, crate::codecs::Error> {
        Ok(postcard::from_bytes(bytes)?)
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
    ///   documentation: <https://docs.rs/postcard>
    fn encode(&self) -> Result<Vec<u8>, crate::codecs::Error> {
        Ok(postcard::to_stdvec(self)?)
    }

    /// Decode a value from its binary representation.
    ///
    /// # Errors
    ///
    /// * To understand the possible errors this decoder may produce, please refer to the official
    ///   documentation: <https://docs.rs/postcard>
    fn decode(bytes: &[u8]) -> Result<T, crate::codecs::Error> {
        Ok(postcard::from_bytes(bytes)?)
    }
}