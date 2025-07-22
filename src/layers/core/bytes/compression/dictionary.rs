use crate::layers::compressors::{ActiveCompressor, DictionaryBytes};
use crate::layers::core::{bytes::Error, Bytes};
use crate::layers::{Compressible, Compressor};

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl Bytes<'_> {
    /// Restores compressed data to its original form, expanding the encoded data to the original
    /// representation.
    ///
    /// # Arguments
    ///
    /// * `dictionary` - Optional external dictionary that can give better performance and
    ///   compression ratios. **The dictionary is not stored with the compressed data. If provided,
    ///   the same dictionary must be used to decompress.**
    ///
    /// # Generics & Lifetimes
    ///
    /// * `V` generic represents the user's value type, for example: `User`, `String`, etc.
    /// * `b` lifetime represents bytes potentially being borrowed from the `redb` database.
    /// * 'd' lifetime represents a dictionary potentially being borrowed from a `Dictionary` or
    ///   `DictionaryProvider`.
    ///
    /// # Errors
    ///
    /// This method may fail for several reasons, including:
    ///
    /// * Input bytes are corrupted or malformed
    /// * Dictionary mismatch (if dynamic dictionary differs from compression-time dictionary)
    ///
    /// Consult the documentation of the compressor backend you are using for more detail on
    /// decompression behavior and potential limitations.
    #[inline]
    pub fn compress<V: Compressible>(
        self,
        dictionary: Option<DictionaryBytes<'_>>
    ) -> Result<Self, Error> {
        if V::DIRECTION.is_write() {
            Ok(ActiveCompressor::<V>::compress(self, dictionary)?)
        } else {
            Ok(self)
        }
    }

    /// Restores compressed data to its original form, expanding the encoded data to the original
    /// representation.
    ///
    /// # Arguments
    ///
    /// * `dictionary` - Optional external dictionary that can give better performance and
    ///   compression ratios. **Must be identical to the dictionary used during compression or
    ///   decompression will fail.**
    ///
    /// # Errors
    ///
    /// This method may fail for several reasons, including:
    ///
    /// * Input bytes are corrupted or malformed
    /// * Dictionary mismatch (if provided dictionary differs from compression-time dictionary)
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
        self,
        dictionary: Option<DictionaryBytes<'_>>
    ) -> Result<Self, Error> {
        if V::DIRECTION.is_read() {
            Ok(ActiveCompressor::<V>::decompress(self, dictionary)?)
        } else {
            Ok(self)
        }
    }
}