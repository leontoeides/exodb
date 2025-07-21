//! Trait implementations that tell the system how to serialize & deserialize types.

use crate::layers::serializers::Method;
use rkyv::bytecheck::CheckBytes;
use rkyv::de::Pool;
use rkyv::rancor::{Error, Strategy};
use rkyv::ser::{allocator::ArenaHandle, Serializer, sharing::Share};
use rkyv::util::AlignedVec;
use rkyv::validation::{archive::ArchiveValidator, shared::SharedValidator, Validator};
use rkyv::{Deserialize, Serialize, Portable};

// -------------------------------------------------------------------------------------------------

#[cfg(feature = "serde-safety")]
impl<'b, T> crate::layers::SerializerBorrowing<'b, T> for T
where T: Deserialize<T, Strategy<Pool, Error>> +
    Serialize<Strategy<Pool, Error>> +
    Portable +
    for<'r> CheckBytes<Strategy<Validator<ArchiveValidator<'r>, SharedValidator>, Error>> +
    for<'r> Serialize<Strategy<Serializer<AlignedVec, ArenaHandle<'r>, Share>, rkyv::rancor::Error>>
{
    #[inline]
    fn serialize(
        &self
    ) -> Result<Vec<u8>, crate::layers::serializers::SerializeError> {
        Ok(rkyv::to_bytes::<rkyv::rancor::Error>(self)?.to_vec())
    }

    #[inline]
    fn deserialize(
        serialized_bytes: &[u8]
    ) -> Result<&'b T, crate::layers::serializers::DeserializeError> {
        let archived = rkyv::access::<T, rkyv::rancor::Error>(serialized_bytes)?;
        let deserialized = rkyv::deserialize::<T, rkyv::rancor::Error>(archived)?;
        Ok(&deserialized)
    }

    #[inline]
    fn method() -> &'static Method {
        &Method::Rkyv
    }
}

#[cfg(not(feature = "serde-safety"))]
impl<T> crate::layers::SerializerBorrowing<'b, T> for T
where T: serde::de::DeserializeOwned + serde::Serialize {
    /// Serialize a value into its binary representation.
    ///
    /// # Errors
    ///
    /// * To understand the possible errors this serializer may produce, please refer to the
    ///   official documentation: <https://docs.rs/rkyv>
    #[inline]
    fn serialize(
        &self
    ) -> Result<Vec<u8>, crate::layers::serializers::SerializeError> {
        Ok(rmp_serde::to_vec(self)?.into())
    }

    /// Deserialize a value into its binary representation.
    ///
    /// # Errors
    ///
    /// * To understand the possible errors this deserializer may produce, please refer to the
    ///   official documentation: <https://docs.rs/rkyv>
    #[inline]
    fn deserialize(
        serialized_bytes: &[u8]
    ) -> Result<T, crate::layers::serializers::DeserializeError> {
        Ok(rmp_serde::from_slice(&serialized_bytes)?)
    }

    /// Returns the serialization method that the current `SerializerBorrowing` trait implements.
    ///
    /// This enables runtime identification of the serialization method in use, allowing
    /// applications to log serialization details, or store metadata about how data was processed in
    /// the data pipeline.
    #[inline]
    fn method() -> &'static Method {
        &Method::MessagePack
    }
}