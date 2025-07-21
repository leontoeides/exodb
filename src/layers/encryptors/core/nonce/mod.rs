//! A nonce in encryption is a unique number that used to help prevent replay attacks.

// Exports

mod error;
pub use crate::layers::encryptors::core::nonce::error::Error;

// Imports

use crate::layers::encryptors::impls::NONCE_SIZE;
use std::borrow::Cow;

// -------------------------------------------------------------------------------------------------
//
/// A nonce in encryption is a unique, random or pseudo-random number used only once to ensure
/// security by preventing replay attacks and that identical plaintexts produce different
/// ciphertexts.
///
/// # Notes
///
/// * This structure attempts to take ownership of the nonce.
pub struct Nonce<'k>(Cow<'k, [u8; NONCE_SIZE]>);

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl<'k> Nonce<'k> {
    /// Converts a borrowed immutable `&[u8; NONCE_SIZE]` array of bytes into a `Nonce` type.
    #[inline]
    #[must_use]
    pub fn from_array(fixed_array: [u8; NONCE_SIZE]) -> Self {
        fixed_array.into()
    }

    /// Converts a borrowed immutable `&[u8]` slice of bytes into a `Nonce` type.
    ///
    /// # Errors
    ///
    /// This conversion can fail if:
    ///
    /// * The provided slice is not `NONCE_SIZE` length.
    #[inline]
    pub fn try_from_slice(slice: &'k [u8]) -> Result<Self, Error> {
        slice.try_into()
    }

    /// Converts an owned `Vec<u8>` collection of bytes into a `Nonce` type.
    ///
    /// # Errors
    ///
    /// This conversion can fail if:
    ///
    /// * The provided `Vec` is not `NONCE_SIZE` length.
    #[inline]
    pub fn try_from_vec(owned_vec: Vec<u8>) -> Result<Self, Error> {
        owned_vec.try_into()
    }

    /// Converts a `Nonce` into an owned fixed-length `[u8; NONCE_SIZE]` array of bytes.
    #[inline]
    #[must_use]
    pub fn into_bytes(self) -> [u8; NONCE_SIZE] {
        self.into()
    }
}

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl AsRef<[u8; NONCE_SIZE]> for Nonce<'_> {
    /// Returns a reference to the bytes in the buffer. Does not allocate.
    #[inline]
    fn as_ref(&self) -> &[u8; NONCE_SIZE] {
        self.0.as_ref()
    }
}

impl std::ops::Deref for Nonce<'_> {
    type Target = [u8; NONCE_SIZE];

    /// Returns a reference to the bytes in the buffer. Does not allocate.
    #[inline]
    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

// Array Conversions

impl<'k> From<&'k [u8; NONCE_SIZE]> for Nonce<'k> {
    /// Converts a borrowed immutable `&[u8; NONCE_SIZE]` fixed array of bytes into a `Nonce` type.
    #[inline]
    fn from(borrowed_fixed_array: &'k [u8; NONCE_SIZE]) -> Self {
        Nonce(Cow::Borrowed(borrowed_fixed_array))
    }
}

impl<'k> From<&'k mut [u8; NONCE_SIZE]> for Nonce<'k> {
    /// Converts a borrowed mutable `&[u8; NONCE_SIZE]` fixed array of bytes into a `Nonce` type.
    #[inline]
    fn from(borrowed_fixed_array: &'k mut [u8; NONCE_SIZE]) -> Self {
        Nonce(Cow::Borrowed(&*borrowed_fixed_array))
    }
}

impl From<[u8; NONCE_SIZE]> for Nonce<'_> {
    /// Converts an owned `[u8; NONCE_SIZE]` fixed array of bytes into a `Nonce` type.
    #[inline]
    fn from(owned_fixed_array: [u8; NONCE_SIZE]) -> Self {
        Nonce(Cow::Owned(owned_fixed_array))
    }
}

// Slice Conversions

impl<'k> TryFrom<&'k [u8]> for Nonce<'k> {
    type Error = Error;

    /// Converts a borrowed immutable `&[u8]` slice of bytes into a `Nonce` type.
    ///
    /// # Errors
    /// This conversion can fail if:
    /// * The provided slice is not `NONCE_SIZE` length.
    fn try_from(borrowed_slice_of_bytes: &'k [u8]) -> Result<Self, Self::Error> {
        let fixed_array: [u8; NONCE_SIZE] = borrowed_slice_of_bytes
            .try_into()
            .map_err(|_| Error::InvalidNonceLength {
                expected_size: NONCE_SIZE,
                provided_size: borrowed_slice_of_bytes.len()
            })?;

        Ok(Nonce(Cow::Owned(fixed_array)))
    }
}

