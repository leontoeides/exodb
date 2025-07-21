//! An enumeration that lists all available serialization methods.

// -------------------------------------------------------------------------------------------------
//
/// Helps provide runtime identification of the serialization method in use, allowing applications
/// to log serialization details, or store metadata about how data was processed in the data
/// pipeline.
///
/// This type is returned by the `Serializer` trait.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
#[non_exhaustive]
pub enum Method {
    /// Bincode using native implementation for compact binary encoding.
    BincodeNative    = 0,

    /// Bincode using `serde` framework for compact binary encoding with broader compatibility. Use
    /// when you need bincode's efficiency but require serde trait compatibility.
    BincodeSerde     = 1,

    /// Bitcode using native implementation for extremely compact binary format with bitwise
    /// optimization. Use when minimizing serialized size is critical.
    BitcodeNative    = 2,

    /// Bitcode using `serde` framework for compact binary format with standard trait support.
    /// Use when you need bitcode's space efficiency with serde ecosystem compatibility.
    BitcodeSerde     = 3,

    /// Borsh binary serialization designed for security-critical applications like blockchain.
    Borsh            = 4,

    /// Message Pack using serde for cross-language binary format with JSON-like flexibility. Use
    /// when you need compact binary serialization with multi-language compatibility.
    MessagePack      = 5,

    /// Müsli descriptive mode providing self-describing binary format with schema information. Use
    /// when you need binary efficiency with built-in type information for debugging or flexibility.
    MusliDescriptive = 6,

    /// Müsli storage mode optimized for compact binary serialization without self-description.
    /// Use when you need maximum space efficiency and have schema information available separately.
    MusliStorage     = 7,

    /// Müsli wire mode designed for network protocols with stable binary representation. Use when
    /// you need consistent serialization for network communication or cross-version compatibility.
    MusliWire        = 8,

    /// Müsli zero copy mode. Refreshingly simple, blazingly fast zero copy primitives by Müsli.
    /// This provides a basic set of tools to deal with types which do not require copying during
    /// deserialization.
    MusliZeroCopy    = 9,

    /// Postcard using serde for compact, embedded-friendly binary format without standard library.
    /// Use when targeting data exchange with resource-constrained environments or embedded systems.
    PostcardSerde    = 10,

    /// Rkyv zero-copy deserialization enabling direct memory mapping of serialized data. Use when
    /// you need maximum deserialization performance and can work with archived data structures.
    Rkyv             = 11,

    /// Zerocopy enabling safe zero-copy parsing with compile-time layout verification. Use when you
    /// need maximum performance for reading structured data without deserialization overhead.
    Zerocopy         = 12
}

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl TryFrom<&u8> for &'static Method {
    type Error = crate::layers::core::descriptors::Error;

    /// Converts a `&u8` word into a serialization `&Method` enum.
    ///
    /// # Errors
    /// Returns an error for unrecognized values.
    #[inline]
    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match value {
            0  => Ok(&Method::BincodeNative),
            1  => Ok(&Method::BincodeSerde),
            2  => Ok(&Method::BitcodeNative),
            3  => Ok(&Method::BitcodeSerde),
            4  => Ok(&Method::Borsh),
            5  => Ok(&Method::MessagePack),
            6  => Ok(&Method::MusliDescriptive),
            7  => Ok(&Method::MusliStorage),
            8  => Ok(&Method::MusliWire),
            9  => Ok(&Method::MusliZeroCopy),
            10 => Ok(&Method::PostcardSerde),
            11 => Ok(&Method::Rkyv),
            12 => Ok(&Method::Zerocopy),
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
            Self::MessagePack      => write!(f, "message pack"),
            Self::MusliDescriptive => write!(f, "müsli descriptive"),
            Self::MusliStorage     => write!(f, "müsli storage"),
            Self::MusliWire        => write!(f, "müsli wire"),
            Self::MusliZeroCopy    => write!(f, "müsli zero-copy"),
            Self::PostcardSerde    => write!(f, "postcard serde"),
            Self::Rkyv             => write!(f, "rkyv"),
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

    /// All variants to test
    #[test]
    fn test_method_roundtrip() {
        let methods = [
            Method::BincodeNative,
            Method::BincodeSerde,
            Method::BitcodeNative,
            Method::BitcodeSerde,
            Method::Borsh,
            Method::MessagePack,
            Method::MusliDescriptive,
            Method::MusliStorage,
            Method::MusliWire,
            Method::MusliZeroCopy,
            Method::PostcardSerde,
            Method::Rkyv,
            Method::Zerocopy,
        ];

        for method in &methods {
            // Convert to u8
            let as_u8: u8 = *method as u8;

            // Convert back via TryFrom
            let recovered = <&Method>::try_from(&as_u8)
                .unwrap_or_else(|_| panic!("failed to convert back from u8: {as_u8}"));

            // Verify round-trip
            assert_eq!(
                method, recovered,
                "round-trip failed for {:?}: {} -> {}",
                method, as_u8, *recovered as u8
            );
        }
    }

    /// Verify the expected bit-shifted values
    #[test]
    fn test_method_values() {
        assert_eq!(Method::BincodeNative as u8,    0);
        assert_eq!(Method::BincodeSerde as u8,     1);
        assert_eq!(Method::BitcodeNative as u8,    2);
        assert_eq!(Method::BitcodeSerde as u8,     3);
        assert_eq!(Method::Borsh as u8,            4);
        assert_eq!(Method::MessagePack as u8,      5);
        assert_eq!(Method::MusliDescriptive as u8, 6);
        assert_eq!(Method::MusliStorage as u8,     7);
        assert_eq!(Method::MusliWire as u8,        8);
        assert_eq!(Method::MusliZeroCopy as u8,    9);
        assert_eq!(Method::PostcardSerde as u8,    10);
        assert_eq!(Method::Rkyv as u8,             11);
        assert_eq!(Method::Zerocopy as u8,         12);
    }

    /// Test that invalid values return errors
    #[test]
    fn test_invalid_method() {
        let invalid_values = [13, 14, 15, 16, 17, 23, 25, 31, 33, 39, 41, 47, 49, 255];

        for invalid in invalid_values {
            assert!(
                <&Method>::try_from(&invalid).is_err(),
                "expected error for invalid value: {invalid}"
            );
        }
    }

    /// Test that error messages are helpful
    #[test]
    fn test_method_error_message() {
        let result = <&Method>::try_from(&99);
        assert!(result.is_err());

        let error = result.unwrap_err();
        let error_msg = error.to_string();
        assert!(error_msg.contains("unrecognized serialization method"));
        assert!(error_msg.contains("99"));
    }
}