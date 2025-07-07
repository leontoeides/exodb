//! Encryption layer parameters. This structure provides the information necessary to process
//! encryption layers.

mod error;
pub use crate::layers::encryptors::parameters::error::Error;

use crate::layers::TailReader;

// -------------------------------------------------------------------------------------------------
//
/// Both AES-GCM and ChaCha20Poly1305 use the same sized nonce of `96`-bits or `12`-bytes.
const NONCE_SIZE: usize = 12;

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
/// | `nonce`  |
/// |----------|
/// | `&[u32]` |
///
/// # Notes
///
/// * In the case of ECC error correction, the layer parameters are implementation specific. In
///   the case of encryption, both AES-GCM and ChaCha20Poly1305 use the same sized nonce of
///   `96`-bits or `12`-bytes, with the same AEAD architecture. So, this structure is shared between
///   them.
pub(super) struct Parameters<'b> {
    pub(crate) nonce: &'b [u8],
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl<'b> Parameters<'b> {
    /// Instantiates a new `Parameters` struct from a nonce.
    #[inline]
    pub fn from_nonce(nonce: &'b [u8]) -> Self {
        Parameters { nonce }
    }

    /// Deserializes `Parameters` from the end of a data buffer.
    ///
    /// Reads the encryption method (u8) and nonce (size depends on method: 12 for ChaCha20, 12 for
    /// AesGcm) in reverse order from the buffer’s end.
    ///
    /// # Layer Structure
    ///
    /// | `data`  | `parameters` |
    /// |---------|--------------|
    /// | `&[u8]` | `Parameters` |
    ///
    /// # Parameters Structure
    ///
    /// | `nonce`  |
    /// |----------|
    /// | `&[u32]` |
    ///
    /// # Errors
    ///
    /// Returns an error if the buffer is too short, the method is invalid, or the nonce size is
    /// incorrect.
    #[inline]
    pub fn from_data_buffer(tail_reader: &mut TailReader<'b>) -> Result<Parameters<'b>, Error> {
        let nonce: &[u8] = tail_reader.read_slice(NONCE_SIZE)
            .map_err(|error| Error::InsufficientData { parameter: "nonce", error })?
            .try_into()
            .map_err(|_| Error::InvalidParameter("nonce"))?;
        Ok(Parameters { nonce: &nonce })
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
    /// | `nonce`  |
    /// |----------|
    /// | `&[u32]` |
    ///
    /// # Errors
    ///
    /// Returns an error if the nonce length is not 12 bytes.
    #[inline]
    pub fn to_data_buffer(&self, buffer: &mut Vec<u8>) -> Result<(), Error> {
        if self.nonce.len() != NONCE_SIZE {
            Err(Error::InvalidNonce { expected_size: NONCE_SIZE, provided_size: self.nonce.len() })
        } else {
            buffer.extend_from_slice(self.nonce);
            Ok(())
        }
    }
}