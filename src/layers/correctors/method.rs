//! An enumeration that lists all available error correction methods.

use crate::layers::descriptors::LAYER_MTHD_SHIFT;

// -------------------------------------------------------------------------------------------------
//
/// An enumeration that helps provide runtime identification of the error correction algorithm in
/// use, allowing applications to log correction details, or store metadata about how data was
/// processed in the data pipeline.
///
/// This type is returned by the `Corrector` trait.
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
    /// Reed-Solomon error correction using polynomial algebra over finite fields for robust
    /// recovery. Use when you need strong protection against burst errors and corruption in storage
    /// or transmission.
    ReedSolomon = 0 << LAYER_MTHD_SHIFT, // 00000_000b
}

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl TryFrom<&u16> for &'static Method {
    type Error = crate::layers::descriptors::Error;

    /// Converts a `&u16` word into a correction `&Method` enum.
    ///
    /// # Errors
    /// Returns an error for unrecognized values.
    #[inline]
    fn try_from(value: &u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(&Method::ReedSolomon),
            _  => Err(Self::Error::UnrecognizedCorrector(*value)),
        }
    }
}

impl std::fmt::Display for Method {
    /// Formats the error correction `Method` as a human-readable string.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ReedSolomon => write!(f, "reed-solomon"),
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
            Method::ReedSolomon,
        ];

        for method in methods {
            // Convert to u16
            let as_u16: u16 = method as u16;

            // Convert back via TryFrom
            let recovered = <&Method>::try_from(&as_u16)
                .expect(&format!("failed to convert back from u16: {}", as_u16));

            // Verify round-trip
            assert_eq!(
                method, *recovered,
                "round-trip failed for {:?}: {} -> {}",
                method, as_u16, *recovered as u16
            );
        }
    }

    #[test]
    fn test_method_values() {
        // Verify the expected bit-shifted values
        assert_eq!(Method::ReedSolomon as u16, 0);
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
        assert!(error_msg.contains("unrecognized error correction method"));
        assert!(error_msg.contains("99"));
    }
}