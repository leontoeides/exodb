//! The `Bytes` buffer is buffer for working with bytes that may either be borrowed or owned.

// -------------------------------------------------------------------------------------------------
//
// Exports

pub mod error;
pub use crate::layers::core::bytes::error::Error;

mod metadata;
pub use crate::layers::core::bytes::metadata::Metadata;

#[cfg(feature = "compressors")]
mod compression;

#[cfg(feature = "correctors")]
mod correction;

#[cfg(feature = "encryptors")]
mod encryption;

#[cfg(feature = "serializers")]
mod serialization;

// mod tests;
mod read;
mod write;

// -------------------------------------------------------------------------------------------------
//
// Imports

use std::borrow::Cow;

// -------------------------------------------------------------------------------------------------
//
/// A buffer for working with bytes that may either be borrowed or owned.
///
/// This buffer type helps avoid unnecessary clones when working with borrowed data from the host
/// application or from storage.
///
/// This `Bytes` buffer is based on `Cow` but it supports metadata and provides many additional
/// Atlatl-specific methods that help with processing the layers pipeline.
///
/// # Generics & Lifetimes
///
/// * `V` generic represents the user's value type, for example: `User`, `Customer`, `String`, etc.
/// * `b` lifetime represents bytes potentially being borrowed from storage or from the host
///   application.
#[derive(Clone, Debug, Default)]
pub struct Bytes<'b> {
    pub(crate) metadata: Metadata,
    pub(crate) data: Cow<'b, [u8]>
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl<'b> Bytes<'b> {
    /// Returns `true` if the bytes are borrowed.
    #[must_use] pub const fn is_borrowed(&self) -> bool {
        matches!(self.data, Cow::Borrowed(_))
    }

    /// Returns `true` if the bytes are owned.
    #[must_use] pub const fn is_owned(&self) -> bool {
        matches!(self.data, Cow::Owned(_))
    }

    /// Returns the number of bytes in the bytes buffer.
    #[must_use] pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns `true` if the bytes buffer is empty.
    #[must_use] pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns an immutable slice of the bytes. Does not allocate.
    #[must_use] pub fn as_slice(&'b self) -> &'b [u8] {
        self.data.as_ref()
    }

    /// Unwraps a `Bytes` buffer into the underlying `Cow<[u8]>` bytes, discarding the metadata.
    #[must_use] pub fn into_bytes(self) -> Cow<'b, [u8]> {
        self.into()
    }

    // Conversions: To

    /// Returns a mutable reference to a `&mut [u8]` slice of bytes if the data is owned. This will
    /// return `None` if the data is immutably borrowed.
    ///
    /// This allows in-place modifications (for example, decryption) without allocation when
    /// possible. If `None` is returned, you may need to clone and own the data explicitly for
    /// mutations.
    pub const fn as_mut(&mut self) -> Option<&mut [u8]> {
        match self.data {
            Cow::Owned(ref mut vec) => Some(vec.as_mut_slice()),
            Cow::Borrowed(_) => None,
        }
    }

    /// Returns a mutable reference to a `&mut Vec<u8>` of bytes if the data is owned. This will
    /// return `None` if the data is immutably borrowed.
    ///
    /// This allows in-place modifications (for example, decryption) without allocation when
    /// possible. If `None` is returned, you may need to clone and own the data explicitly for
    /// mutations.
    pub const fn as_mut_vec(&mut self) -> Option<&mut Vec<u8>> {
        match self.data {
            Cow::Owned(ref mut vec) => Some(vec),
            Cow::Borrowed(_) => None,
        }
    }

    /// Destructures the `Bytes` buffer into a tuple, where element `0` is the `Metadata` and
    /// element `1` is a `Cow<[u8]>` that contains the data.
    ///
    /// This is useful when an operation requires unencumbered & owned data, but the metadata needs
    /// to be preserved.
    #[must_use] pub(crate) fn into_parts(self) -> (Metadata, Cow<'b, [u8]>) {
        self.into()
    }

    // Conversions: From

    /// Instantiates a `Value` type from a borrowed `&[u8]` slice of bytes.
    #[must_use] pub fn from_slice(slice: &'b [u8]) -> Self {
        slice.into()
    }

    /// Instantiates a `Value` type from an owned `[u8; N]` array of bytes.
    #[must_use] pub fn from_array<const N: usize>(array: [u8; N]) -> Self {
        array.into()
    }

    /// Instantiates a `Value` type from a borrowed `[u8; N]` array of bytes.
    #[must_use] pub fn from_array_ref<const N: usize>(array: &'b [u8; N]) -> Self {
        array.into()
    }

    /// Instantiates a `Value` type from an owned `Vec<u8>` collection of bytes.
    #[must_use] pub fn from_vec(vec: Vec<u8>) -> Self {
        vec.into()
    }

    /// Instantiates a `Value` type from a borrowed `&Vec<u8>` collection of bytes.
    #[must_use] pub fn from_vec_ref(vec: &'b Vec<u8>) -> Self {
        vec.into()
    }

    /// Instantiates a `Bytes` buffer from an owned `Cow` clone-on-write type.
    #[must_use] pub fn from_cow(clone_on_write: Cow<'b, [u8]>) -> Self {
        clone_on_write.into()
    }

    /// Instantiates a `Bytes` buffer from `Metadata` and `Cow<[u8]>` data parts.
    ///
    /// This is useful when an operation requires unencumbered & owned data, but the metadata needs
    /// to be preserved.
    #[must_use] pub(crate) fn from_parts(metadata: Metadata, data: Cow<'b, [u8]>) -> Self {
        (metadata, data).into()
    }

    /// Instantiates a `Bytes` buffer from an owned `Vec<u8>` and automatically marks the data as
    /// “recovered.”
    #[must_use] pub(crate) fn from_recovered_data(recovered_data: Vec<u8>) -> Self {
        Bytes {
            metadata: Metadata::default(),
            data: recovered_data.into()
        }
    }

    /// Truncates the internal buffer to the specified length, keeping only the first `len` bytes.
    ///
    /// This method modifies the buffer in-place by shortening it to the given length.
    ///
    /// * For borrowed data, it creates a new slice reference to the truncated portion.
    /// * For owned data, it truncates the vector directly.
    ///
    /// The metadata is preserved unchanged.
    ///
    /// If `len` is greater than or equal to the current length, no truncation occurs.
    pub fn truncate(&mut self, len: usize) {
        if len < self.data.len() {
            match &mut self.data {
                Cow::Borrowed(slice) => self.data = Cow::Borrowed(&slice[..len]),
                Cow::Owned(vec) => vec.truncate(len),
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

// Comparisons and Ordering

impl std::cmp::PartialEq for Bytes<'_> {
    /// Compares two `Bytes` instances for equality based solely on their data content.
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl std::cmp::Eq for Bytes<'_> {}

impl std::cmp::PartialOrd for Bytes<'_> {
    /// Provides partial ordering for `Bytes` instances based on lexicographic comparison of data.
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for Bytes<'_> {
    /// Provides total ordering for `Bytes` instances using lexicographic comparison of data.
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.data.cmp(&other.data)
    }
}

// Conversions

impl std::convert::AsRef<[u8]> for Bytes<'_> {
    /// Returns a reference to the bytes in the buffer. Does not allocate.
        fn as_ref(&self) -> &[u8] {
        self.data.as_ref()
    }
}

