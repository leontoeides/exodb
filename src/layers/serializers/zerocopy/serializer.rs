//! Trait implementations that tell the system how to serialize & deserialize types.

use crate::layers::{serializers::{Error, Method}, ValueBuf};

// -------------------------------------------------------------------------------------------------

impl<'b, T> crate::layers::Serializer<'b, T> for T
where
    T: zerocopy::Immutable + zerocopy::IntoBytes + zerocopy::TryFromBytes
{
    /// Serialize a value into its binary representation.
    ///
    /// # Errors
    ///
    /// * To understand the possible errors this serializer may produce, please refer to the
    ///   official documentation: <https://docs.rs/zerocopy>
    #[inline]
    fn serialize(&self) -> Result<ValueBuf<'b>, Error> {
        Ok(self.as_bytes().to_vec().into())
    }

    /// Deserialize a value into its binary representation.
    ///
    /// # Errors
    ///
    /// * To understand the possible errors this deserializer may produce, please refer to the
    ///   official documentation: <https://docs.rs/zerocopy>
    #[inline]
    fn deserialize(serialized_bytes: ValueBuf<'b>) -> Result<T, Error> {
        T::try_read_from_bytes(&serialized_bytes).map_err(|_error| Error::Zerocopy)
    }

    /// Returns the serialization method that the current `Serializer` trait implements.
    ///
    /// This enables runtime identification of the serialization method in use, allowing
    /// applications to log serialization details, or store metadata about how data was processed in
    /// the data pipeline.
    #[inline]
    fn method() -> &'static Method {
        &Method::Zerocopy
    }
}