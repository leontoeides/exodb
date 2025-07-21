//! An encryption key is a string of characters or bytes used to lock (encrypt) or unlock (decrypt)
//! data, keeping it secure from unauthorized access

// Exports

mod error;
pub use crate::layers::encryptors::core::key_bytes::error::Error;

// Imports

use crate::layers::encryptors::impls::KEY_SIZE;
use std::borrow::Cow;

// -------------------------------------------------------------------------------------------------
//
/// An encryption key is a series of bytes used to lock (encrypt) or unlock (decrypt) data, keeping
/// it secure from unauthorized access.
///
/// This structure represents a previously validated key originating from a `KeyRing` struct or
/// `KeyRingProvider` trait.
pub struct KeyBytes<'k>(Cow<'k, [u8; KEY_SIZE]>);

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl<'k> KeyBytes<'k> {
    /// Converts a borrowed immutable `&[u8]` slice of bytes into a `KeyBytes` type.
    ///
    /// # Notes
    ///
    /// * When a key is provided in slice form, it will be sent directly to the encryption backend
    ///   with no additional processing. Ensure your key is properly hashed and the correct size for
    ///   the encryption backend you've chosen.
    ///
    /// # Errors
    /// This conversion can fail if:
    /// * The provided slice is not `KEY_SIZE` length.
    #[inline]
    pub fn try_from_slice(slice: &'k [u8]) -> Result<Self, Error> {
        slice.try_into()
    }

    /// Converts a borrowed immutable `&[u8; KEY_SIZE]` array of bytes into a `KeyBytes` type.
    #[inline]
    #[must_use]
    pub fn from_array(fixed_array: &'k [u8; KEY_SIZE]) -> Self {
        fixed_array.into()
    }

    /// Converts an owned `Vec<u8>` collection of bytes into a `KeyBytes` type.
    ///
    /// # Errors
    /// This conversion can fail if:
    /// * The provided slice is not `KEY_SIZE` length.
    #[inline]
    pub fn try_from_vec(owned_vec: Vec<u8>) -> Result<Self, Error> {
        owned_vec.try_into()
    }
}

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl AsRef<[u8; KEY_SIZE]> for KeyBytes<'_> {
    /// Returns a reference to the bytes in the buffer. Does not allocate.
    #[inline]
    fn as_ref(&self) -> &[u8; KEY_SIZE] {
        self.0.as_ref()
    }
}

impl std::ops::Deref for KeyBytes<'_> {
    type Target = [u8; KEY_SIZE];

    /// Returns a reference to the bytes in the buffer. Does not allocate.
    #[inline]
    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

// Array Conversions

impl<'k> From<&'k [u8; KEY_SIZE]> for KeyBytes<'k> {
    /// Converts a borrowed immutable `&[u8; KEY_SIZE]` fixed array of bytes into a `KeyBytes` type.
    #[inline]
    fn from(borrowed_fixed_array: &'k [u8; KEY_SIZE]) -> Self {
        KeyBytes(Cow::Borrowed(borrowed_fixed_array))
    }
}

impl<'k> From<&'k mut [u8; KEY_SIZE]> for KeyBytes<'k> {
    /// Converts a borrowed mutable `&[u8; KEY_SIZE]` fixed array of bytes into a `KeyBytes` type.
    #[inline]
    fn from(borrowed_fixed_array: &'k mut [u8; KEY_SIZE]) -> Self {
        KeyBytes(Cow::Borrowed(&*borrowed_fixed_array))
    }
}

impl From<[u8; KEY_SIZE]> for KeyBytes<'_> {
    /// Converts an owned `[u8; KEY_SIZE]` fixed array of bytes into a `KeyBytes` type.
    #[inline]
    fn from(owned_fixed_array: [u8; KEY_SIZE]) -> Self {
        KeyBytes(Cow::Owned(owned_fixed_array))
    }
}

// Slice Conversions

impl<'k> TryFrom<&'k [u8]> for KeyBytes<'k> {
    type Error = Error;

    /// Converts a borrowed immutable `&[u8]` slice of bytes into a `KeyBytes` type.
    ///
    /// # Errors
    /// This conversion can fail if:
    /// * The provided slice is not `KEY_SIZE` length.
    fn try_from(borrowed_slice_of_bytes: &'k [u8]) -> Result<Self, Self::Error> {
        let fixed_array: [u8; KEY_SIZE] = borrowed_slice_of_bytes
            .try_into()
            .map_err(|_| Error::InvalidKeyLength {
                expected_size: KEY_SIZE,
                provided_size: borrowed_slice_of_bytes.len()
            })?;

        Ok(KeyBytes(Cow::Owned(fixed_array)))
    }
}

