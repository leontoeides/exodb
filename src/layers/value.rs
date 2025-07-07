use crate::layers::ValueBuf;
use std::borrow::Cow;

// -------------------------------------------------------------------------------------------------
//
/// Represents the final result of processing layered data (serialized, encrypted, compressed,
/// error corrected, etc.).
///
/// This enum encapsulates three possible outcomes of the processing pipeline:
/// * Borrowed bytes · When processing can be done without allocation, preserving the original
///   borrowed slice from `redb`.
/// * Owned bytes · When processing requires allocation (e.g., decompression, decryption)
/// * Deserialized value · When deserialization succeeds, producing a typed value
///
/// The lifetime parameter `'b` represents the lifetime of the original borrowed data, while `V` is
/// the target deserialization type.
#[derive(Debug)]
pub enum Value<'b, V> {
    /// Raw bytes that may be borrowed (zero-copy) or owned (allocated during processing).
    ///
    /// This variant is used when:
    /// * Deserialization hasn't been attempted yet
    /// * Deserialization failed but the bytes are still valid
    /// * The consumer wants to work with raw bytes directly
    Bytes(Cow<'b, [u8]>),

    /// A successfully deserialized value of type `V`.
    ///
    /// This variant indicates that all processing layers have been successfully applied and the
    /// data has been deserialized into the target type.
    Value(V),
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl<'b, V> Value<'b, V> {
    /// Returns `true` if this contains borrowed bytes (zero-copy).
    pub fn is_slice(&self) -> bool {
        matches!(self, Self::Bytes(Cow::Borrowed(_)))
    }

    /// Returns `true` if this contains owned bytes (allocated during processing).
    pub fn is_vec(&self) -> bool {
        matches!(self, Self::Bytes(Cow::Owned(_)))
    }

    /// Returns `true` if this contains a deserialized value.
    pub fn is_value(&self) -> bool {
        matches!(self, Self::Value(_))
    }

    /// Returns `true` if this contains any form of bytes (borrowed or owned).
    pub fn is_bytes(&self) -> bool {
        matches!(self, Self::Bytes(_))
    }

    /// Attempts to extract a borrowed slice, consuming the `Value`.
    ///
    /// # Returns
    /// * `Ok(&'b [u8])` if this contains borrowed bytes
    /// * `Err(Self)` if this contains owned bytes or a deserialized value
    ///
    /// # Examples
    /// ```
    /// # use std::borrow::Cow;
    /// # use atlatl::layers::Value;
    /// let value: Value<'_, &str> = Value::Bytes(Cow::Borrowed(b"hello"));
    /// assert!(value.as_slice().is_ok());
    /// ```
    pub fn as_slice(self) -> Result<&'b [u8], Self> {
        match self {
            Value::Bytes(Cow::Borrowed(slice)) => Ok(slice),
            other => Err(other),
        }
    }

    /// Attempts to extract an owned vector, consuming the `Value`.
    ///
    /// # Returns
    /// * `Ok(Vec<u8>)` if this contains owned bytes
    /// * `Err(Self)` if this contains borrowed bytes or a deserialized value
    ///
    /// # Examples
    /// ```
    /// # use std::borrow::Cow;
    /// # use atlatl::layers::Value;
    /// let value: Value<'_, &str> = Value::Bytes(Cow::Owned(vec![1, 2, 3]));
    /// assert!(value.into_vec().is_ok());
    /// ```
    pub fn into_vec(self) -> Result<Vec<u8>, Self> {
        match self {
            Value::Bytes(Cow::Owned(vec)) => Ok(vec),
            other => Err(other),
        }
    }

    /// Attempts to extract the deserialized value, consuming the `Value`.
    ///
    /// # Returns
    /// * `Ok(V)` if this contains a deserialized value
    /// * `Err(Self)` if this contains bytes
    ///
    /// # Examples
    /// ```
    /// # use atlatl::layers::Value;
    /// let value: Value<String> = Value::Value("hello".to_string());
    /// assert!(value.into_value().is_ok());
    /// ```
    pub fn into_value(self) -> Result<V, Self> {
        match self {
            Value::Value(value) => Ok(value),
            other => Err(other),
        }
    }

    /// Attempts to extract any bytes (borrowed or owned), consuming the `Value`.
    ///
    /// # Returns
    /// * `Ok(Cow<'b, [u8]>)` if this contains bytes
    /// * `Err(Self)` if this contains a deserialized value
    pub fn into_bytes(self) -> Result<Cow<'b, [u8]>, Self> {
        match self {
            Value::Bytes(cow) => Ok(cow),
            other => Err(other),
        }
    }

    /// Converts owned bytes to borrowed if possible, or clones borrowed bytes to owned.
    ///
    /// This is useful when you need to change the ownership model of the bytes.
    pub fn into_owned_bytes(self) -> Result<Vec<u8>, Self> {
        match self {
            Value::Bytes(cow) => Ok(cow.into_owned()),
            other => Err(other),
        }
    }

    /// Returns a reference to the bytes without consuming the `Value`.
    ///
    /// # Returns
    /// * `Some(&[u8])` if this contains bytes
    /// * `None` if this contains a deserialized value
    pub fn as_bytes(&self) -> Option<&[u8]> {
        match self {
            Value::Bytes(cow) => Some(cow.as_ref()),
            Value::Value(_) => None,
        }
    }

    /// Maps the deserialized value type `V` to `U` using the provided function.
    ///
    /// This is useful for transforming the contained value without affecting bytes.
    pub fn map_value<U, F>(self, f: F) -> Value<'b, U>
    where
        F: FnOnce(V) -> U,
    {
        match self {
            Value::Bytes(cow) => Value::Bytes(cow),
            Value::Value(value) => Value::Value(f(value)),
        }
    }
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl<'b, V: crate::layers::Serializer::<'b, V>> Value<'b, V> {
    pub fn deserialize(self) -> Result<V, crate::layers::serializers::DeserializeError> {
        match self {
            Value::Bytes(cow) => Ok(V::deserialize(cow.as_ref().into())?),
            Value::Value(value) => Ok(value),
        }
    }
}

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl<'b, V> From<&'b [u8]> for Value<'b, V> {
    fn from(slice: &'b [u8]) -> Self {
        Value::Bytes(Cow::Borrowed(slice))
    }
}

impl<'b, V> From<Cow<'b, [u8]>> for Value<'b, V> {
    fn from(clone_on_write: Cow<'b, [u8]>) -> Self {
        Value::Bytes(clone_on_write)
    }
}

impl<'b, V> From<ValueBuf<'b>> for Value<'b, V> {
    fn from(value_buf: ValueBuf<'b>) -> Self {
        Value::Bytes(value_buf.data)
    }
}

impl<'b, V> From<Vec<u8>> for Value<'b, V> {
    fn from(vec: Vec<u8>) -> Self {
        Value::Bytes(Cow::Owned(vec))
    }
}