// Conversions: Slices

impl<'b> std::convert::From<&'b [u8]> for Bytes<'b> {
    /// Converts a borrowed immutable `&[u8]` slice of bytes into a `Bytes` buffer.
        fn from(borrowed_slice_of_bytes: &'b [u8]) -> Self {
        Bytes {
            metadata: Metadata::default(),
            data: borrowed_slice_of_bytes.into()
        }
    }
}

impl<'b> std::convert::From<&'b mut [u8]> for Bytes<'b> {
    /// Converts a borrowed mutable `&mut [u8]` slice of bytes into a `Bytes` buffer.
        fn from(borrowed_slice_of_bytes: &'b mut [u8]) -> Self {
        Bytes {
            metadata: Metadata::default(),
            data: (&*borrowed_slice_of_bytes).into()
        }
    }
}

impl<'b> std::convert::From<&'b str> for Bytes<'b> {
    /// Converts a borrowed `&str` string slice into a `Bytes` buffer.
        fn from(string: &'b str) -> Self {
        Bytes {
            metadata: Metadata::default(),
            data: Cow::Borrowed(string.as_bytes())
        }
    }
}

// Conversions: Arrays

impl<const N: usize> std::convert::From<[u8; N]> for Bytes<'_> {
    /// Converts an owned array of bytes into a `Bytes` buffer.
        fn from(owned_array_of_bytes: [u8; N]) -> Self {
        Bytes {
            metadata: Metadata::default(),
            data: Cow::Owned(owned_array_of_bytes.into())
        }
    }
}

