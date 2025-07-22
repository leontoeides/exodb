//! A byte-buffer reader that reads right-to-left from a bytes buffer. This helps read any layer
//! parameters or metadata stored in encryption and ECC error correction layers.

use crate::layers::core::{Bytes, tail_readers::Error};

// -------------------------------------------------------------------------------------------------
//
/// A byte-buffer reader that reads data right-to-left from an immutable slice of bytes.
///
/// `TailReaderBytes` is designed for parsing metadata and structured data that is stored at the end
/// of encoded buffers. It maintains a position that moves backwards through the buffer as data is
/// read, making it efficient for protocols that append metadata in reverse order or store critical
/// information at the tail end of the data.
///
/// This reader is primary intended for layer processing where descriptors, error correction
/// metadata, shard information, and other control data is positioned at the end of byte arrays for
/// efficient access without requiring knowledge of the preceding data structure.
pub struct TailReaderBytes<'t, 'b> {
    /// `Bytes` is similar to a `Cow`. These bytes may be owned or borrowed.
    bytes: &'t mut Bytes<'b>,

    /// Cursor's current position from start of the buffer, for reading right-to-left.
    position: usize,
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl<'t, 'b> TailReaderBytes<'t, 'b> {
    /// Creates a new `TailReaderBytes` from a `Bytes` buffer.
    ///
    /// The reader is positioned at the end of the vector, ready to read backwards.
    #[must_use] pub fn from_bytes(bytes: &'t mut Bytes<'b>) -> Self {
        Self::from(bytes)
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
                self.bytes.data[self.position],
                self.bytes.data[self.position + 1],
                self.bytes.data[self.position + 2],
                self.bytes.data[self.position + 3],
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
            let vec = self.bytes.data[self.position..self.position + total_size]
                .chunks_exact(U32_SIZE)
                .map(|chunk| {
                    let bytes = [chunk[0], chunk[1], chunk[2], chunk[3]];
                    u32::from_le_bytes(bytes)
                })
                .collect();
            Ok(vec)
        }
    }

    /// Closes the tail reader and truncates the byte buffer according to how many bytes were read.
    ///
    /// This drops the reader and returns a slice containing only the bytes that were not read,
    /// effectively "trimming off" the consumed tail data.
    pub fn close(self) {
        self.bytes.truncate(self.position);
    }
}

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl std::convert::AsRef<[u8]> for TailReaderBytes<'_, '_> {
    /// Returns a reference to the bytes in the buffer.
    fn as_ref(&self) -> &[u8] {
        &self.bytes.data[..self.position]
    }
}

impl<'t, 'b> std::convert::From<&'t mut Bytes<'b>> for TailReaderBytes<'t, 'b> {
    /// Wraps a `Bytes` buffer into a `TailReaderBytes` for a reading fields right-to-left.
    fn from(bytes: &'t mut Bytes<'b>) -> Self {
        Self { position: bytes.len(), bytes }
    }
}

impl<'b> std::ops::Deref for TailReaderBytes<'_, 'b> {
    type Target = Bytes<'b>;

    /// Returns a reference to the underlying `Bytes` data buffer.
    fn deref(&self) -> &Self::Target {
        self.bytes
    }
}