//! Trait implementations that tell the system how to serialize & deserialize types.

use crate::layers::{serializers::{Error, Method}, ValueBuf};
use musli::{Decode, Encode, alloc::System, mode::Binary};

// -------------------------------------------------------------------------------------------------

impl<'b, T> crate::layers::Serializer<'b, T> for T
where
    T: Encode<Binary> + for<'de> Decode<'de, Binary, System>
{
    /// Serialize a value into its binary representation.
    ///
    /// # Errors
    ///
    /// * To understand the possible errors this serializer may produce, please refer to the
    ///   official documentation: <https://docs.rs/musli>
    #[inline]
    fn serialize(&self) -> Result<ValueBuf<'b>, Error> {
        Ok(musli::storage::to_vec(self)?.into())
    }

    /// Deserialize a value into its binary representation.
    ///
    /// * To understand the possible errors this deserializer may produce, please refer to the
    ///   official documentation: <https://docs.rs/musli>
    #[inline]
    fn deserialize(bytes: ValueBuf<'b>) -> Result<T, Error> {
        Ok(musli::storage::from_slice(&bytes)?)
    }

    /// Returns the serialization method that the current `Serializer` trait implements.
    ///
    /// This enables runtime identification of the serialization method in use, allowing
    /// applications to log serialization details, or store metadata about how data was processed in
    /// the data pipeline.
    #[inline]
    fn method() -> &'static Method {
        &Method::MusliStorage
    }
}