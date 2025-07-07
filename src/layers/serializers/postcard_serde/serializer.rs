//! Trait implementations that tell the system how to serialize & deserialize types.

use crate::layers::{serializers::{Error, Method}, ValueBuf};
use musli::{Decode, Encode, alloc::System, mode::Binary};

// -------------------------------------------------------------------------------------------------

#[cfg(feature = "serde-safety")]
impl<'b, T> crate::layers::Serializer<'b, T> for T
where T: serde::de::DeserializeOwned + serde::Serialize + crate::layers::serializers::SafeForPostcardSerde {
    /// Serialize a value into its binary representation.
    ///
    /// # Errors
    ///
    /// * To understand the possible errors this serializer may produce, please refer to the
    ///   official documentation: <https://docs.rs/postcard>
    #[inline]
    fn serialize(&self) -> Result<ValueBuf<'b>, Error> {
        Ok(postcard::to_stdvec(self)?.into())
    }

    /// Deserialize a value into its binary representation.
    ///
    /// # Errors
    ///
    /// * To understand the possible errors this deserializer may produce, please refer to the
    ///   official documentation: <https://docs.rs/postcard>
    #[inline]
    fn deserialize(serialized_bytes: ValueBuf<'b>) -> Result<T, Error> {
        Ok(postcard::from_bytes(&serialized_bytes)?)
    }

    /// Returns the serialization method that the current `Serializer` trait implements.
    ///
    /// This enables runtime identification of the serialization method in use, allowing
    /// applications to log serialization details, or store metadata about how data was processed in
    /// the data pipeline.
    #[inline]
    fn method() -> &'static Method {
        &Method::PostcardSerde
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
    ///   official documentation: <https://docs.rs/postcard>
    #[inline]
    fn serialize(&self) -> Result<ValueBuf<'b>, Error> {
        Ok(postcard::to_stdvec(self)?.into())
    }

    /// Deserialize a value into its binary representation.
    ///
    /// # Errors
    ///
    /// * To understand the possible errors this deserializer may produce, please refer to the
    ///   official documentation: <https://docs.rs/postcard>
    #[inline]
    fn deserialize(serialized_bytes: ValueBuf<'b>) -> Result<T, Error> {
        Ok(postcard::from_bytes(&serialized_bytes)?)
    }

    /// Returns the serialization method that the current `Serializer` trait implements.
    ///
    /// This enables runtime identification of the serialization method in use, allowing
    /// applications to log serialization details, or store metadata about how data was processed in
    /// the data pipeline.
    #[inline]
    fn method() -> &'static Method {
        &Method::PostcardSerde
    }
}