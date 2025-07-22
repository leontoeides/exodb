//! Trait implementation that tells the system how to serialize & deserialize types.

use crate::layers::core::{Bytes, Value};
use crate::layers::serializers::{DeserializeError, Method, SerializeError};

// -------------------------------------------------------------------------------------------------

#[cfg(feature = "serde-safety")]
impl<'b, T> crate::layers::Serializer<'b, T> for T
where T: serde::de::DeserializeOwned + serde::Serialize + crate::layers::serializers::impls::bincode_serde::serde_safety::SafeForBincodeSerde {
    /// Serializes an owned value into its binary representation.
    ///
    /// # Errors
    ///
    /// * To understand the possible errors this serializer may produce, please refer to the
    ///   official documentation: <https://docs.rs/bincode>
    ///
    /// # Generics & Lifetimes
    ///
    /// * `T` generic represents the user's value type, for example: `User`, `String`, etc.
    /// * `b` lifetime represents bytes potentially being borrowed from the host application.
    #[inline]
    fn serialize(
        self
    ) -> Result<Bytes<'b>, SerializeError> {
        Ok(bincode::serde::encode_to_vec(self, bincode::config::standard())?.into())
    }

    /// Serializes a borrowed value into its binary representation.
    ///
    /// # Errors
    ///
    /// * To understand the possible errors this serializer may produce, please refer to the
    ///   official documentation: <https://docs.rs/bincode>
    ///
    /// # Generics & Lifetimes
    ///
    /// * `T` generic represents the user's value type, for example: `User`, `String`, etc.
    /// * `b` lifetime represents bytes potentially being borrowed from the host application.
    #[inline]
    fn serialize_ref(
        &'b self
    ) -> Result<Bytes<'b>, SerializeError> {
        Ok(bincode::serde::encode_to_vec(self, bincode::config::standard())?.into())
    }

    /// Deserializes a series of bytes into a `T` native value.
    ///
    /// # Errors
    ///
    /// * To understand the possible errors this deserializer may produce, please refer to the
    ///   official documentation: <https://docs.rs/bincode>
    ///
    /// # Generics & Lifetimes
    ///
    /// * `T` generic represents the user's value type, for example: `User`, `String`, etc.
    /// * `b` lifetime represents bytes potentially being borrowed from the `redb` database.
    #[inline]
    fn deserialize(
        serialized_bytes: Bytes<'b>
    ) -> Result<Value<'b, T>, DeserializeError> {
        let (value, _bytes_read): (T, usize) = bincode::serde::decode_from_slice(
            &serialized_bytes,
            bincode::config::standard()
        )?;
        Ok(value.into())
    }

    /// Returns the serialization method that the current `Serializer` trait implements.
    ///
    /// This enables runtime identification of the serialization method in use, allowing
    /// applications to log serialization details, or store metadata about how data was processed in
    /// the data pipeline.
    #[inline]
    fn method() -> &'static Method {
        &Method::BincodeSerde
    }
}

#[cfg(not(feature = "serde-safety"))]
impl<'b, T> crate::layers::Serializer<'b, T> for T
where T: serde::de::DeserializeOwned + serde::Serialize {
    /// Serializes an owned value into its binary representation.
    ///
    /// # Errors
    ///
    /// * To understand the possible errors this serializer may produce, please refer to the
    ///   official documentation: <https://docs.rs/bincode>
    ///
    /// # Generics & Lifetimes
    ///
    /// * `T` generic represents the user's value type, for example: `User`, `String`, etc.
    /// * `b` lifetime represents bytes potentially being borrowed from the host application.
    #[inline]
    fn serialize(
        self
    ) -> Result<Bytes<'b>, SerializeError> {
        Ok(bincode::serialize(&self)?.into())
    }

    /// Serializes a borrowed value into its binary representation.
    ///
    /// # Errors
    ///
    /// * To understand the possible errors this serializer may produce, please refer to the
    ///   official documentation: <https://docs.rs/bincode>
    ///
    /// # Generics & Lifetimes
    ///
    /// * `T` generic represents the user's value type, for example: `User`, `String`, etc.
    /// * `b` lifetime represents bytes potentially being borrowed from the host application.
    #[inline]
    fn serialize_ref(
        &'b self
    ) -> Result<Bytes<'b>, SerializeError> {
        Ok(bincode::serialize(self)?.into())
    }

    /// Deserializes a series of bytes into a `T` native value.
    ///
    /// # Errors
    ///
    /// * To understand the possible errors this deserializer may produce, please refer to the
    ///   official documentation: <https://docs.rs/bincode>
    ///
    /// # Generics & Lifetimes
    ///
    /// * `T` generic represents the user's value type, for example: `User`, `String`, etc.
    /// * `b` lifetime represents bytes potentially being borrowed from the `redb` database.
    #[inline]
    fn deserialize(
        serialized_bytes: Bytes<'b>
    ) -> Result<Value<'b, T>, DeserializeError> {
        Ok(bincode::deserialize::<T>(&serialized_bytes)?.into())
    }

    /// Returns the serialization method that the current `Serializer` trait implements.
    ///
    /// This enables runtime identification of the serialization method in use, allowing
    /// applications to log serialization details, or store metadata about how data was processed in
    /// the data pipeline.
    #[inline]
    fn method() -> &'static Method {
        &Method::BincodeSerde
    }
}