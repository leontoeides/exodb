//! The `ValueOrBytes` type contains a Rust-native type value or raw bytes.

// Exports

mod error;
pub use crate::layers::core::value_or_bytes::error::Error;

// Imports

use crate::layers::core::{Bytes, Value};
use std::borrow::Cow;

// -------------------------------------------------------------------------------------------------
//
/// Contains a Rust-native typed value or raw bytes.
///
/// This types bridges raw database storage and typed application data. It allows values to flow
/// serialization, compression, encryption, and ECC error correction in both directions (on-read and
/// on-write) while being memory efficient, reducing processing, and memory copies.
///
/// # Generics & Lifetimes
///
/// * `V` generic represents the user's value type, for example: `User`, `Customer`, `String`, etc.
/// * `b` lifetime represents bytes potentially being borrowed from storage or from the host
///   application.
#[derive(Debug)]
pub enum ValueOrBytes<'b, V> {
    /// Raw bytes in the processing pipeline.
    ///
    /// Represents data that hasn't been deserialized yet and is being processed by intermediate
    /// layers, or, data that is meant to be stored or retrieved as binary data.
    Bytes(Bytes<'b>),

    /// A Rust-native typed value ready for application use.
    ///
    /// Contains successfully processed data when reading from storage, or, initial input when
    /// writing to storage.
    Value(Value<'b, V>),
}

// -------------------------------------------------------------------------------------------------

impl<'b, V> ValueOrBytes<'b, V> {
    /// Returns `true` if the data is borrowed.
    pub const fn is_borrowed(&self) -> bool {
        match self {
            Self::Bytes(bytes) => bytes.is_borrowed(),
            Self::Value(value) => value.is_borrowed(),
        }
    }

    /// Returns `true` if the data is owned.
    pub const fn is_owned(&self) -> bool {
        match self {
            Self::Bytes(bytes) => bytes.is_owned(),
            Self::Value(value) => value.is_owned(),
        }
    }

    /// Returns `true` if the data is bytes.
    pub const fn is_bytes(&self) -> bool {
        matches!(self, Self::Bytes(_))
    }

    /// Returns `true` if the data is natively-typed value.
    pub const fn is_value(&self) -> bool {
        matches!(self, Self::Value(_))
    }

    // Conversions: From

    /// Instantiates a `Value` type from an owned `V` type.
    #[must_use] pub fn from_value(value: V) -> Self {
        Self::Value(value.into())
    }

    /// Instantiates a `Value` type from a borrowed `&V` type.
    #[must_use] pub fn from_value_ref(value: &'b V) -> Self {
        Self::Value(value.into())
    }

    /// Instantiates a `Value` type from a borrowed `&[u8]` slice of bytes.
    #[must_use] pub fn from_slice(slice: &'b [u8]) -> Self {
        Self::Bytes(slice.into())
    }

    /// Instantiates a `Value` type from an owned `[u8; N]` array of bytes.
    #[must_use] pub fn from_array<const N: usize>(array: [u8; N]) -> Self {
        Self::Bytes(array.into())
    }

    /// Instantiates a `Value` type from a borrowed `[u8; N]` array of bytes.
    #[must_use] pub fn from_array_ref<const N: usize>(array: &'b [u8; N]) -> Self {
        Self::Bytes(array.into())
    }

    /// Instantiates a `Value` type from an owned `Vec<u8>` collection of bytes.
    #[must_use] pub fn from_vec(vec: Vec<u8>) -> Self {
        Self::Bytes(vec.into())
    }

    /// Instantiates a `Value` type from a borrowed `&Vec<u8>` collection of bytes.
    #[must_use] pub fn from_vec_ref(vec: &'b Vec<u8>) -> Self {
        Self::Bytes(vec.into())
    }

    /// Instantiates a `Bytes` buffer from an owned `Cow` clone-on-write type.
    #[must_use] pub fn from_cow(clone_on_write: Cow<'b, [u8]>) -> Self {
        Self::Bytes(clone_on_write.into())
    }

    // Conversions: From

    /// Attempts to return the `Value` from a `ValueOrBytes` type.
    ///
    /// # Errors
    /// This will fail if `ValueOrBytes` contains buffer of bytes.
    pub fn try_into_value(self) -> Result<Value<'b, V>, Error> {
        match self {
            ValueOrBytes::Value(v) => Ok(v),
            ValueOrBytes::Bytes(_) => Err(Error::ExpectedTypedValueGotBytes),
        }
    }

    /// Attempts to return the `Bytes` from a `ValueOrBytes` type.
    ///
    /// # Errors
    /// This will fail if `ValueOrBytes` contains a typed value.
    pub fn try_into_bytes(self) -> Result<Bytes<'b>, Error> {
        match self {
            ValueOrBytes::Bytes(b) => Ok(b),
            ValueOrBytes::Value(_) => Err(Error::ExpectedBytesGotTypedValue),
        }
    }
}

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

