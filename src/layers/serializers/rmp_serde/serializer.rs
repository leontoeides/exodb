//! Trait implementations that tell the system how to serialize & deserialize types.

use crate::layers::{serializers::Method, ValueBuf};

// -------------------------------------------------------------------------------------------------

#[cfg(feature = "serde-safety")]
impl<'b, T> crate::layers::Serializer<'_, T> for T
where T: serde::de::DeserializeOwned + serde::Serialize + crate::layers::serializers::SafeForRmpSerde {
    /// Serialize a value into its binary representation.
    ///
    /// # Errors
    ///
    /// * To understand the possible errors this serializer may produce, please refer to the
    ///   official documentation: <https://docs.rs/rmp-serde>
    #[inline]
    fn serialize(
        &self
    ) -> Result<ValueBuf<'_>, crate::layers::serializers::SerializeError> {
        Ok(rmp_serde::to_vec(self)?.into())
    }

    /// Deserialize a value into its binary representation.
    ///
    /// # Errors
    ///
    /// * To understand the possible errors this deserializer may produce, please refer to the
    ///   official documentation: <https://docs.rs/rmp-serde>
    #[inline]
    fn deserialize(
        serialized_bytes: ValueBuf<'_>
    ) -> Result<T, crate::layers::serializers::DeserializeError> {
        Ok(rmp_serde::from_slice(&serialized_bytes)?)
    }

    /// Returns the serialization method that the current `Serializer` trait implements.
    ///
    /// This enables runtime identification of the serialization method in use, allowing
    /// applications to log serialization details, or store metadata about how data was processed in
    /// the data pipeline.
    #[inline]
    fn method() -> &'static Method {
        &Method::RmpSerde
    }
}

#[cfg(not(feature = "serde-safety"))]
impl<T> crate::layers::Serializer<T> for T
where T: serde::de::DeserializeOwned + serde::Serialize {
    /// Serialize a value into its binary representation.
    ///
    /// # Errors
    ///
    /// * To understand the possible errors this serializer may produce, please refer to the
    ///   official documentation: <https://docs.rs/rmp-serde>
    #[inline]
    fn serialize(
        &self
    ) -> Result<ValueBuf<'_>, crate::layers::serializers::SerializeError> {
        Ok(rmp_serde::to_vec(self)?.into())
    }

    /// Deserialize a value into its binary representation.
    ///
    /// # Errors
    ///
    /// * To understand the possible errors this deserializer may produce, please refer to the
    ///   official documentation: <https://docs.rs/rmp-serde>
    #[inline]
    fn deserialize(
        serialized_bytes: ValueBuf<'_>
    ) -> Result<T, crate::layers::serializers::DeserializeError> {
        Ok(rmp_serde::from_slice(&serialized_bytes)?)
    }

    /// Returns the serialization method that the current `Serializer` trait implements.
    ///
    /// This enables runtime identification of the serialization method in use, allowing
    /// applications to log serialization details, or store metadata about how data was processed in
    /// the data pipeline.
    #[inline]
    fn method() -> &'static Method {
        &Method::RmpSerde
    }
}