//! Trait implementation that tells the system how to serialize & deserialize types.

use crate::layers::core::{Bytes, Value};
use crate::layers::serializers::{DeserializeError, Method, SerializeError};
use std::borrow::Cow;

// -------------------------------------------------------------------------------------------------

impl<'b, T> crate::layers::Serializer<'b, T> for T
where T: Clone + musli_zerocopy::ZeroCopy {
    /// Serializes an owned `T` value into its binary representation.
    ///
    /// # Errors
    ///
    /// * To understand the possible errors this serializer may produce, please refer to the
    ///   official documentation: <https://docs.rs/musli-zerocopy>
    ///
    /// # Generics & Lifetimes
    ///
    /// * `T` generic represents the user's value type, for example: `User`, `String`, etc.
    /// * `b` lifetime represents bytes potentially being borrowed from the host application.
    #[inline]
    fn serialize(
        mut self
    ) -> Result<Bytes<'b>, SerializeError> {
        Ok(self.to_bytes().to_vec().into())
    }

    /// Serializes a borrowed `&T` value into its binary representation.
    ///
    /// # Errors
    ///
    /// * To understand the possible errors this serializer may produce, please refer to the
    ///   official documentation: <https://docs.rs/musli-zerocopy>
    ///
    /// # Generics & Lifetimes
    ///
    /// * `T` generic represents the user's value type, for example: `User`, `String`, etc.
    /// * `b` lifetime represents bytes potentially being borrowed from the host application.
    #[inline]
    fn serialize_ref(
        &'b self
    ) -> Result<Bytes<'b>, SerializeError> {
        // The below `clone` is used for mutable access so Musli can generate padding:
        Ok(self.clone().to_bytes().to_vec().into())
    }

    /// Deserializes a series of bytes into a `T` native value.
    ///
    /// # Errors
    ///
    /// * To understand the possible errors this deserializer may produce, please refer to the
    ///   official documentation: <https://docs.rs/musli-zerocopy>
    ///
    /// # Generics & Lifetimes
    ///
    /// * `T` generic represents the user's value type, for example: `User`, `String`, etc.
    /// * `b` lifetime represents bytes potentially being borrowed from the `redb` database.
    #[inline]
    fn deserialize(
        serialized_bytes: Bytes<'b>
    ) -> Result<Value<'b, T>, DeserializeError> {
        let (_metadata, data) = serialized_bytes.into_parts();

        match data {
            Cow::Borrowed(slice) => {
                let value = T::from_bytes(slice)?;
                Ok(Value::Borrowed(value))
            },
            Cow::Owned(vec) => {
                let value = T::from_bytes(&vec)?;
                Ok(Value::Owned(value.clone()))
            },
        }
    }

    /// Returns the serialization method that the current `Serializer` trait implements.
    ///
    /// This enables runtime identification of the serialization method in use, allowing
    /// applications to log serialization details, or store metadata about how data was processed in
    /// the data pipeline.
    #[inline]
    fn method() -> &'static Method {
        &Method::MusliZeroCopy
    }
}