//! A byte-buffer reader that reads right-to-left from an owned `Vec<u8>` of bytes. This helps
//! process any metadata stored in encryption and ECC error correction layers.

use crate::layers::core::tail_readers::Error;

// -------------------------------------------------------------------------------------------------
//
/// A byte-buffer reader that reads right-to-left from an owned `Vec<u8>` slice of bytes.
///
/// `TailReaderMut` is designed for parsing metadata and structured data that is stored at the end
/// of encoded buffers. It maintains a position that moves backwards through the buffer as data is
/// read, making it efficient for protocols that append metadata in reverse order or store critical
/// information at the tail end of the data.
///
/// This reader is primary intended for layer processing where descriptors, error correction
/// metadata, shard information, and other control data is positioned at the end of byte arrays for
/// efficient access without requiring knowledge of the preceding data structure.
pub struct TailReaderMut {
    /// An owned `Vec<u8>` of bytes.
    data: Vec<u8>,

    /// Cursor's current position from start of the buffer, for reading right-to-left.
    position: usize,
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl TailReaderMut {
    /// Creates a new `TailReaderMut` from a `Vec<u8>`.
    ///
    /// The reader is positioned at the end of the vector, ready to read backwards.
    #[inline]
    #[must_use]
    pub fn from_vec(vec: Vec<u8>) -> Self {
        Self::from(vec)
    }

    /// Returns the current byte position of the `TailReaderMut` when reading backwards
    /// right-to-left. It indicates the end of the buffer.
    #[must_use]
    pub const fn position(&self) -> &usize {
        &self.position
    }

    /// Reads a little-endian `u8` from the end of a data buffer, moving the `position` backwards by
    /// one byte, and returns it as `u8`.
    ///
    /// This function reads metadata stored at the end of encoded data by:
    /// 1. Checking if there are at least `1` bytes remaining from the current `position`.
    /// 2. Moving the `position` backwards by `1` bytes.
    /// 3. Reading the `u8` value in little-endian format.
    ///
    /// # Errors
    ///
    /// Returns an error if there's insufficient data to read a `u8` type from the buffer.
    #[inline]
    pub fn read_u8_le(&mut self) -> Result<u8, Error> {
        const U8_SIZE: usize = std::mem::size_of::<u8>();
        if self.position < U8_SIZE {
            Err(Error::EndOfBuffer {
                bytes_read: U8_SIZE,
                bytes_remaining: self.position
            })
        } else {
            self.position -= U8_SIZE;
            let bytes = [
                self.data[self.position]
            ];
            Ok(u8::from_le_bytes(bytes))
        }
    }

    /// Reads a little-endian `u16` from the end of a data buffer, moving the `position` backwards
    /// by two bytes, and returns it as `u16`.
    ///
    /// This function reads metadata stored at the end of encoded data by:
    /// 1. Checking if there are at least `2` bytes remaining from the current `position`.
    /// 2. Moving the `position` backwards by `2` bytes.
    /// 3. Reading the `u16` value in little-endian format.
    ///
    /// # Errors
    ///
    /// Returns an error if there's insufficient data to read a `u16` type from the buffer.
    #[inline]
    pub fn read_u16_le(&mut self) -> Result<u16, Error> {
        const U16_SIZE: usize = std::mem::size_of::<u16>();
        if self.position < U16_SIZE {
            Err(Error::EndOfBuffer {
                bytes_read: U16_SIZE,
                bytes_remaining: self.position
            })
        } else {
            self.position -= U16_SIZE;
            let bytes = [
                self.data[self.position],
                self.data[self.position + 1]
            ];
            Ok(u16::from_le_bytes(bytes))
        }
    }
    
    /// Reads a little-endian `u32` from the end of a data buffer, moving the `position` backwards
    /// by four bytes, and returns it as `u32`.
    ///
    /// This function reads metadata stored at the end of encoded data by:
    /// 1. Checking if there are at least `4` bytes remaining from the current `position`.
    /// 2. Moving the `position` backwards by `4` bytes.
    /// 3. Reading the `u32` value in little-endian format.
    ///
    /// # Errors
    ///
    /// Returns an error if there's insufficient data to read a `u32` type from the buffer.
    #[inline]
    pub fn read_u32_le(&mut self) -> Result<u32, Error> {
        const U32_SIZE: usize = std::mem::size_of::<u32>();
        if self.position < U32_SIZE {
            Err(Error::EndOfBuffer {
                bytes_read: U32_SIZE,
                bytes_remaining: self.position
            })
        } else {
            self.position -= U32_SIZE;
            let bytes = [
                self.data[self.position],
                self.data[self.position + 1],
                self.data[self.position + 2],
                self.data[self.position + 3],
            ];
            Ok(u32::from_le_bytes(bytes))
        }
    }

    /// Reads multiple little-endian `u32` double-words from the end of a data buffer, moving the
    /// `position` backwards and returning them as a `Vec<u32>`.
    ///
    /// This function reads metadata stored at the end of encoded data by:
    /// 1. Checking if there are at least `len * 4` bytes remaining from the current `position`.
    /// 2. Moving the `position` backwards by `len * 4` bytes.
    /// 3. Reading the `Vec<u32>` values in little-endian format.
    ///
    /// # Errors
    ///
    /// Returns an error if there's insufficient data to read `len * u32` types from the buffer.
    #[inline]
    pub fn read_u32_le_vec(&mut self, len: usize) -> Result<Vec<u32>, Error> {
        const U32_SIZE: usize = std::mem::size_of::<u32>();
        let total_size = len * U32_SIZE;
        if self.position < total_size {
            Err(Error::EndOfBufferBytes {
                number_of_elements: len,
                element_bytes: U32_SIZE,
                total_bytes: total_size,
                bytes_remaining: self.position
            })
        } else {
            self.position -= total_size;
            let vec = self.data[self.position..self.position + total_size]
                .chunks_exact(U32_SIZE)
                .map(|chunk| {
                    let bytes = [chunk[0], chunk[1], chunk[2], chunk[3]];
                    u32::from_le_bytes(bytes)
                })
                .collect();
            Ok(vec)
        }
    }

    /// Reads a byte slice from the end of a data buffer, moving the `position` backwards and
    /// returning them as a `&[u8]`.
    ///
    /// This function reads metadata stored at the end of encoded data by:
    /// 1. Checking if there are at least `len * 1` bytes remaining from the current `position`.
    /// 2. Moving the `position` backwards by `len * 1` bytes.
    /// 3. Reading the `&[u8]` slice.
    ///
    /// # Errors
    ///
    /// Returns an error if there's insufficient data to read `len * u8` types from the buffer.
    #[inline]
    pub fn read_slice(&mut self, len: usize) -> Result<&[u8], Error> {
        if self.position < len {
            Err(Error::EndOfBufferBytes {
                number_of_elements: len,
                element_bytes: 1,
                total_bytes: len,
                bytes_remaining: self.position
            })
        } else {
            self.position -= len;
            let slice = &self.data[self.position..self.position + len];
            Ok(slice)
        }
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
    #[inline]
    #[allow(clippy::missing_panics_doc, reason = "panic not possible, size is checked before unwrap")]
    pub fn read_array<const SIZE: usize>(&mut self) -> Result<&[u8; SIZE], Error> {
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
    #[inline]
    #[must_use]
    pub fn remaining(self) -> Vec<u8> {
        self.into()
    }
}

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl AsRef<[u8]> for TailReaderMut {
    /// Returns a reference to the bytes in the buffer. Does not allocate.
    #[inline]
    fn as_ref(&self) -> &[u8] {
        &self.data[..self.position]
    }
}

impl From<TailReaderMut> for Vec<u8> {
    /// Unwraps a `TailReaderMut` struct into the underlying owned `Vec<u8>` bytes.
    #[inline]
    fn from(mut tail_reader: TailReaderMut) -> Self {
        tail_reader.data.truncate(tail_reader.position);
        tail_reader.data
    }
}

impl From<Vec<u8>> for TailReaderMut {
    /// Wraps a `Vec<u8>` bytes into a `TailReaderMut` for a reading fields right-to-left.
    #[inline]
    fn from(vec: Vec<u8>) -> Self {
        Self { position: vec.len(), data: vec }
    }
}