//! An enumeration that lists all available error correction methods.

// -------------------------------------------------------------------------------------------------
//
/// Helps provide runtime identification of the error correction algorithm in use, allowing
/// applications to log correction details, or store metadata about how data was processed in the
/// data pipeline.
///
/// This type is returned by the `Corrector` trait.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
#[non_exhaustive]
pub enum Method {
    /// Reed-Solomon error correction using polynomial algebra over finite fields for robust
    /// recovery. Use when you need strong protection against burst errors and corruption in storage
    /// or transmission.
    ReedSolomon = 0,
}

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl TryFrom<&u8> for &'static Method {
    type Error = crate::layers::core::descriptors::Error;

    /// Converts a `&u8` word into a correction `&Method` enum.
    ///
    /// # Errors
    /// Returns an error for unrecognized values.
    #[inline]
    fn try_from(value: &u8) -> Result<Self, Self::Error> {
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

    /// All variants to test
    #[test]
    fn test_method_roundtrip() {
        let methods = [
            Method::ReedSolomon,
        ];

        for method in methods {
            // Convert to u8
            let as_u8: u8 = method as u8;

            // Convert back via TryFrom
            let recovered = <&Method>::try_from(&as_u8)
                .unwrap_or_else(|_| panic!("failed to convert back from u8: {as_u8}"));

            // Verify round-trip
            assert_eq!(
                method, *recovered,
                "round-trip failed for {:?}: {} -> {}",
                method, as_u8, *recovered as u8
            );
        }
    }

    /// Verify the expected bit-shifted values
    #[test]
    fn test_method_values() {
        assert_eq!(Method::ReedSolomon as u8, 0);
    }

    /// Test that invalid values return errors
    #[test]
    fn test_invalid_method() {
        let invalid_values = [1, 2, 7, 9, 15, 17, 23, 25, 31, 33, 39, 41, 47, 49, 255];

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
        assert!(error_msg.contains("unrecognized error correction method"));
        assert!(error_msg.contains("99"));
    }
}