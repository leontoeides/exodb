//! Trait implementations that tell the system how to encode & decode types.

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
    /// Encode a value into its binary representation.
    ///
    /// # Errors
    ///
    /// * To understand the possible errors this encoder may produce, please refer to the official
    ///   documentation: <https://docs.rs/rkyv>
    fn encode(&self) -> Result<Vec<u8>, crate::codecs::Error> {
        Ok(rkyv::to_bytes::<rkyv::rancor::Error>(self)?.to_vec())
    }

    /// Decode a value from its binary representation.
    ///
    /// # Errors
    ///
    /// * To understand the possible errors this decoder may produce, please refer to the official
    ///   documentation: <https://docs.rs/rkyv>
    fn decode(bytes: &[u8]) -> Result<T, crate::codecs::Error> {
        let archived = rkyv::access::<T, rkyv::rancor::Error>(bytes)?;
        let deserialized = rkyv::deserialize::<T, rkyv::rancor::Error>(archived)?;
        Ok(deserialized)
    }
}