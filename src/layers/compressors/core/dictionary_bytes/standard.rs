//! A common, general-purpose `DictionaryBytesStandard` implementation that works with byte slices. Good for
//! use with compressors like `lz4_flex` and `flate2`'s zlib implementation.

// -------------------------------------------------------------------------------------------------
//
/// A compression dictionary is used to store frequently occurring patterns or sequences in data,
/// allowing for more performant encoding and reducing the overall size of the data during
/// compression.
///
/// This structure represents a previously validated dictionary originating from a `Dictionary`
/// struct or `DictionaryProvider` trait.
///
/// This implementation works with byte-slices and is relatively general-purpsoe. Good for use with
/// compressors like `lz4_flex` and `flate2`'s zlib implementation.
pub struct DictionaryBytesStandard<'d>(&'d [u8]);

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl<'d> DictionaryBytesStandard<'d> {
    /// Converts a borrowed immutable `&[u8]` slice of bytes into a `DictionaryBytesStandard` type.
    #[inline]
    #[must_use]
    pub fn from_slice(borrowed_slice_of_bytes: &'d [u8]) -> Self {
        borrowed_slice_of_bytes.into()
    }

    /// Converts an borrowed immutable `Vec<u8>` collection of bytes into a
    /// `DictionaryBytesStandard` type.
    #[inline]
    #[must_use]
    pub fn from_vec(borrowed_vec_of_bytes: &'d std::vec::Vec<u8>) -> Self {
        borrowed_vec_of_bytes.into()
    }
}

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

// Conversions: Slices

impl<'d> From<&'d [u8]> for DictionaryBytesStandard<'d> {
    /// Converts a borrowed immutable `&[u8]` slice of bytes into a `DictionaryBytesStandard` type.
    fn from(borrowed_slice_of_bytes: &'d [u8]) -> Self {
        DictionaryBytesStandard(borrowed_slice_of_bytes)
    }
}

impl<'d> From<&'d mut [u8]> for DictionaryBytesStandard<'d> {
    /// Converts a borrowed mutable `&mut [u8]` slice of bytes into a `DictionaryBytesStandard` type.
    ///
    #[inline]
    fn from(borrowed_slice_of_bytes: &'d mut [u8]) -> Self {
        DictionaryBytesStandard(&*borrowed_slice_of_bytes)
    }
}

// Conversions: Other

impl<'d> From<&'d Vec<u8>> for DictionaryBytesStandard<'d> {
    /// Converts a borrowed immutable `&Vec<u8>` collection of bytes into a `DictionaryBytesStandard` type.
    fn from(borrowed_vec_of_bytes: &'d Vec<u8>) -> Self {
        DictionaryBytesStandard(borrowed_vec_of_bytes.as_slice())
    }
}

impl<'d> From<&'d mut Vec<u8>> for DictionaryBytesStandard<'d> {
    /// Converts a borrowed mutable `&Vec<u8>` collection of bytes into a `DictionaryBytesStandard` type.
    #[inline]
    fn from(borrowed_vec_of_bytes: &'d mut Vec<u8>) -> Self {
        DictionaryBytesStandard(borrowed_vec_of_bytes.as_slice())
    }
}

// Conversions: Other

impl<'d> From<DictionaryBytesStandard<'d>> for &'d [u8] {
    /// Converts a `DictionaryBytesStandard` type into a borrowed immutable `&[u8]` slice of bytes.
    fn from(dictionary_bytes: DictionaryBytesStandard<'d>) -> Self {
        dictionary_bytes.0
    }
}

impl std::convert::AsRef<[u8]> for DictionaryBytesStandard<'_> {
    /// Returns a reference to the bytes in the buffer. Does not allocate.
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.0
    }
}

// Operations

impl std::ops::Deref for DictionaryBytesStandard<'_> {
    type Target = [u8];

    /// Returns a reference to the bytes in the buffer. Does not allocate.
    #[inline]
    fn deref(&self) -> &Self::Target {
        self.0
    }
}