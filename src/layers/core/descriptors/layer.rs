//! An enumeration that specifies when a processing layer (such as compression, encryption,
//! serialization, or error correction) should be applied for a given type.

// -------------------------------------------------------------------------------------------------
//
/// Represents a data processing layer in the storage pipeline.
///
/// Layers are applied in a specific order during writes and reversed during reads:
/// * Write Order: Serialization → Compression → Encryption → Error Correction
/// * Read Order: Error Correction → Encryption → Compression → Serialization
///
/// Each layer pushes its identifier to the end of the data buffer after processing, creating a
/// "layer stack" that enables precise error diagnostics and proper reverse processing during reads.
///
/// # Layer Stack Format
///
/// ```text
/// [processed_data][Layer::Serialization][Layer::Compression][Layer::Encryption][Layer::Correction]
/// ```
///
/// During reads, layers are popped from the end and processed in reverse order. If a layer mismatch
/// occurs, the system can generate specific error messages like "Attempting to decompress data that
/// was encrypted with `ChaCha20`".
///
/// # Error Handling
///
/// The layer stack enables actionable error messages by tracking exactly which transformations were
/// applied:
///
/// * Layer Mismatch: "Expected compression layer, found encryption layer"
/// * Missing Layer: "Data was compressed but no compression layer found in stack"
/// * Wrong Self: "Attempting to decrypt data that was never encrypted"
/// * Corrupted Stack: "Layer stack is incomplete or corrupted"
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
#[non_exhaustive]
pub enum Layer {
    /// Converts structured data to/from byte representation.
    ///
    /// Always applied first during writes, last during reads.
    Raw           = 0, // 0000b

    /// Converts structured data to/from byte representation.
    ///
    /// Always applied first during writes, last during reads.
    Serialization = 1, // 0001b

    /// Reduces data size using compression algorithms.
    ///
    /// Applied after serialization during writes. Common algorithms include `Zstd`, `LZ4`, or
    /// `Brotli` depending on feature gate selection.
    Compression   = 2, // 0010b

    /// Secures data using encryption algorithms.
    ///
    /// Applied after compression during writes. Encryption algorithms like `ChaCha20-Poly1305` or
    /// `AES-GCM` are selected via feature gates.
    Encryption    = 3, // 0011b

    /// Adds error correction codes for data integrity and recovery.
    ///
    /// Applied last during writes, using techniques like Reed-Solomon coding to create parity
    /// shards that can recover from data corruption.
    Correction    = 4, // 0100b
}

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl TryFrom<&u8> for &'static Layer {
    type Error = crate::layers::core::descriptors::Error;

    /// Converts a `&u8` word into a `&Layer` enum.
    ///
    /// # Errors
    /// Returns an error for unrecognized values.
    #[inline]
    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(&Layer::Raw),
            1 => Ok(&Layer::Serialization),
            2 => Ok(&Layer::Compression),
            3 => Ok(&Layer::Encryption),
            4 => Ok(&Layer::Correction),
            _ => Err(Self::Error::UnrecognizedLayer(*value)),
        }
    }
}

impl std::fmt::Display for Layer {
    /// Formats the `Layer` as a human-readable string.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Raw           => write!(f, "raw"),
            Self::Serialization => write!(f, "serialization"),
            Self::Compression   => write!(f, "compression"),
            Self::Encryption    => write!(f, "encryption"),
            Self::Correction    => write!(f, "correction"),
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
    fn test_layer_roundtrip() {
        let layers = [
            Layer::Raw,
            Layer::Serialization,
            Layer::Compression,
            Layer::Encryption,
            Layer::Correction,
        ];

        for layer in layers {
            // Convert to u8
            let as_u8: u8 = layer as u8;

            // Convert back via TryFrom
            let recovered = <&Layer>::try_from(&as_u8)
                .unwrap_or_else(|_| panic!("failed to convert back from u8: {as_u8}"));

            // Verify round-trip
            assert_eq!(
                &layer, recovered,
                "round-trip failed for {:?}: {} -> {}",
                layer, as_u8, *recovered as u8
            );
        }
    }

    /// Verify the expected bit-shifted values
    #[test]
    fn test_layer_values() {
        assert_eq!(Layer::Raw as u8, 0);
        assert_eq!(Layer::Serialization as u8, 1);
        assert_eq!(Layer::Compression as u8, 2);
        assert_eq!(Layer::Encryption as u8, 3);
        assert_eq!(Layer::Correction as u8, 4);
    }

    /// Test that invalid values return errors
    #[test]
    fn test_invalid_layer() {
        let invalid_values = [5, 7, 9, 15, 17, 23, 25, 31, 33, 39, 41, 47, 49, 255];

        for invalid in invalid_values {
            assert!(
                <&Layer>::try_from(&invalid).is_err(),
                "expected error for invalid value: {invalid}"
            );
        }
    }

    #[test]
    fn test_layer_error_message() {
        // Test that error messages are helpful
        let result = <&Layer>::try_from(&99);
        assert!(result.is_err());

        let error = result.unwrap_err();
        let error_msg = error.to_string();
        assert!(error_msg.contains("unrecognized layer type"));
        assert!(error_msg.contains("99"));
    }
}