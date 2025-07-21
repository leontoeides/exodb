//! Encryption layer parameters. This structure provides the information necessary to process
//! encryption layers.

// Exports

mod error;
pub use crate::layers::encryptors::core::parameters::error::Error;

// Imports

use crate::layers::core::tail_readers::{TailReader, TailReaderMut};
use crate::layers::encryptors::core::Nonce;
use crate::layers::encryptors::impls::NONCE_SIZE;

// -------------------------------------------------------------------------------------------------
//
/// Encryption layer parameters.
///
/// This structure provides the information necessary to process encryption layers.
///
/// # Layer Structure
///
/// | `data`  | `parameters` |
/// |---------|--------------|
/// | `&[u8]` | `Parameters` |
///
/// # Parameters Structure
///
/// | `nonce`              |
/// |----------------------|
/// | `&[u32; NONCE_SIZE]` |
pub struct Parameters<'b> {
    /// A nonce is a unique, random or pseudo-random number used only once to ensure security by
    /// preventing replay attacks and that identical plaintexts produce different ciphertexts.
    pub nonce: Nonce<'b>,
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl<'b> Parameters<'b> {
    /// Instantiates a new `Parameters` struct from a nonce.
    #[inline]
    pub const fn from_nonce(nonce: Nonce<'b>) -> Self {
        Parameters { nonce }
    }

    /// Deserializes `Parameters` from the end of an immutable data buffer.
    ///
    /// Reads the encryption method (u8) and nonce (size depends on method: `12` bytes for
    /// `ChaCha20`, `12` bytes for `AesGcm`, etc.) in reverse order from the buffer’s end.
    ///
    /// # Layer Structure
    ///
    /// | `data`  | `parameters` |
    /// |---------|--------------|
    /// | `&[u8]` | `Parameters` |
    ///
    /// # Parameters Structure
    ///
    /// | `nonce`              |
    /// |----------------------|
    /// | `&[u32; NONCE_SIZE]` |
    ///
    /// # Errors
    ///
    /// Returns an error if the buffer is too short, the method is invalid, or the nonce size is
    /// incorrect.
    #[inline]
    pub fn from_data_buffer(
        tail_reader: &mut TailReader<'b>
    ) -> Result<Self, Error> {
        let array: &[u8; NONCE_SIZE] = tail_reader.read_array::<NONCE_SIZE>()
            .map_err(|error| Error::InsufficientData { parameter: "nonce", error })?;

        Ok(Parameters { nonce: Nonce::from(array) })
    }

    /// Deserializes `Parameters` from the end of an mutable data buffer.
    ///
    /// Reads the encryption method (u8) and nonce (size depends on method: `12` bytes for
    /// `ChaCha20`, `12` bytes for `AesGcm`, etc.) in reverse order from the buffer’s end.
    ///
    /// # Layer Structure
    ///
    /// | `data`  | `parameters` |
    /// |---------|--------------|
    /// | `&[u8]` | `Parameters` |
    ///
    /// # Parameters Structure
    ///
    /// | `nonce`              |
    /// |----------------------|
    /// | `&[u32; NONCE_SIZE]` |
    ///
    /// # Errors
    ///
    /// Returns an error if the buffer is too short, the method is invalid, or the nonce size is
    /// incorrect.
    #[inline]
    pub fn from_data_buffer_mut(
        tail_reader: &'b mut TailReaderMut
    ) -> Result<Self, Error> {
        let array: &[u8; NONCE_SIZE] = tail_reader.read_array::<NONCE_SIZE>()
            .map_err(|error| Error::InsufficientData { parameter: "nonce", error })?;

        Ok(Parameters { nonce: Nonce::from(array) })
    }

    /// Serializes `Parameters` to a data buffer, appending fields to the end.
    ///
    /// Appends the nonce (12 bytes) and encryption method (u8) to the provided buffer, matching the
    /// format expected by `from_data_buffer`.
    ///
    /// # Layer Structure
    ///
    /// | `data`  | `parameters` |
    /// |---------|--------------|
    /// | `&[u8]` | `Parameters` |
    ///
    /// # Parameters Structure
    ///
    /// | `nonce`              |
    /// |----------------------|
    /// | `&[u32; NONCE_SIZE]` |
    ///
    /// # Errors
    ///
    /// Returns an error if the nonce length is not 12 bytes.
    #[inline]
    pub fn into_data_buffer(self, buffer: &mut Vec<u8>) {
        buffer.extend(self.nonce.into_bytes());
    }
}