impl<'b, const N: usize> std::convert::From<&'b [u8; N]> for Bytes<'b> {
    /// Converts a borrowed array of bytes into a `Bytes` buffer.
        fn from(borrowed_array_of_bytes: &'b [u8; N]) -> Self {
        Bytes {
            metadata: Metadata::default(),
            data: Cow::Borrowed(borrowed_array_of_bytes)
        }
    }
}

impl<'b, const N: usize> std::convert::From<&'b mut [u8; N]> for Bytes<'b> {
    /// Converts a mutable borrowed array of bytes into a `Bytes` buffer.
        fn from(borrowed_array_of_bytes: &'b mut [u8; N]) -> Self {
        Bytes {
            metadata: Metadata::default(),
            data: Cow::Borrowed(borrowed_array_of_bytes)
        }
    }
}

// Conversions: Cows

impl<'b> std::convert::From<&'b Cow<'b, [u8]>> for Bytes<'b> {
    /// Converts an borrowed immutable `&Cow<'b, [u8]>` collection of bytes into a `Bytes` buffer.
        fn from(borrowed_clone_on_write: &'b Cow<'b, [u8]>) -> Self {
        Bytes {
            metadata: Metadata::default(),
            data: match borrowed_clone_on_write {
                Cow::Borrowed(slice) => Cow::Borrowed(*slice),
                Cow::Owned(vec) => Cow::Borrowed(vec)
            }
        }
    }
}

impl<'b> std::convert::From<&'b mut Cow<'b, [u8]>> for Bytes<'b> {
    /// Converts a borrowed mutable `&mut Cow<'b, [u8]>` collection of bytes into a `Bytes` buffer.
        fn from(borrowed_clone_on_write: &'b mut Cow<'b, [u8]>) -> Self {
        Bytes {
            metadata: Metadata::default(),
            data: match borrowed_clone_on_write {
                Cow::Borrowed(slice) => Cow::Borrowed(*slice),
                Cow::Owned(vec) => Cow::Borrowed(vec)
            }
        }
    }
}

impl<'b> std::convert::From<Cow<'b, [u8]>> for Bytes<'b> {
    /// Converts an owned `Cow<'b, [u8]>` collection of bytes into a `Bytes` buffer.
        fn from(owned_clone_on_write: Cow<'b, [u8]>) -> Self {
        Bytes {
            metadata: Metadata::default(),
            data: owned_clone_on_write
        }
    }
}

// Conversions: Vecs

impl<'b> std::convert::From<&'b Vec<u8>> for Bytes<'b> {
    /// Converts a borrowed immutable `&Vec<u8>` collection of bytes into a `Bytes` buffer.
        fn from(borrowed_vec_of_bytes: &'b Vec<u8>) -> Self {
        Bytes {
            metadata: Metadata::default(),
            data: Cow::Borrowed(borrowed_vec_of_bytes)
        }
    }
}

impl<'b> std::convert::From<&'b mut Vec<u8>> for Bytes<'b> {
    /// Converts a borrowed mutable `&Vec<u8>` collection of bytes into a `Bytes` buffer.
        fn from(borrowed_vec_of_bytes: &'b mut Vec<u8>) -> Self {
        Bytes {
            metadata: Metadata::default(),
            data: Cow::Borrowed(&*borrowed_vec_of_bytes)
        }
    }
}

impl std::convert::From<Vec<u8>> for Bytes<'_> {
    /// Converts an owned `Vec<u8>` collection of bytes into a `Bytes` buffer.
        fn from(owned_vec_of_bytes: Vec<u8>) -> Self {
        Bytes {
            metadata: Metadata::default(),
            data: Cow::Owned(owned_vec_of_bytes)
        }
    }
}

impl std::convert::From<String> for Bytes<'_> {
    /// Converts an owned `String` into a `Bytes` buffer.
        fn from(string: String) -> Self {
        Bytes {
            metadata: Metadata::default(),
            data: Cow::Owned(string.into_bytes())
        }
    }
}

