//! An enumeration that lists all available encryption methods.

// -------------------------------------------------------------------------------------------------
//
/// Helps provide runtime identification of the encryption algorithm in use, allowing applications
/// to log encryption details, or store metadata about how data was processed in the data pipeline.
///
/// This type is returned by the `Encryptor` trait.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
#[repr(u8)]
#[non_exhaustive]
pub enum Method {
    /// `AES-GCM` combining AES block cipher with Galois Counter Mode for authenticated encryption.
    /// Use when you need hardware-accelerated performance and built-in authentication on modern
    /// CPUs.
    AesGcm   = 0,

    /// `ChaCha20Poly1305` stream cipher offering high performance and resistance to timing attacks.
    /// Use when you need fast encryption with strong security guarantees on diverse hardware.
    ChaCha20 = 1,
}

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl TryFrom<&u8> for &'static Method {
    type Error = crate::layers::core::descriptors::Error;

    /// Converts a `&u8` word into an encryption `&Method` enum.
    ///
    /// # Errors
    /// Returns an error for unrecognized values.
    #[inline]
    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(&Method::AesGcm),
            1 => Ok(&Method::ChaCha20),
            _ => Err(Self::Error::UnrecognizedEncryptor(*value)),
        }
    }
}

impl std::fmt::Display for Method {
    /// Formats the encryption `Method` as a human-readable string.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ChaCha20  => write!(f, "chacha20poly1305"),
            Self::AesGcm    => write!(f, "aes-gcm"),
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
            Method::AesGcm,
            Method::ChaCha20,
        ];

        for method in methods {
            // Convert to u8
            let as_u8: u8 = method as u8;

            // Convert back via TryFrom
            let recovered = <&Method>::try_from(&as_u8)
                .unwrap_or_else(|_| panic!("failed to convert back from u8: {as_u8}"));

            // Verify round-trip
            assert_eq!(
                &method, recovered,
                "round-trip failed for {:?}: {} -> {}",
                method, as_u8, *recovered as u8
            );
        }
    }

    /// Verify the expected bit-shifted values
    #[test]
    fn test_method_values() {
        assert_eq!(Method::AesGcm as u8,   0);
        assert_eq!(Method::ChaCha20 as u8, 1);
    }

    /// Test that invalid values return errors
    #[test]
    fn test_invalid_method() {
        let invalid_values = [2, 3, 4, 5, 6, 7, 9, 15, 17, 23, 25, 31, 33, 39, 41, 47, 49, 255];

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
        assert!(error_msg.contains("unrecognized encryption method"));
        assert!(error_msg.contains("99"));
    }
}