/*
// Conversions: Slices

impl<'b, V> From<&'b [u8]> for ValueOrBytes<'b, V> {
    /// Converts a borrowed immutable `&[u8]` slice of bytes into a `ValueOrBytes` type.
    fn from(borrowed_slice_of_bytes: &'b [u8]) -> Self {
        Self::Bytes(borrowed_slice_of_bytes.into())
    }
}

impl<'b, V> From<&'b mut [u8]> for ValueOrBytes<'b, V> {
    /// Converts a borrowed mutable `&mut [u8]` slice of bytes into a `ValueOrBytes` type.
    fn from(borrowed_slice_of_bytes: &'b mut [u8]) -> Self {
        Self::Bytes(borrowed_slice_of_bytes.into())
    }
}

impl<'b, V> From<&'b str> for ValueOrBytes<'b, V> {
    /// Converts a borrowed `&str` string slice into a `ValueOrBytes` type.
    fn from(string: &'b str) -> Self {
        Self::Bytes(string.into())
    }
}

// Conversions: Arrays

impl<const N: usize, V> From<[u8; N]> for ValueOrBytes<'_, V> {
    /// Converts an owned array of bytes into a `ValueOrBytes` type.
    fn from(owned_array_of_bytes: [u8; N]) -> Self {
        Self::Bytes(owned_array_of_bytes.into())
    }
}

impl<'b, const N: usize, V> From<&'b [u8; N]> for ValueOrBytes<'b, V> {
    /// Converts a borrowed array of bytes into a `ValueOrBytes` type.
    fn from(borrowed_array_of_bytes: &'b [u8; N]) -> Self {
        Self::Bytes(borrowed_array_of_bytes.into())
    }
}

impl<'b, const N: usize, V> From<&'b mut [u8; N]> for ValueOrBytes<'b, V> {
    /// Converts a mutable borrowed array of bytes into a `ValueOrBytes` type.
    fn from(borrowed_array_of_bytes: &'b mut [u8; N]) -> Self {
        Self::Bytes(borrowed_array_of_bytes.into())
    }
}

// Conversions: Cows

impl<'b, V> From<&'b Cow<'b, [u8]>> for ValueOrBytes<'b, V> {
    /// Converts an borrowed immutable `&Cow<'b, [u8]>` collection of bytes into a `ValueOrBytes`
    /// type.
    fn from(borrowed_clone_on_write: &'b Cow<'b, [u8]>) -> Self {
        Self::Bytes(borrowed_clone_on_write.into())
    }
}

impl<'b, V> From<&'b mut Cow<'b, [u8]>> for ValueOrBytes<'b, V> {
    /// Converts a borrowed mutable `&mut Cow<'b, [u8]>` collection of bytes into a `ValueOrBytes`
    /// type.
    fn from(borrowed_clone_on_write: &'b mut Cow<'b, [u8]>) -> Self {
        Self::Bytes(borrowed_clone_on_write.into())
    }
}

impl<'b, V> From<Cow<'b, [u8]>> for ValueOrBytes<'b, V> {
    /// Converts an owned `Cow<'b, [u8]>` collection of bytes into a `ValueOrBytes` type.
    fn from(owned_clone_on_write: Cow<'b, [u8]>) -> Self {
        Self::Bytes(owned_clone_on_write.into())
    }
}

// Conversions: Vecs

impl<'b, V> From<&'b Vec<u8>> for ValueOrBytes<'b, V> {
    /// Converts a borrowed immutable `&Vec<u8>` collection of bytes into a `ValueOrBytes` type.
    fn from(borrowed_vec_of_bytes: &'b Vec<u8>) -> Self {
        Self::Bytes(borrowed_vec_of_bytes.into())
    }
}

impl<'b, V> From<&'b mut Vec<u8>> for ValueOrBytes<'b, V> {
    /// Converts a borrowed mutable `&Vec<u8>` collection of bytes into a `ValueOrBytes` type.
    fn from(borrowed_vec_of_bytes: &'b mut Vec<u8>) -> Self {
        Self::Bytes(borrowed_vec_of_bytes.into())
    }
}

impl<V> From<Vec<u8>> for ValueOrBytes<'_, V> {
    /// Converts an owned `Vec<u8>` collection of bytes into a `ValueOrBytes` type.
    fn from(owned_vec_of_bytes: Vec<u8>) -> Self {
        Self::Bytes(owned_vec_of_bytes.into())
    }
}

impl<V> std::convert::From<String> for ValueOrBytes<'_, V> {
    /// Converts an owned `String` into a `ValueOrBytes` type.
        fn from(string: String) -> Self {
        Self::Bytes(string.into())
    }
}
*/

// Conversions: Other

impl<'b, V> std::convert::From<Bytes<'b>> for ValueOrBytes<'b, V> {
    /// Converts an owned `Bytes` bytes buffer into a `ValueOrBytes` type.
    fn from(bytes_buffer: Bytes<'b>) -> Self {
        Self::Bytes(bytes_buffer)
    }
}

impl<'b, V> std::convert::From<&'b Bytes<'b>> for ValueOrBytes<'b, V> {
    /// Converts an borrowed `&Bytes` bytes buffer into a `ValueOrBytes` type.
    fn from(bytes_buffer: &'b Bytes<'b>) -> Self {
        Self::Bytes(bytes_buffer.as_ref().into())
    }
}

impl<'b, V> std::convert::From<Value<'b, V>> for ValueOrBytes<'b, V> {
    /// Converts an owned `Value` typed value into a `ValueOrBytes` type.
    fn from(value: Value<'b, V>) -> Self {
        Self::Value(value)
    }
}

impl<'b, V> std::convert::From<&'b Value<'b, V>> for ValueOrBytes<'b, V> {
    /// Converts an borrowed `&Value` typed value into a `ValueOrBytes` type.
    fn from(value: &'b Value<'b, V>) -> Self {
        Self::Value(value.as_ref().into())
    }
}

use crate::layers::Serializer;

impl<'b, V: Serializer<'b, V>> std::convert::From<V> for ValueOrBytes<'b, V> {
    /// Converts an owned `Bytes` typed value into a `ValueOrBytes` type.
    fn from(value: V) -> Self {
        Self::Value(value.into())
    }
}

impl<'b, V: Serializer<'b, V>> std::convert::From<&'b V> for ValueOrBytes<'b, V> {
    /// Converts an borrowed `&V` typed value into a `ValueOrBytes` type.
    fn from(value: &'b V) -> Self {
        Self::Value(value.into())
    }
}