impl<'k> TryFrom<&'k mut [u8]> for Nonce<'k> {
    type Error = Error;

    /// Converts a borrowed mutable `&mut [u8]` slice of bytes into a `Nonce` type.
    ///
    /// # Errors
    /// This conversion can fail if:
    /// * The provided slice is not `NONCE_SIZE` length.
    #[inline]
    fn try_from(borrowed_slice_of_bytes: &'k mut [u8]) -> Result<Self, Self::Error> {
        Nonce::try_from(&*borrowed_slice_of_bytes)
    }
}

// Cow Conversions

impl<'k> From<&'k Cow<'k, [u8; NONCE_SIZE]>> for Nonce<'k> {
    /// Converts a borrowed immutable `&Cow<'k, [u8]; NONCE_SIZE>` collection of bytes into a `Nonce`
    /// type.
    #[inline]
    fn from(borrowed_clone_on_write: &'k Cow<'k, [u8; NONCE_SIZE]>) -> Self {
        Nonce(match borrowed_clone_on_write {
            Cow::Borrowed(fixed_array_ref) => Cow::Borrowed(fixed_array_ref),
            Cow::Owned(fixed_array) => Cow::Borrowed(fixed_array)
        })
    }
}

impl<'k> From<&'k mut Cow<'k, [u8; NONCE_SIZE]>> for Nonce<'k> {
    /// Converts a borrowed mutable `&mut Cow<'k, [u8]; NONCE_SIZE>` collection of bytes into a `Nonce`
    /// type.
    #[inline]
    fn from(borrowed_clone_on_write: &'k mut Cow<'k, [u8; NONCE_SIZE]>) -> Self {
        Nonce(match borrowed_clone_on_write {
            Cow::Borrowed(fixed_array_ref) => Cow::Borrowed(fixed_array_ref),
            Cow::Owned(fixed_array) => Cow::Borrowed(fixed_array)
        })
    }
}

impl<'k> From<Cow<'k, [u8; NONCE_SIZE]>> for Nonce<'k> {
    /// Converts an owned `Cow<'k, [u8; NONCE_SIZE]>` collection of bytes into a `Nonce` type.
    #[inline]
    fn from(owned_clone_on_write: Cow<'k, [u8; NONCE_SIZE]>) -> Self {
        Nonce(owned_clone_on_write)
    }
}

// Vec Conversions

impl<'k> TryFrom<&'k Vec<u8>> for Nonce<'k> {
    type Error = Error;

    /// Converts a borrowed immutable `&Vec<u8>` collection of bytes into a `Nonce` type.
    ///
    /// # Errors
    /// This conversion can fail if:
    /// * The provided `Vec` is not `NONCE_SIZE` length.
    fn try_from(borrowed_vec_of_bytes: &'k Vec<u8>) -> Result<Self, Self::Error> {
        let fixed_array: [u8; NONCE_SIZE] = borrowed_vec_of_bytes
            .as_slice()
            .try_into()
            .map_err(|_| Error::InvalidNonceLength {
                expected_size: NONCE_SIZE,
                provided_size: borrowed_vec_of_bytes.len()
            })?;

        Ok(Nonce(Cow::Owned(fixed_array)))
    }
}

impl<'k> TryFrom<&'k mut Vec<u8>> for Nonce<'k> {
    type Error = Error;

    /// Converts a borrowed mutable `&Vec<u8>` collection of bytes into a `Nonce` type.
    ///
    /// # Errors
    /// This conversion can fail if:
    /// * The provided `Vec` is not `NONCE_SIZE` length.
    #[inline]
    fn try_from(borrowed_vec_of_bytes: &'k mut Vec<u8>) -> Result<Self, Self::Error> {
        Nonce::try_from(&*borrowed_vec_of_bytes)
    }
}

impl TryFrom<Vec<u8>> for Nonce<'_> {
    type Error = Error;

    /// Converts an owned `Vec<u8>` collection of bytes into a `Nonce` type.
    ///
    /// # Errors
    /// This conversion can fail if:
    /// * The provided `Vec` is not `NONCE_SIZE` length.
    fn try_from(owned_vec_of_bytes: Vec<u8>) -> Result<Self, Self::Error> {
        let fixed_array: [u8; NONCE_SIZE] = owned_vec_of_bytes
            .as_slice()
            .try_into()
            .map_err(|_| Error::InvalidNonceLength {
                expected_size: NONCE_SIZE,
                provided_size: owned_vec_of_bytes.len()
            })?;

        Ok(Nonce(Cow::Owned(fixed_array)))
    }
}

// Other Conversions

impl<'k> From<Nonce<'k>> for [u8; NONCE_SIZE] {
    /// Converts an owned `Nonce` into an owned fixed array of bytes.
    #[inline]
    fn from(nonce: Nonce<'k>) -> Self {
        match nonce.0 {
        	Cow::Borrowed(fixed_array) => *fixed_array,
			Cow::Owned(fixed_array) => fixed_array,
        }
    }
}
