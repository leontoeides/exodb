//! Describes the layer type (compression, serialization, etc.) and applied direction for a layer 
//! (read-only, write-only, both directions, do not use, etc.)

use crate::layers::descriptors::{Direction, Error, Layer};

/// A structure that provides a common interface for extracting layer information from packed 
/// descriptor bytes.
///
/// # Layer Descriptor
///
/// Represents:
/// 1. Layer type (for example, serialization, compression, encryption, error correction, etc.)
/// 2. Implementation type (for example, Brotli, LZ4, Zlib, etc.)
/// 3. Direction applicatiom (for example, compress on write-only, read data back as compressed)
///
/// The format is `000000DDIIIIILLL` where:
/// * `LLL` (bits 0-2): Layer type (3 bits = 8 layers types max.)
/// * `IIIII` (bits 3-7): Implementation (5 bits = 32 implementations per layer max.)
/// * `DD` (bits 8-9): Direction when applied (2 bits = 4 directions)
/// * `000000` (bits 10-15): Reserved for future use
pub struct Descriptor(u16);

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl Descriptor {
    /// Instantiates a `Descriptor` from a `u16` word.
    #[inline]
    pub fn try_from_u16(word: u16) -> Result<Descriptor, Error> {
        word.try_into()
    }

    /// Instantiates a `Descriptor` from a `&[u8; 2]` bytes.
    #[inline]
    pub fn try_from_bytes(bytes: &[u8; 2]) -> Result<Descriptor, Error> {
        bytes.try_into()
    }

    /// Extracts the layer type from the `u16` layer descriptor word.
    ///
    /// `Layer` is an enumeration that specifies a type of layer, such as compression, encryption,
    /// serialization, or error correction.
    #[inline]
    pub fn layer(&self) -> Result<&'static Layer, Error> {
        <&Layer>::try_from(self.raw())
    }

    /// Extracts the direction from the `u16` layer descriptor word.
    ///
    /// The direction specifies when this layer should be applied during data processing (read-only,
    /// write-only, both, or never).
    #[inline]
    pub fn direction(&self) -> Result<&'static Direction, Error> {
        <&Direction>::try_from(self.raw())
    }

    /// Validates the descriptor's internal consistency.
    ///
    /// This method performs comprehensive validation of all bit fields to ensure the descriptor
    /// contains valid, coherent data.
    #[inline]
    fn validate(&self) -> Result<(), Error> {
        let raw = self.raw();

        if (raw & 0b1111_1100_0000_0000) != 0 {
            Err(Error::ReservationBitsUsed(*raw))
        } else {
            Ok(())
        }
    }    

    /// Extracts the serialization method implementation from the `&u16` layer descriptor word.
    ///
    /// `Method` is an enumeration that provides identification of the serialization method, such
    /// as `BincodeSerde`, `BitcodeNative`, `RmpSerde`, etc.
    #[inline]
    pub fn serialization_method(
        &self
    ) -> Result<&'static crate::layers::serializers::Method, Error> {
        <&crate::layers::serializers::Method>::try_from(&self.0)
    }

    /// Extracts the compression method implementation from the `&u16` layer descriptor word.
    ///
    /// `Method` is an enumeration that provides identification of the compression method, such
    /// as `Brotli`, `Lz4`, `Zlib`, etc.
    #[inline]
    pub fn compression_method(
        &self
    ) -> Result<&'static crate::layers::compressors::Method, Error> {
        <&crate::layers::compressors::Method>::try_from(&self.0)
    }  

    /// Extracts the encryption method implementation from the `&u16` layer descriptor word.
    ///
    /// `Method` is an enumeration that provides identification of the encryption method, such as
    /// `AesGcm`, `ChaCha20`, etc.
    #[inline]
    pub fn encryption_method(
        &self
    ) -> Result<&'static crate::layers::encryptors::Method, Error> {
        <&crate::layers::encryptors::Method>::try_from(&self.0)
    } 

    /// Extracts the method error correction implementation from the `&u16` layer descriptor word.
    ///
    /// `Method` is an enumeration that provides identification of the error correction method, such
    /// as `ReedSolomon`, etc.
    #[inline]
    pub fn error_correction_method(
        &self
    ) -> Result<&'static crate::layers::correctors::Method, Error> {
        <&crate::layers::correctors::Method>::try_from(&self.0)
    }     

    /// Returns the raw `&u16` layer descriptor word.
    ///
    /// This provides access to the underlying packed representation, useful for serialization,
    /// debugging, or low-level operations.
    #[inline]
    fn raw(&self) -> &u16 {
        &self.0
    }
}

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl TryFrom<u16> for Descriptor {
    type Error = Error;

    /// Convert a `u16` descriptor word into a `Descriptor` structure.
    #[inline]
    fn try_from(word: u16) -> Result<Descriptor, Self::Error> {
        let descriptor = Self(word);
        descriptor.validate()?;
        Ok(descriptor)
    }
}

impl TryFrom<&[u8; 2]> for Descriptor {
    type Error = Error;

    /// Convert a `&[u8; 2]` descriptor word into a `Descriptor` structure.
    #[inline]
    fn try_from(bytes: &[u8; 2]) -> Result<Descriptor, Self::Error> {
        let word = u16::from_le_bytes(*bytes);
        word.try_into()
    }
}