impl<'k> TryFrom<&'k mut [u8]> for KeyBytes<'k> {
    type Error = Error;

    /// Converts a borrowed mutable `&mut [u8]` slice of bytes into a `KeyBytes` type.
    ///
    /// # Errors
    /// This conversion can fail if:
    /// * The provided slice is not `KEY_SIZE` length.
    #[inline]
    fn try_from(borrowed_slice_of_bytes: &'k mut [u8]) -> Result<Self, Self::Error> {
        KeyBytes::try_from(&*borrowed_slice_of_bytes)
    }
}

// Cow Conversions

impl<'k> From<&'k Cow<'k, [u8; KEY_SIZE]>> for KeyBytes<'k> {
    /// Converts a borrowed immutable `&Cow<'k, [u8]; KEY_SIZE>` collection of bytes into a `KeyBytes`
    /// type.
    #[inline]
    fn from(borrowed_clone_on_write: &'k Cow<'k, [u8; KEY_SIZE]>) -> Self {
        KeyBytes(match borrowed_clone_on_write {
            Cow::Borrowed(fixed_array_ref) => Cow::Borrowed(fixed_array_ref),
            Cow::Owned(fixed_array) => Cow::Borrowed(fixed_array)
        })
    }
}

impl<'k> From<&'k mut Cow<'k, [u8; KEY_SIZE]>> for KeyBytes<'k> {
    /// Converts a borrowed mutable `&mut Cow<'k, [u8]; KEY_SIZE>` collection of bytes into a `KeyBytes`
    /// type.
    #[inline]
    fn from(borrowed_clone_on_write: &'k mut Cow<'k, [u8; KEY_SIZE]>) -> Self {
        KeyBytes(match borrowed_clone_on_write {
            Cow::Borrowed(fixed_array_ref) => Cow::Borrowed(fixed_array_ref),
            Cow::Owned(fixed_array) => Cow::Borrowed(fixed_array)
        })
    }
}

impl<'k> From<Cow<'k, [u8; KEY_SIZE]>> for KeyBytes<'k> {
    /// Converts an owned `Cow<'k, [u8; KEY_SIZE]>` collection of bytes into a `KeyBytes` type.
    #[inline]
    fn from(owned_clone_on_write: Cow<'k, [u8; KEY_SIZE]>) -> Self {
        KeyBytes(owned_clone_on_write)
    }
}

// Vec Conversions

impl<'k> TryFrom<&'k Vec<u8>> for KeyBytes<'k> {
    type Error = Error;

    /// Converts a borrowed immutable `&Vec<u8>` collection of bytes into a `KeyBytes` type.
    ///
    /// # Errors
    /// This conversion can fail if:
    /// * The provided `Vec` is not `KEY_SIZE` length.
    fn try_from(borrowed_vec_of_bytes: &'k Vec<u8>) -> Result<Self, Self::Error> {
        let fixed_array: [u8; KEY_SIZE] = borrowed_vec_of_bytes
            .as_slice()
            .try_into()
            .map_err(|_| Error::InvalidKeyLength {
                expected_size: KEY_SIZE,
                provided_size: borrowed_vec_of_bytes.len()
            })?;

        Ok(KeyBytes(Cow::Owned(fixed_array)))
    }
}

impl<'k> TryFrom<&'k mut Vec<u8>> for KeyBytes<'k> {
    type Error = Error;

    /// Converts a borrowed mutable `&Vec<u8>` collection of bytes into a `KeyBytes` type.
    ///
    /// # Errors
    /// This conversion can fail if:
    /// * The provided `Vec` is not `KEY_SIZE` length.
    #[inline]
    fn try_from(borrowed_vec_of_bytes: &'k mut Vec<u8>) -> Result<Self, Self::Error> {
        KeyBytes::try_from(&*borrowed_vec_of_bytes)
    }
}

impl TryFrom<Vec<u8>> for KeyBytes<'_> {
    type Error = Error;

    /// Converts an owned `Vec<u8>` collection of bytes into a `KeyBytes` type.
    ///
    /// # Errors
    /// This conversion can fail if:
    /// * The provided `Vec` is not `KEY_SIZE` length.
    fn try_from(owned_vec_of_bytes: Vec<u8>) -> Result<Self, Self::Error> {
        let fixed_array: [u8; KEY_SIZE] = owned_vec_of_bytes
            .as_slice()
            .try_into()
            .map_err(|_| Error::InvalidKeyLength {
                expected_size: KEY_SIZE,
                provided_size: owned_vec_of_bytes.len()
            })?;

        Ok(KeyBytes(Cow::Owned(fixed_array)))
    }
}