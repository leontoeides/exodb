//! A `zstd` specific `DictionaryBytesZstd` implementation.

use crate::layers::compressors::Compressible;
use crate::layers::compressors::impls::zstd::compression_level;
use zstd::dict::{DecoderDictionary, EncoderDictionary};

// -------------------------------------------------------------------------------------------------
//
/// A compression dictionary is used to store frequently occurring patterns or sequences in data,
/// allowing for more performant encoding and reducing the overall size of the data during
/// compression.
///
/// This structure represents a previously validated dictionary originating from a `Dictionary`
/// struct or `DictionaryProvider` trait.
///
/// This particular implementation is intended for use with the `zstd` crate.
pub struct DictionaryBytesZstd<'d> {
    encoder_dictionary: EncoderDictionary<'d>,
    decoder_dictionary: DecoderDictionary<'d>,
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl<'d> DictionaryBytesZstd<'d> {
    /// Converts a borrowed immutable `&[u8]` slice of bytes into a prepared `DictionaryBytesZstd`
    /// type.
    #[inline]
    #[must_use]
    pub fn from_slice<V: Compressible>(slice: &'d [u8]) -> Self {
        DictionaryBytesZstd {
            encoder_dictionary: EncoderDictionary::new(slice, compression_level::<V>()),
            decoder_dictionary: DecoderDictionary::new(slice),
        }
    }

    /// Returns a reference to an encoder dictionary, intended to be used with
    /// `zstd::bulk::Compressor`.
    #[inline]
    #[must_use]
    pub const fn as_encoder_dict(&self) -> &EncoderDictionary {
        &self.encoder_dictionary
    }

    /// Returns a reference to an decoder dictionary, intended to be used with
    /// `zstd::bulk::Decompressor`.
    #[inline]
    #[must_use]
    pub const fn as_decoder_dict(&self) -> &DecoderDictionary {
        &self.decoder_dictionary
    }
}