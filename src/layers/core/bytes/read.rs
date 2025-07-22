use crate::layers::{
    Compressible,
    core::Bytes,
    core::Value,
    core::ValueOrBytes,
    Correctable,
    Encryptable,
    encryptors::KeyBytes,
    Serializable,
    Serializer,
    core::bytes::Error,
};

#[cfg(feature = "compress-dictionaries")]
use crate::layers::compressors::DictionaryBytes;

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl<'b, 'k, 'd> Bytes<'b> {
    /// # Generics & Lifetimes
    ///
    /// * `V` generic represents the user's value type, for example: `User`, `String`, etc.
    ///
    /// * 'k' lifetime represents a key potentially being borrowed from a `KeyRing` or
    ///   `KeyProvider`.
    ///
    /// * 'd' lifetime represents a dictionary potentially being borrowed from a `Dictionary` or
    ///   `DictionaryProvider`.
    #[cfg(feature = "compress-dictionaries")]
    pub fn apply_read_layers<V>(
        value_buf: Self,
        key: KeyBytes<'k>,
        dictionary: Option<DictionaryBytes<'d>>
    ) -> Result<ValueOrBytes<'b, V>, Error>
    where V:
        Correctable +
        Encryptable +
        Compressible +
        Serializer::<'b, V> + Serializable,
    {
        let value_or_bytes = value_buf
            .recover::<V>()?
            .decrypt::<V>(key)?
            .decompress::<V>(dictionary)?
            .deserialize::<V>()?;

        Ok(value_or_bytes)
    }

    /// # Generics & Lifetimes
    ///
    /// * `V` generic represents the user's value type, for example: `User`, `String`, etc.
    ///
    /// * 'k' lifetime represents a key potentially being borrowed from a `KeyRing` or
    ///   `KeyProvider`.
    #[cfg(not(feature = "compress-dictionaries"))]
    pub fn apply_read_layers<V>(
        value_buf: Self,
        key: KeyBytes<'k>,
    ) -> Result<ValueOrBytes<'b, V>, Error>
    where V:
        Correctable +
        Encryptable +
        Compressible +
        Serializer::<'b, V> + Serializable,
    {
        let value_or_bytes = value_buf
            .recover::<V>()?
            .decrypt::<V>(key)?
            .decompress::<V>()?
            .deserialize::<V>()?;

        Ok(value_or_bytes)
    }
}