use crate::layers::compressors::ActiveCompressor;
use crate::layers::core::{bytes::Error, Bytes};
use crate::layers::{Compressible, Compressor};

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl Bytes<'_> {
    /// Restores compressed data to its original form, expanding the encoded data to the original
    /// representation.
    ///
    /// # Generics & Lifetimes
    ///
    /// * `V` generic represents the user's value type, for example: `User`, `String`, etc.
    /// * `b` lifetime represents bytes potentially being borrowed from the `redb` database.
    ///
    /// # Errors
    ///
    /// This method may fail for several reasons, including:
    ///
    /// * Input bytes are corrupted or malformed
    ///
    /// Consult the documentation of the compressor backend you are using for more detail on
    /// decompression behavior and potential limitations.
    #[inline]
    pub fn compress<V: Compressible>(
        self
    ) -> Result<Self, Error> {
        if V::DIRECTION.is_write() {
            Ok(ActiveCompressor::<V>::compress(self)?)
        } else {
            Ok(self)
        }
    }

    /// Restores compressed data to its original form, expanding the encoded data to the original
    /// representation.
    ///
    /// # Errors
    ///
    /// This method may fail for several reasons, including:
    ///
    /// * Input bytes are corrupted or malformed
    ///
    /// Consult the documentation of the compressor backend you are using for more detail on
    /// decompression behavior and potential limitations.
    ///
    /// # Generics & Lifetimes
    ///
    /// * `V` generic represents the user's value type, for example: `User`, `String`, etc.
    /// * `b` lifetime represents bytes potentially being borrowed from the `redb` database.
    #[inline]
    pub fn decompress<V: Compressible>(
        self
    ) -> Result<Self, Error> {
        if V::DIRECTION.is_read() {
            Ok(ActiveCompressor::<V>::decompress(self)?)
        } else {
            Ok(self)
        }
    }
}