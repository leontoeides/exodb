use crate::layers::{
    Compressible,
    compressors::DictionaryBytes,
    core::Bytes,
    core::ValueOrBytes,
    Correctable,
    Encryptable,
    encryptors::{KeyBytes, Nonce},
    Serializable,
    Serializer,
    core::bytes::Error,
};

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
    pub fn apply_write_layers<V>(
        value_or_bytes: impl Into<ValueOrBytes<'b, V>>,
        key: KeyBytes<'k>,
        nonce: Option<Nonce<'k>>,
        dictionary: Option<DictionaryBytes<'d>>
    ) -> Result<Self, Error>
    where V:
        Serializer::<'b, V> + Serializable +
        Compressible +
        Encryptable +
        Correctable + 'b
    {
        let value_or_bytes: ValueOrBytes<V> = value_or_bytes.into();

        Self::serialize(value_or_bytes)?
            .compress::<V>(dictionary)?
            .encrypt::<V>(key, nonce)?
            .protect::<V>()
    }
}