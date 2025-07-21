//! An enumeration that specifies when a processing direction (such as compression, encryption,
//! serialization, or error correction) should be applied for a given type.

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
/// * Use [`Direction::OnWrite`] to store data in compressed or encrypted form and returned raw.
///   Useful for serving pre-compressed assets (such as a Brotli-encoded JPEG) to HTTP clients.
///
/// * Use [`Direction::OnRead`] when inserting data that's always been processed externally. This
///   might've been done on another node.
///
/// * Use [`Direction::Both`] (default) for types that should be processed transparently in both
///   directions.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
#[repr(u8)]
pub enum Direction {
    /// Do not apply this layer at all, for the type.
    ///
    /// For example, compression might be unnecessary for small data structures.
    None    = 0,

    /// Apply this layer only during reads.
    ///
    /// Useful when the inserted data is always serialized, encrypted, or compressed elsewhere, such
    /// as another node.
    OnRead  = 1,

    /// Apply this layer only during writes.
    ///
    /// Useful when returning raw data. For example, returning an JPEG image in Brotli-compressed
    /// format without any additional processing.
    OnWrite = 2,

    /// Apply this layer during both reads and writes.
    ///
    /// This is the default. It enables transparent, symmetric processing.
    #[default]
    Both    = 3,
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl Direction {
    /// Returns whether this layer should be evaluated on read or not.
    #[inline]
    #[must_use]
    pub const fn is_read(self) -> bool {
        matches!(self, Self::OnRead | Self::Both)
    }

    /// Returns whether this layer should be evaluated on write or not.
    #[inline]
    #[must_use]
    pub const fn is_write(self) -> bool {
        matches!(self, Self::OnWrite | Self::Both)
    }
}

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl TryFrom<&u8> for &'static Direction {
    type Error = crate::layers::core::descriptors::Error;

    /// Converts a `&u8` word into a `&Layer` enum.
    ///
    /// # Errors
    /// Returns an error for unrecognized values.
    #[inline]
    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(&Direction::None),
            1 => Ok(&Direction::OnRead),
            2 => Ok(&Direction::OnWrite),
            3 => Ok(&Direction::Both),
            _ => Err(Self::Error::UnrecognizedDirection(*value)),
        }
    }
}

impl std::fmt::Display for Direction {
    /// Formats the `Direction` as a human-readable string.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None    => write!(f, "none"),
            Self::OnRead  => write!(f, "read-only"),
            Self::OnWrite => write!(f, "write-only"),
            Self::Both    => write!(f, "both"),
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
    fn test_direction_roundtrip() {
        let directions = [
            Direction::None,
            Direction::OnRead,
            Direction::OnWrite,
            Direction::Both,
        ];

        for direction in directions {
            // Convert to u8
            let as_u8: u8 = direction as u8;

            // Convert back via TryFrom
            let recovered = <&Direction>::try_from(&as_u8)
                .unwrap_or_else(|_| panic!("failed to convert back from u8: {as_u8}"));

            // Verify round-trip
            assert_eq!(
                &direction, recovered,
                "round-trip failed for {:?}: {} -> {}",
                direction, as_u8, *recovered as u8
            );
        }
    }

    /// Verify the expected bit-shifted values
    #[test]
    fn test_direction_values() {
        assert_eq!(Direction::None as u8,    0);
        assert_eq!(Direction::OnRead as u8,  1);
        assert_eq!(Direction::OnWrite as u8, 2);
        assert_eq!(Direction::Both as u8,    3);
    }

    /// Test that invalid values return errors
    #[test]
    fn test_invalid_direction() {
        let invalid_values = [4, 7, 9, 15, 17, 23, 25, 31, 33, 39, 41, 47, 49, 255];

        for invalid in invalid_values {
            assert!(
                <&Direction>::try_from(&invalid).is_err(),
                "expected error for invalid value: {invalid}"
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