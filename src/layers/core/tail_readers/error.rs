//! Contains the error type returned from the tail readers.

// -------------------------------------------------------------------------------------------------
//
/// An error that occurs when attempting to read from a `TailReader`.
#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum Error {
    /// Insufficient data in buffer to perform requested read.
    ///
    /// This typically indicates a corrupted, truncated, or malformed buffer where the expected
    /// data is missing from the end of the byte array.
    #[error(
        "attempted to read {bytes_read} bytes from buffer \
        but only {bytes_remaining} bytes remain"
    )]
    EndOfBuffer {
        bytes_read: usize,
        bytes_remaining: usize
    },

    /// Not enough data to read vector.
    ///
    /// This typically indicates a corrupted, truncated, or malformed buffer where the expected
    /// integer is missing from the end of the byte array.
    #[error(
        "attempted to {number_of_elements} elements × {element_bytes} bytes each \
        ({total_bytes} total bytes) from buffer but only {bytes_remaining} bytes remain"
    )]
    EndOfBufferBytes {
        number_of_elements: usize,
        element_bytes: usize,
        total_bytes: usize,
        bytes_remaining: usize
    }
}