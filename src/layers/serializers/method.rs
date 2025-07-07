//! An enumeration that lists all available serialization methods.

use crate::layers::descriptors::LAYER_MTHD_SHIFT;

// -------------------------------------------------------------------------------------------------
//
/// An enumeration that helps provide runtime identification of the serialization method in use,
/// allowing applications to log serialization details, or store metadata about how data was
/// processed in the data pipeline.
///
/// This type is returned by the `Serializer` trait.
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u16)]
#[non_exhaustive]
pub enum Method {
    /// Bincode using native implementation for compact binary encoding.
    BincodeNative    = 0 << LAYER_MTHD_SHIFT, // 00000_000b

    /// Bincode using `serde` framework for compact binary encoding with broader compatibility. Use
    /// when you need bincode's efficiency but require serde trait compatibility.
    BincodeSerde     = 1 << LAYER_MTHD_SHIFT, // 00001_000b

    /// Bitcode using native implementation for extremely compact binary format with bitwise
    /// optimization. Use when minimizing serialized size is critical.
    BitcodeNative    = 2 << LAYER_MTHD_SHIFT, // 00010_000b

    /// Bitcode using `serde` framework for compact binary format with standard trait support.
    /// Use when you need bitcode's space efficiency with serde ecosystem compatibility.
    BitcodeSerde     = 3 << LAYER_MTHD_SHIFT, // 00011_000b

    /// Borsh binary serialization designed for security-critical applications like blockchain.
    Borsh            = 4 << LAYER_MTHD_SHIFT, // 00100_000b

    /// Müsli descriptive mode providing self-describing binary format with schema information. Use
    /// when you need binary efficiency with built-in type information for debugging or flexibility.
    MusliDescriptive = 5 << LAYER_MTHD_SHIFT, // 00101_000b

    /// Müsli storage mode optimized for compact binary serialization without self-description.
    /// Use when you need maximum space efficiency and have schema information available separately.
    MusliStorage     = 6 << LAYER_MTHD_SHIFT, // 00110_000b

    /// Müsli wire mode designed for network protocols with stable binary representation. Use when
    /// you need consistent serialization for network communication or cross-version compatibility.
    MusliWire        = 7 << LAYER_MTHD_SHIFT, // 00111_000b

    /// Postcard using serde for compact, embedded-friendly binary format without standard library.
    /// Use when targeting data exchange with resource-constrained environments or embedded systems.
    PostcardSerde    = 8 << LAYER_MTHD_SHIFT,  // 01000_000b

    /// Rkyv zero-copy deserialization enabling direct memory mapping of serialized data. Use when
    /// you need maximum deserialization performance and can work with archived data structures.
    Rkyv             = 9 << LAYER_MTHD_SHIFT,  // 01001_000b

    /// MessagePack using serde for cross-language binary format with JSON-like flexibility. Use
    /// when you need compact binary serialization with multi-language compatibility.
    RmpSerde         = 10 << LAYER_MTHD_SHIFT, // 01010_000b

    /// Zerocopy enabling safe zero-copy parsing with compile-time layout verification. Use when you
    /// need maximum performance for reading structured data without deserialization overhead.
    Zerocopy         = 11 << LAYER_MTHD_SHIFT, // 01011_000b
}

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl TryFrom<&u16> for &'static Method {
    type Error = crate::layers::descriptors::Error;

    /// Converts a `&u16` word into a serialization `&Method` enum.
    ///
    /// # Errors
    /// Returns an error for unrecognized values.
    #[inline]
    fn try_from(value: &u16) -> Result<Self, Self::Error> {
        match value {
            0  => Ok(&Method::BincodeNative),
            8  => Ok(&Method::BincodeSerde),
            16 => Ok(&Method::BitcodeNative),
            24 => Ok(&Method::BitcodeSerde),
            32 => Ok(&Method::Borsh),
            40 => Ok(&Method::MusliDescriptive),
            48 => Ok(&Method::MusliStorage),
            56 => Ok(&Method::MusliWire),
            64 => Ok(&Method::PostcardSerde),
            72 => Ok(&Method::Rkyv),
            80 => Ok(&Method::RmpSerde),
            88 => Ok(&Method::Zerocopy),
            _  => Err(Self::Error::UnrecognizedSerializer(*value)),
        }
    }
}

impl std::fmt::Display for Method {
    /// Formats the serialization `Method` as a human-readable string.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BincodeNative    => write!(f, "bincode native"),
            Self::BincodeSerde     => write!(f, "bincode serde"),
            Self::BitcodeNative    => write!(f, "bitcode native"),
            Self::BitcodeSerde     => write!(f, "bitcode serde"),
            Self::Borsh            => write!(f, "borsh"),
            Self::MusliDescriptive => write!(f, "müsli descriptive"),
            Self::MusliStorage     => write!(f, "müsli storage"),
            Self::MusliWire        => write!(f, "müsli wire"),
            Self::PostcardSerde    => write!(f, "postcard serde"),
            Self::Rkyv             => write!(f, "rkyv"),
            Self::RmpSerde         => write!(f, "messagepack serde"),
            Self::Zerocopy         => write!(f, "zerocopy"),
        }
    }
}

// -------------------------------------------------------------------------------------------------
//
// Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_method_roundtrip() {
        // All variants to test
        let methods = [
            Method::BincodeNative,
            Method::BincodeSerde,
            Method::BitcodeNative,
            Method::BitcodeSerde,
            Method::Borsh,
            Method::MusliDescriptive,
            Method::MusliStorage,
            Method::MusliWire,
            Method::PostcardSerde,
            Method::Rkyv,
            Method::RmpSerde,
            Method::Zerocopy,
        ];

        for method in &methods {
            // Convert to u16
            let as_u16: u16 = *method as u16;

            // Convert back via TryFrom
            let recovered = <&Method>::try_from(&as_u16)
                .expect(&format!("failed to convert back from u16: {}", as_u16));

            // Verify round-trip
            assert_eq!(
                method, recovered,
                "round-trip failed for {:?}: {} -> {}",
                method, as_u16, *recovered as u16
            );
        }
    }

    #[test]
    fn test_method_values() {
        // Verify the expected bit-shifted values
        assert_eq!(Method::BincodeNative as u16, 0);
        assert_eq!(Method::BincodeSerde as u16, 8);
        assert_eq!(Method::BitcodeNative as u16, 16);
        assert_eq!(Method::BitcodeSerde as u16, 24);
        assert_eq!(Method::Borsh as u16, 32);
        assert_eq!(Method::MusliDescriptive as u16, 40);
        assert_eq!(Method::MusliStorage as u16, 48);
        assert_eq!(Method::MusliWire as u16, 56);
        assert_eq!(Method::PostcardSerde as u16, 64);
        assert_eq!(Method::Rkyv as u16, 72);
        assert_eq!(Method::RmpSerde as u16, 80);
        assert_eq!(Method::Zerocopy as u16, 88);
    }

    #[test]
    fn test_invalid_method() {
        // Test that invalid values return errors
        let invalid_values = [1, 7, 9, 15, 17, 23, 25, 31, 33, 39, 41, 47, 49, 255];

        for invalid in invalid_values {
            assert!(
                <&Method>::try_from(&invalid).is_err(),
                "expected error for invalid value: {}",
                invalid
            );
        }
    }

    #[test]
    fn test_method_error_message() {
        // Test that error messages are helpful
        let result = <&Method>::try_from(&99);
        assert!(result.is_err());

        let error = result.unwrap_err();
        let error_msg = error.to_string();
        assert!(error_msg.contains("unrecognized serialization method"));
        assert!(error_msg.contains("99"));
    }
}