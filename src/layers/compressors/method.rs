//! An enumeration that lists all available compression methods.

use crate::layers::descriptors::LAYER_MTHD_SHIFT;

// -------------------------------------------------------------------------------------------------
//
/// An enumeration that helps provide runtime identification of the compression algorithm in use,
/// allowing applications to log compression details, or store metadata about how data was processed
/// in the data pipeline.
///
/// This type is returned by the `Compressor` trait.
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
    /// Brotli compression optimized for web content and text data. Use when you need excellent
    /// compression ratios for web assets, APIs, or text-heavy data.
    Brotli  = 0 << LAYER_MTHD_SHIFT, // 00000_000b

    /// Bzip2 compression using block-sorting algorithms for high compression ratios. Use when
    /// storage space is critical and you can tolerate slower compression/decompression speeds.
    Bzip2   = 1 << LAYER_MTHD_SHIFT, // 00001_000b

    /// Deflate compression using LZ77 and Huffman coding, widely supported across platforms. Use
    /// when you need reliable compression with broad compatibility and moderate performance.
    Deflate = 2 << LAYER_MTHD_SHIFT, // 00010_000b

    /// Gzip compression wrapping `Deflate` with checksums and headers for data integrity. Use when
    /// you need deflate compression with built-in error detection and standard tooling support.
    Gzip    = 3 << LAYER_MTHD_SHIFT, // 00011_000b

    /// LZ4 compression prioritizing extremely fast decompression over compression ratio. Use when
    /// you need real-time performance and can accept larger file sizes.
    Lz4     = 4 << LAYER_MTHD_SHIFT, // 00100_000b

    /// Zlib compression wrapping deflate with error detection and cross-platform reliability. Use
    /// when you need solid general-purpose compression with good library ecosystem support.
    Zlib    = 5 << LAYER_MTHD_SHIFT, // 00101_000b

    /// Zstandard compression offering configurable speed/ratio trade-offs with excellent
    /// performance. Use when you want modern compression with tunable characteristics for diverse
    /// workloads.
    Zstd    = 6 << LAYER_MTHD_SHIFT, // 00110_000b
}

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl TryFrom<&u16> for &'static Method {
    type Error = crate::layers::descriptors::Error;

    /// Converts a `&u16` word into a compression `&Method` enum.
    ///
    /// # Errors
    /// Returns an error for unrecognized values.
    #[inline]
    fn try_from(value: &u16) -> Result<Self, Self::Error> {
        match value {
            0  => Ok(&Method::Brotli),
            8  => Ok(&Method::Bzip2),
            16 => Ok(&Method::Deflate),
            24 => Ok(&Method::Gzip),
            32 => Ok(&Method::Lz4),
            40 => Ok(&Method::Zlib),
            48 => Ok(&Method::Zstd),
            _  => Err(Self::Error::UnrecognizedCompressor(*value)),
        }
    }
}

impl std::fmt::Display for Method {
    /// Formats the compression `Method` as a human-readable string.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Brotli  => write!(f, "brotli"),
            Self::Bzip2   => write!(f, "bzip2"),
            Self::Deflate => write!(f, "deflate"),
            Self::Gzip    => write!(f, "gzip"),
            Self::Lz4     => write!(f, "lz4"),
            Self::Zlib    => write!(f, "zlib"),
            Self::Zstd    => write!(f, "zstd"),
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
            &Method::Brotli,
            &Method::Bzip2,
            &Method::Deflate,
            &Method::Gzip,
            &Method::Lz4,
            &Method::Zlib,
            &Method::Zstd,
        ];

        for method in &methods {
            // Convert to u16
            let as_u16: u16 = **method as u16;

            // Convert back via TryFrom
            let recovered = <&Method>::try_from(&as_u16)
                .expect(&format!("failed to convert back from u16: {}", as_u16));

            // Verify round-trip
            assert_eq!(
                method, &recovered,
                "round-trip failed for {:?}: {} -> {}",
                method, as_u16, *recovered as u16
            );
        }
    }

    #[test]
    fn test_method_values() {
        // Verify the expected bit-shifted values
        assert_eq!(Method::Brotli as u16, 0);
        assert_eq!(Method::Bzip2 as u16, 8);
        assert_eq!(Method::Deflate as u16, 16);
        assert_eq!(Method::Gzip as u16, 24);
        assert_eq!(Method::Lz4 as u16, 32);
        assert_eq!(Method::Zlib as u16, 40);
        assert_eq!(Method::Zstd as u16, 48);
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
        assert!(error_msg.contains("unrecognized compression method"));
        assert!(error_msg.contains("99"));
    }
}