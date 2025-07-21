//! An encryption key is a string of characters or bytes used to lock (encrypt) or unlock (decrypt)
//! data, keeping it secure from unauthorized access

use crate::layers::encryptors::{kdf::Error, KEY_SIZE};
use std::borrow::Cow;

// -------------------------------------------------------------------------------------------------
//
/// An encryption key is a string of characters or series of bytes used to lock (encrypt) or unlock
/// (decrypt) data, keeping it secure from unauthorized access
///
/// # Notes
///
/// * When a key is provided in string form, it will be lazily hashed into a key when encryption or
///   decryption is applied. This process does use resources, so it's preferred to provide a key as
///   `[u8; KEY_SIZE]` fixed-sized array of bytes whenever possible.
///
/// * When a key is provided in bytes form, it will be sent directly to the encryption backend with
///   no additional processing. Ensure your key is properly hashed and the correct size for the
///   encryption backend you've chosen.
pub enum Key<'k> {
    /// A key that was provided in string format.
    ///
    /// This string will be lazily hashed when encryption or decryption is applied. The conversion
    /// process into a hashed key does use resources, so it's preferrable to provide the key as a
    /// `[u8; KEY_SIZE]` fixed-sized array of bytes whenever it's possible.
	String(Cow<'k, str>),

    /// A key that was provided as an fixed-length array of bytes.
    ///
    /// This key is able to be sent directly to the encryption backend with no additional
    /// processing. Ensure your key is properly hashed and the correct size for the encryption
    /// backend you've chosen.
    Bytes(Cow<'k, [u8; KEY_SIZE]>)
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl<'k> Key<'k> {
    /// Converts a borrowed immutable `&str` string slice into a `Key` type.
    ///
    /// # Notes
    ///
    /// * When a key is provided in string form, it will be lazily hashed into a key when encryption
    ///   or decryption is applied. This process does use resources, so it's preferred to provide a
    ///   key as `[u8; KEY_SIZE]` fixed-sized array of bytes whenever possible.
    #[inline]
    pub fn from_str(borrowed_str: &'k str) -> Self {
        borrowed_str.into()
    }

    /// Converts a borrowed immutable `&String` string into a `Key` type.
    ///
    /// # Notes
    ///
    /// * When a key is provided in string form, it will be lazily hashed into a key when encryption
    ///   or decryption is applied. This process does use resources, so it's preferred to provide a
    ///   key as `[u8; KEY_SIZE]` fixed-sized array of bytes whenever possible.
    #[inline]
    pub fn from_string(borrowed_string: &'k String) -> Self {
        borrowed_string.into()
    }

    /// Converts a borrowed immutable `&[u8]` slice of bytes into a `Key` type.
    ///
    /// # Notes
    ///
    /// * When a key is provided in slice form, it will be sent directly to the encryption backend
    ///   with no additional processing. Ensure your key is properly hashed and the correct size for
    ///   the encryption backend you've chosen.
    ///
    /// # Errors
    ///
    /// This conversion can fail if:
    ///
    /// * The provided slice is not `KEY_SIZE` length.
    #[inline]
    pub fn try_from_slice(slice: &'k [u8]) -> Result<Self, Error> {
        slice.try_into()
    }

    /// Converts a borrowed immutable `&[u8; KEY_SIZE]` array of bytes into a `Key` type.
    ///
    /// # Notes
    ///
    /// * When a key is provided in array form, it will be sent directly to the encryption backend
    ///   with no additional processing. Ensure your key is properly hashed and the correct size for
    ///   the encryption backend you've chosen.
    #[inline]
    pub fn from_array(fixed_array: &'k [u8; KEY_SIZE]) -> Self {
        fixed_array.into()
    }

    /// Converts an owned `Vec<u8>` collection of bytes into a `Key` type.
    ///
    /// # Notes
    ///
    /// * When a key is provided in `Vec` form, it will be sent directly to the encryption backend
    ///   with no additional processing. Ensure your key is properly hashed and the correct size for
    ///   the encryption backend you've chosen.
    #[inline]
    pub fn from_vec(owned_vec: Vec<u8>) -> Result<Self, Error> {
        owned_vec.try_into()
    }
}

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

// Strings

impl<'k> From<&'k str> for Key<'k> {
    /// Converts a borrowed immutable `&str` string slice into a `Key` type.
    #[inline]
    fn from(borrowed_str: &'k str) -> Self {
        Key::String(Cow::Borrowed(borrowed_str))
    }
}

impl<'k> From<&'k mut str> for Key<'k> {
    /// Converts a borrowed mutable `&mut str` string slice into a `Key` type.
    #[inline]
    fn from(borrowed_str: &'k mut str) -> Self {
        Key::String(Cow::Borrowed(&*borrowed_str))
    }
}

