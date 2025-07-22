//! A byte-buffer reader that reads right-to-left from an immutable slice of bytes This helps read
//! any layer parameters or metadata stored in encryption and ECC error correction layers.

use crate::layers::core::{Bytes, tail_readers::Error};

// -------------------------------------------------------------------------------------------------
//
/// A byte-buffer reader that reads data right-to-left from an immutable slice of bytes.
///
/// `TailReader` is designed for parsing metadata and structured data that is stored at the end of
/// encoded buffers. It maintains a position that moves backwards through the buffer as data is
/// read, making it efficient for protocols that append metadata in reverse order or store critical
/// information at the tail end of the data.
///
/// This reader is primary intended for layer processing where descriptors, error correction
/// metadata, shard information, and other control data is positioned at the end of byte arrays for
/// efficient access without requiring knowledge of the preceding data structure.
pub struct TailReader<'b> {
    /// A reference to an immutable slice of bytes.
    data: &'b [u8],

    /// Cursor's current position from start of the buffer, for reading right-to-left.
    position: usize,
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl<'b> TailReader<'b> {
    /// Creates a new `TailReader` from a byte slice.
    ///
    /// The reader is positioned at the end of the slice, ready to read backwards.
    #[must_use] pub fn from_slice(slice: &'b [u8]) -> Self {
        Self::from(slice)
    }

    /// Reads a fixed-length array `&[u8; SIZE]` from the end of the data buffer, moving the
    /// `position` backwards and returning the bytes as a `&[u8; SIZE]`.
    ///
    /// This function behaves similarly to `read_slice` but returns a reference to a fixed-size
    /// array instead of a slice. It is useful when the length is known at compile time.
    ///
    /// This function reads metadata stored at the end of encoded data by:
    /// 1. Checking if there are at least `SIZE * 1` bytes remaining from the current `position`.
    /// 2. Moving the `position` backwards by `SIZE * 1` bytes.
    /// 3. Reading the `&[u8; SIZE]` array.
    ///
    /// # Errors
    ///
    /// Returns an error if there's insufficient data to read `SIZE` bytes from the buffer.
    #[allow(clippy::missing_panics_doc, reason = "panic not possible, size is checked before unwrap")]
    pub fn read_array<const SIZE: usize>(&mut self) -> Result<&'b [u8; SIZE], Error> {
        if self.position < SIZE {
            Err(Error::EndOfBufferBytes {
                number_of_elements: SIZE,
                element_bytes: 1,
                total_bytes: SIZE,
                bytes_remaining: self.position,
            })
        } else {
            self.position -= SIZE;
            let slice = &self.data[self.position..self.position + SIZE];
            Ok(<&[u8; SIZE]>::try_from(slice).unwrap())
        }
    }

    /// Closes the reader and returns the remaining unread bytes.
    ///
    /// This consumes the reader and returns a slice containing only the bytes that were not read,
    /// effectively "trimming off" the consumed tail data.
    #[must_use] pub fn close(self) -> &'b [u8] {
        self.into()
    }
}

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl std::convert::AsRef<[u8]> for TailReader<'_> {
    /// Returns a reference to the bytes in the buffer. Does not allocate.
    fn as_ref(&self) -> &[u8] {
        &self.data[..self.position]
    }
}

impl<'b> std::convert::From<&'b [u8]> for TailReader<'b> {
    /// Wraps a `&[u8]` borrowed immutable slice into a `TailReader` for a reading fields
    /// right-to-left.
    fn from(slice: &'b [u8]) -> Self {
        Self { data: slice, position: slice.len() }
    }
}

impl<'b> std::convert::From<&'b Bytes<'b>> for TailReader<'b> {
    /// Wraps a `Vec<u8>` borrowed vector into a `TailReader` for a reading fields right-to-left.
    fn from(bytes: &'b Bytes<'b>) -> Self {
        Self { data: bytes.as_ref(), position: bytes.len() }
    }
}

impl<'b> std::convert::From<&'b Vec<u8>> for TailReader<'b> {
    /// Wraps a `Vec<u8>` borrowed vector into a `TailReader` for a reading fields right-to-left.
    fn from(vec: &'b Vec<u8>) -> Self {
        Self { data: vec.as_ref(), position: vec.len() }
    }
}

impl<'b> std::convert::From<TailReader<'b>> for &'b [u8] {
    /// Unwraps a `TailReader` struct into the underlying `&[u8]` immutable byte slice.
    fn from(tail_reader: TailReader<'b>) -> Self {
        &tail_reader.data[..tail_reader.position]
    }
}

impl<'b> std::ops::Deref for TailReader<'b> {
    type Target = [u8];

    /// Returns a reference to the underlying `&[u8]` immutable byte slice.
    fn deref(&self) -> &'b Self::Target {
        &self.data[..self.position]
    }
}