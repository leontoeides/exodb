use crate::layers::{
    Compressible,
    compressors::DictionaryBytes,
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

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl<'b, 'k, 'd> Bytes<'b> {
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
}