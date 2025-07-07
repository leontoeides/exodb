//! Trait implementations that tell the system how to serialize & deserialize types.

use rkyv::bytecheck::CheckBytes;
use rkyv::de::Pool;
use rkyv::rancor::{Error, Strategy};
use rkyv::ser::{allocator::ArenaHandle, Serializer, sharing::Share};
use rkyv::util::AlignedVec;
use rkyv::validation::{archive::ArchiveValidator, shared::SharedValidator, Validator};
use rkyv::{Deserialize, Serialize, Portable};

// -------------------------------------------------------------------------------------------------

impl<T> crate::Codec<T> for T
where
    T: Deserialize<T, Strategy<Pool, Error>> +
        Serialize<Strategy<Pool, Error>> +
        Portable +
        for<'a> CheckBytes<Strategy<Validator<ArchiveValidator<'a>, SharedValidator>, Error>> +
        for<'a> Serialize<Strategy<Serializer<AlignedVec, ArenaHandle<'a>, Share>, rkyv::rancor::Error>>
{
    /// Serialize a value into its binary representation.
    ///
    /// # Errors
    ///
    /// * To understand the possible errors this serializer may produce, please refer to the
    ///   official documentation: <https://docs.rs/rkyv>
    fn serialize(&self) -> Result<Vec<u8>, crate::layers::serializers::Error> {
        Ok(rkyv::to_bytes::<rkyv::rancor::Error>(self)?.to_vec())
    }

    /// Deserialize a value into its binary representation.
    ///
    /// # Errors
    ///
    /// * To understand the possible errors this deserializer may produce, please refer to the
    ///   official documentation: <https://docs.rs/rkyv>
    fn deserialize(bytes: &[u8]) -> Result<T, crate::layers::serializers::Error> {
        let archived = rkyv::access::<T, rkyv::rancor::Error>(bytes)?;
        let deserialized = rkyv::deserialize::<T, rkyv::rancor::Error>(archived)?;
        Ok(deserialized)
    }
}