impl<'k> From<&'k String> for Key<'k> {
    /// Converts a borrowed immutable `&String` string into a `Key` type.
    #[inline]
    fn from(borrowed_string: &'k String) -> Self {
        Key::String(Cow::Borrowed(borrowed_string))
    }
}

impl<'k> From<&'k mut String> for Key<'k> {
    /// Converts a borrowed mutable `&mut String` string into a `Key` type.
    #[inline]
    fn from(borrowed_string: &'k mut String) -> Self {
        Key::String(Cow::Borrowed(&*borrowed_string))
    }
}

impl<'k> From<String> for Key<'k> {
    /// Converts a owned `String` string into a `Key` type.
    #[inline]
    fn from(string: String) -> Self {
        Key::String(Cow::Owned(string))
    }
}

// Arrays

impl<'k> From<&'k [u8; KEY_SIZE]> for Key<'k> {
    /// Converts a borrowed immutable `&[u8; KEY_SIZE]` fixed array of bytes into a `Key` type.
    #[inline]
    fn from(borrowed_fixed_array: &'k [u8; KEY_SIZE]) -> Self {
        Key::Bytes(Cow::Borrowed(borrowed_fixed_array))
    }
}

impl<'k> From<&'k mut [u8; KEY_SIZE]> for Key<'k> {
    /// Converts a borrowed mutable `&[u8; KEY_SIZE]` fixed array of bytes into a `Key` type.
    #[inline]
    fn from(borrowed_fixed_array: &'k mut [u8; KEY_SIZE]) -> Self {
        Key::Bytes(Cow::Borrowed(&*borrowed_fixed_array))
    }
}

impl<'k> From<[u8; KEY_SIZE]> for Key<'k> {
    /// Converts an owned `[u8; KEY_SIZE]` fixed array of bytes into a `Key` type.
    #[inline]
    fn from(owned_fixed_array: [u8; KEY_SIZE]) -> Self {
        Key::Bytes(Cow::Owned(owned_fixed_array))
    }
}

// Slices

impl<'k> TryFrom<&'k [u8]> for Key<'k> {
    type Error = Error;

    /// Converts a borrowed immutable `&[u8]` slice of bytes into a `Key` type.
    ///
    /// # Errors
    /// This conversion can fail if:
    /// * The provided slice is not `KEY_SIZE` length.
    fn try_from(borrowed_slice_of_bytes: &'k [u8]) -> Result<Self, Self::Error> {
        let fixed_array: [u8; KEY_SIZE] = borrowed_slice_of_bytes
            .try_into()
            .or_else(|_| Err(Error::InvalidKeyLength {
                expected_size: KEY_SIZE,
                provided_size: borrowed_slice_of_bytes.len()
            }))?;

        Ok(Key::Bytes(Cow::Owned(fixed_array)))
    }
}

impl<'k> TryFrom<&'k mut [u8]> for Key<'k> {
    type Error = Error;

    /// Converts a borrowed mutable `&mut [u8]` slice of bytes into a `Key` type.
    ///
    /// # Errors
    /// This conversion can fail if:
    /// * The provided slice is not `KEY_SIZE` length.
    #[inline]
    fn try_from(borrowed_slice_of_bytes: &'k mut [u8]) -> Result<Self, Self::Error> {
        Key::try_from(&*borrowed_slice_of_bytes)
    }
}

