use crate::layers::core::{bytes::Error, Bytes, Value, ValueOrBytes};
use crate::layers::{Serializable, Serializer};

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl<'b> Bytes<'b> {
    /// Serializes the value into a binary byte vector suitable for storage.
    ///
    /// # Errors
    ///
    /// Consult the documentation of the serializer backend you are using for more detail on
    /// serialization behavior and potential limitations.
    ///
    /// # Generics & Lifetimes
    ///
    /// * `V` generic represents the user's value type, for example: `User`, `String`, etc.
    /// * `b` lifetime represents bytes potentially being borrowed from the `redb` database.
    #[inline]
    pub fn serialize<V: Serializer::<'b, V> + Serializable + 'b>(
        value_or_bytes: ValueOrBytes<'b, V>
    ) -> Result<Self, Error> {
        if V::DIRECTION.is_write() {
            match value_or_bytes.try_into_value()? {
                Value::Borrowed(value_ref) => Ok(<V>::serialize_ref(value_ref)?),
                Value::Owned(value) => Ok(<V>::serialize(value)?),
            }
        } else {
            Ok(value_or_bytes.try_into_bytes()?)
        }
    }

    /// Reduces data size by eliminating redundancy creating smaller representations of bytes.
    ///
    /// # Errors
    ///
    /// Consult the documentation of the compressor backend you are using for more detail on
    /// compression behavior and potential limitations.
    #[inline]
    pub fn deserialize<V: Serializer::<'b, V> + Serializable>(
        self
    ) -> Result<ValueOrBytes<'b, V>, Error> {
        if V::DIRECTION.is_read() {
            Ok(V::deserialize(self)?.into())
        } else {
            Ok(self.into())
        }
    }
}