//! Trait implementations that tell the system how to serialize & deserialize types.

use crate::layers::{serializers::{Error, Method}, ValueBuf};

// -------------------------------------------------------------------------------------------------

impl<'b, T> crate::layers::Serializer<'b, T> for T
where T: bincode::Decode<()> + bincode::Encode {
    /// Serialize a value into its binary representation.
    ///
    /// # Errors
    ///
    /// * To understand the possible errors this serializer may produce, please refer to the
    ///   official documentation: <https://docs.rs/bincode>
    #[inline]
    fn serialize(&self) -> Result<ValueBuf<'b>, Error> {
        Ok(bincode::encode_to_vec(self, bincode::config::standard())?.into())
    }

    /// Deserialize a value into its binary representation.
    ///
    /// # Errors
    ///
    /// * To understand the possible errors this deserializer may produce, please refer to the
    ///   official documentation: <https://docs.rs/bincode>
    #[inline]
    fn deserialize(serialized_bytes: ValueBuf<'b>) -> Result<T, Error> {
        let (value, _bytes_read) = bincode::decode_from_slice(
            &serialized_bytes,
            bincode::config::standard()
        )?;
        Ok(value)
    }

    /// Returns the serialization method that the current `Serializer` trait implements.
    ///
    /// This enables runtime identification of the serialization method in use, allowing
    /// applications to log serialization details, or store metadata about how data was processed in
    /// the data pipeline.
    #[inline]
    fn method() -> &'static Method {
        &Method::BincodeNative
    }
}