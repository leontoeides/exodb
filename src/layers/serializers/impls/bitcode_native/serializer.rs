//! Trait implementation that tells the system how to serialize & deserialize types.

use crate::layers::core::{Bytes, Value};
use crate::layers::serializers::{DeserializeError, Method, SerializeError};

// -------------------------------------------------------------------------------------------------

impl<'b, T> crate::layers::Serializer<'b, T> for T
where T: for<'d> bitcode::Decode<'d> + bitcode::Encode {
    /// Serializes an owned value into its binary representation.
    ///
    /// # Errors
    ///
    /// * To understand the possible errors this serializer may produce, please refer to the
    ///   official documentation: <https://docs.rs/bitcode>
    ///
    /// # Generics & Lifetimes
    ///
    /// * `T` generic represents the user's value type, for example: `User`, `String`, etc.
    /// * `b` lifetime represents bytes potentially being borrowed from the host application.
    #[inline]
    fn serialize(
        self
    ) -> Result<Bytes<'b>, SerializeError> {
        Ok(bitcode::encode(&self).into())
    }

    /// Serializes a borrowed value into its binary representation.
    ///
    /// # Errors
    ///
    /// * To understand the possible errors this serializer may produce, please refer to the
    ///   official documentation: <https://docs.rs/bitcode>
    ///
    /// # Generics & Lifetimes
    ///
    /// * `T` generic represents the user's value type, for example: `User`, `String`, etc.
    /// * `b` lifetime represents bytes potentially being borrowed from the host application.
    #[inline]
    fn serialize_ref(
        &'b self
    ) -> Result<Bytes<'b>, SerializeError> {
        Ok(bitcode::encode(self).into())
    }

    /// Deserializes a series of bytes into a `T` native value.
    ///
    /// # Errors
    ///
    /// * To understand the possible errors this deserializer may produce, please refer to the
    ///   official documentation: <https://docs.rs/bitcode>
    ///
    /// # Generics & Lifetimes
    ///
    /// * `T` generic represents the user's value type, for example: `User`, `String`, etc.
    /// * `b` lifetime represents bytes potentially being borrowed from the `redb` database.
    #[inline]
    fn deserialize(
        serialized_bytes: Bytes<'b>
    ) -> Result<Value<'b, T>, DeserializeError> {
        Ok(bitcode::decode::<T>(serialized_bytes.as_slice())?.into())
    }

    /// Returns the serialization method that the current `Serializer` trait implements.
    ///
    /// This enables runtime identification of the serialization method in use, allowing
    /// applications to log serialization details, or store metadata about how data was processed in
    /// the data pipeline.
    #[inline]
    fn method() -> &'static Method {
        &Method::BitcodeNative
    }
}