// Conversions: Other

impl<'b> std::convert::From<Bytes<'b>> for Vec<u8> {
    /// Convert an owned `Bytes` buffer into an owned `Vec<u8>` collection of bytes.
        fn from(bytes: Bytes<'b>) -> Self {
        bytes.data.into_owned()
    }
}

impl<'b> std::convert::From<&'b mut Bytes<'b>> for Option<&'b mut Vec<u8>> {
    /// Returns a mutable reference to the underlying `Vec<u8>` if the data is owned, or `None` if
    /// borrowed.
    ///
    /// This allows in-place modifications (for example, decryption) without allocation when
    /// possible. If `None` is returned, you may need to clone and own the data explicitly for
    /// mutations.
        fn from(bytes: &'b mut Bytes<'b>) -> Self {
        match bytes.data {
            Cow::Owned(ref mut vec) => Some(vec),
            Cow::Borrowed(_) => None,
        }
    }
}

impl<'b> std::convert::From<Bytes<'b>> for Cow<'b, [u8]> {
    /// Unwraps a `Bytes` into the underlying `Cow<[u8]>` bytes, discarding the metadata.
        fn from(value_buf: Bytes<'b>) -> Self {
        value_buf.data
    }
}

impl<'b> std::convert::From<Bytes<'b>> for (Metadata, Cow<'b, [u8]>) {
    /// Destructures the `Bytes` buffer into a tuple, where element `0` is the `Metadata` and
    /// element `1` is a `Cow<[u8]>` that contains the data.
    ///
    /// This is useful when an operation requires unencumbered & owned data, but the metadata needs
    /// to be preserved.
        fn from(value_buf: Bytes<'b>) -> (Metadata, Cow<'b, [u8]>) {
        (value_buf.metadata, value_buf.data)
    }
}

impl<'b> std::convert::From<(Metadata, Cow<'b, [u8]>)> for Bytes<'b> {
    /// Instantiates a `Bytes` buffer from `Metadata` and `Cow<[u8]>` data parts.
    ///
    /// This is useful when an operation requires unencumbered & owned data, but the metadata needs
    /// to be preserved.
        fn from(tuple: (Metadata, Cow<'b, [u8]>)) -> Self {
        Bytes { metadata: tuple.0, data: tuple.1 }
    }
}

// Iterators

impl std::iter::Extend<u8> for Bytes<'_> {
    /// Extends the `Bytes` with the contents of an iterator of bytes.
    ///
    /// If the buffer contains borrowed data, it will be converted to owned data to allow
    /// modification.
        fn extend<T: IntoIterator<Item = u8>>(&mut self, iter: T) {
        let iter = iter.into_iter();

        // Get size hint to potentially optimize allocation
        let (lower_bound, _) = iter.size_hint();

        match &mut self.data {
            Cow::Borrowed(slice) => {
                // Convert to owned with optimized capacity
                let mut vec = Vec::with_capacity(slice.len() + lower_bound);
                vec.extend_from_slice(slice);
                vec.extend(iter);
                self.data = Cow::Owned(vec);
            }
            Cow::Owned(vec) => {
                // Reserve space based on size hint and extend directly
                vec.reserve(lower_bound);
                vec.extend(iter);
            }
        }
    }
}

impl<'b> std::iter::Extend<&'b u8> for Bytes<'b> {
    /// Extends the `Bytes` with the contents of an iterator of byte references.
    ///
    /// This implementation dereferences each byte reference and delegates to the main `Extend<u8>`
    /// implementation.
        fn extend<T: IntoIterator<Item = &'b u8>>(&mut self, iter: T) {
        self.extend(iter.into_iter().copied());
    }
}

impl std::iter::FromIterator<u8> for Bytes<'_> {
    /// Create a new `Bytes` from an iterator of bytes.
        fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        Bytes {
            metadata: Metadata::default(),
            data: Cow::Owned(iter.into_iter().collect()),
        }
    }
}

// Operators

impl<'b> std::ops::Deref for Bytes<'b> {
    type Target = Cow<'b, [u8]>;

    /// Returns a reference to the bytes in the buffer. Does not allocate.
        fn deref(&self) -> &Self::Target {
        &self.data
    }
}