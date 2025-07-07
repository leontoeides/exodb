//! An enumeration that specifies when a processing direction (such as compression, encryption,
//! serialization, or error correction) should be applied for a given type.

// -------------------------------------------------------------------------------------------------
//
/// Bit-shift position of the `Direction` field in a `u16` layer descriptor.
pub const LAYER_DRCT_SHIFT: u16 = 8;

// -------------------------------------------------------------------------------------------------
//
/// Specifies when a processing direction (such as compression, encryption, serialization, or error
/// correction) should be applied for a given type.
///
/// This allows per-type control over how and when directions are engaged during database reads and
/// writes, enabling efficient or specialized data handling.
///
/// # Examples
///
/// * Use [`Direction::None`] for types that don't benefit from processing. For example, compression
///   could be turned off for very small value types.
///
/// * Use [`Direction::WriteOnly`] to store data in compressed or encrypted form and returned raw.
///   Useful for serving pre-compressed assets (such as a Brotli-encoded JPEG) to HTTP clients.
///
/// * Use [`Direction::ReadOnly`] when inserting data that's always been processed externally. This
///   might've been done on another node.
///
/// * Use [`Direction::Both`] (default) for types that should be processed transparently in both
///   directions.
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
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
#[repr(u16)]
pub enum Direction {
    /// Do not apply this layer at all, for the type.
    ///
    /// For example, compression might be unnecessary for small data structures.
    None      = 0 << LAYER_DRCT_SHIFT, // 00_00000_000b

    /// Apply this layer only during reads.
    ///
    /// Useful when the inserted data is always serialized, encrypted, or compressed elsewhere, such
    /// as another node.
    ReadOnly  = 1 << LAYER_DRCT_SHIFT, // 01_00000_000b

    /// Apply this layer only during writes.
    ///
    /// Useful when returning raw data. For example, returning an JPEG image in Brotli-compressed
    /// format without any additional processing.
    WriteOnly = 2 << LAYER_DRCT_SHIFT, // 10_00000_000b

    /// Apply this layer during both reads and writes.
    ///
    /// This is the default. It enables transparent, symmetric processing.
    #[default]
    Both      = 3 << LAYER_DRCT_SHIFT, // 11_00000_000b
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl Direction {
    /// Returns whether this layer should be evaluated on read or not.
    #[inline]
    pub fn is_read(&self) -> bool {
        matches!(self, Direction::ReadOnly | Direction::Both)
    }

    /// Returns whether this layer should be evaluated on write or not.
    #[inline]
    pub fn is_write(&self) -> bool {
        matches!(self, Direction::WriteOnly | Direction::Both)
    }
}

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl TryFrom<&u16> for &'static Direction {
    type Error = crate::layers::descriptors::Error;

    /// Converts a `&u16` word into a `&Layer` enum.
    ///
    /// # Errors
    /// Returns an error for unrecognized values.
    #[inline]
    fn try_from(value: &u16) -> Result<Self, Self::Error> {
        match value {
            0   => Ok(&Direction::None),
            256 => Ok(&Direction::ReadOnly),
            512 => Ok(&Direction::WriteOnly),
            768 => Ok(&Direction::Both),
            _   => Err(Self::Error::UnrecognizedDirection(*value)),
        }
    }
}

impl std::fmt::Display for Direction {
    /// Formats the `Direction` as a human-readable string.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None      => write!(f, "none"),
            Self::ReadOnly  => write!(f, "read-only"),
            Self::WriteOnly => write!(f, "write-only"),
            Self::Both      => write!(f, "both"),
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
    fn test_direction_roundtrip() {
        // All variants to test
        let directions = [
            Direction::None,
            Direction::ReadOnly,
            Direction::WriteOnly,
            Direction::Both,
        ];

        for direction in directions {
            // Convert to u16
            let as_u16: u16 = direction as u16;

            // Convert back via TryFrom
            let recovered = <&Direction>::try_from(&as_u16)
                .expect(&format!("failed to convert back from u16: {}", as_u16));

            // Verify round-trip
            assert_eq!(
                &direction, recovered,
                "round-trip failed for {:?}: {} -> {}",
                direction, as_u16, *recovered as u16
            );
        }
    }

    #[test]
    fn test_direction_values() {
        // Verify the expected bit-shifted values
        assert_eq!(Direction::None as u16, 0);
        assert_eq!(Direction::ReadOnly as u16, 256);
        assert_eq!(Direction::WriteOnly as u16, 512);
        assert_eq!(Direction::Both as u16, 768);
    }

    #[test]
    fn test_invalid_direction() {
        // Test that invalid values return errors
        let invalid_values = [4, 7, 9, 15, 17, 23, 25, 31, 33, 39, 41, 47, 49, 255];

        for invalid in invalid_values {
            assert!(
                <&Direction>::try_from(&invalid).is_err(),
                "expected error for invalid value: {}",
                invalid
            );
        }
    }

    #[test]
    fn test_direction_error_message() {
        // Test that error messages are helpful
        let result = <&Direction>::try_from(&99);
        assert!(result.is_err());

        let error = result.unwrap_err();
        let error_msg = error.to_string();
        assert!(error_msg.contains("unrecognized direction"));
        assert!(error_msg.contains("99"));
    }
}