// Cows

impl<'k> From<&'k Cow<'k, [u8; KEY_SIZE]>> for Key<'k> {
    /// Converts a borrowed immutable `&Cow<'k, [u8]; KEY_SIZE>` collection of bytes into a `Key`
    /// type.
    #[inline]
    fn from(borrowed_clone_on_write: &'k Cow<'k, [u8; KEY_SIZE]>) -> Key<'k> {
        Key::Bytes(match borrowed_clone_on_write {
            Cow::Borrowed(fixed_array_ref) => Cow::Borrowed(fixed_array_ref),
            Cow::Owned(fixed_array) => Cow::Borrowed(fixed_array)
        })
    }
}

impl<'k> From<&'k mut Cow<'k, [u8; KEY_SIZE]>> for Key<'k> {
    /// Converts a borrowed mutable `&mut Cow<'k, [u8]; KEY_SIZE>` collection of bytes into a `Key`
    /// type.
    #[inline]
    fn from(borrowed_clone_on_write: &'k mut Cow<'k, [u8; KEY_SIZE]>) -> Key<'k> {
        Key::Bytes(match borrowed_clone_on_write {
            Cow::Borrowed(fixed_array_ref) => Cow::Borrowed(fixed_array_ref),
            Cow::Owned(fixed_array) => Cow::Borrowed(fixed_array)
        })
    }
}

impl<'k> From<Cow<'k, [u8; KEY_SIZE]>> for Key<'k> {
    /// Converts an owned `Cow<'k, [u8; KEY_SIZE]>` collection of bytes into a `Key` type.
    #[inline]
    fn from(owned_clone_on_write: Cow<'k, [u8; KEY_SIZE]>) -> Key<'k> {
        Key::Bytes(owned_clone_on_write)
    }
}

// Vecs

impl<'k> TryFrom<&'k Vec<u8>> for Key<'k> {
    type Error = Error;

    /// Converts a borrowed immutable `&Vec<u8>` collection of bytes into a `Key` type.
    ///
    /// # Errors
    /// This conversion can fail if:
    /// * The provided `Vec` is not `KEY_SIZE` length.
    fn try_from(borrowed_vec_of_bytes: &'k Vec<u8>) -> Result<Self, Self::Error> {
        let fixed_array: [u8; KEY_SIZE] = borrowed_vec_of_bytes
            .as_slice()
            .try_into()
            .or_else(|_| Err(Error::InvalidKeyLength {
                expected_size: KEY_SIZE,
                provided_size: borrowed_vec_of_bytes.len()
            }))?;

        Ok(Key::Bytes(Cow::Owned(fixed_array)))
    }
}

impl<'k> TryFrom<&'k mut Vec<u8>> for Key<'k> {
    type Error = Error;

    /// Converts a borrowed mutable `&Vec<u8>` collection of bytes into a `Key` type.
    ///
    /// # Errors
    /// This conversion can fail if:
    /// * The provided `Vec` is not `KEY_SIZE` length.
    #[inline]
    fn try_from(borrowed_vec_of_bytes: &'k mut Vec<u8>) -> Result<Self, Self::Error> {
        Key::try_from(&*borrowed_vec_of_bytes)
    }
}

impl<'k> TryFrom<Vec<u8>> for Key<'k> {
    type Error = Error;

    /// Converts an owned `Vec<u8>` collection of bytes into a `Key` type.
    ///
    /// # Errors
    /// This conversion can fail if:
    /// * The provided `Vec` is not `KEY_SIZE` length.
    fn try_from(owned_vec_of_bytes: Vec<u8>) -> Result<Self, Self::Error> {
        let fixed_array: [u8; KEY_SIZE] = owned_vec_of_bytes
            .as_slice()
            .try_into()
            .or_else(|_| Err(Error::InvalidKeyLength {
                expected_size: KEY_SIZE,
                provided_size: owned_vec_of_bytes.len()
            }))?;

        Ok(Key::Bytes(Cow::Owned(fixed_array)))
    }
}