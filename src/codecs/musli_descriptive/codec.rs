//! Trait implementations that tell the system how to encode & decode types.

use musli::{Decode, Encode, alloc::System, mode::Binary};

// -------------------------------------------------------------------------------------------------

impl<T> crate::Codec<T> for T
where
    T: Encode<Binary> + for<'de> Decode<'de, Binary, System>
{
    /// Encode a value into its binary representation.
    ///
    /// # Errors
    ///
    /// * To understand the possible errors this encoder may produce, please refer to the official
    ///   documentation: <https://docs.rs/musli>
    fn encode(&self) -> Result<Vec<u8>, crate::codecs::Error> {
        Ok(musli::descriptive::to_vec(self)?)
    }

    /// Decode a value from its binary representation.
    ///
    /// * To understand the possible errors this encoder may produce, please refer to the official
    ///   documentation: <https://docs.rs/musli>
    fn decode(bytes: &[u8]) -> Result<T, crate::codecs::Error> {
        Ok(musli::descriptive::from_slice(bytes)?)
    }
}