//! An enumeration that lists all available compression methods.

// -------------------------------------------------------------------------------------------------
//
/// Helps provide runtime identification of the compression algorithm in use, allowing applications
/// to log compression details, or store metadata about how data was processed in the data pipeline.
///
/// This type is returned by the `Compressor` trait.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
#[non_exhaustive]
pub enum Method {
    /// Brotli compression optimized for web content and text data. Use when you need excellent
    /// compression ratios for web assets, APIs, or text-heavy data.
    Brotli  = 0,

    /// Bzip2 compression using block-sorting algorithms for high compression ratios. Use when
    /// storage space is critical and you can tolerate slower compression/decompression speeds.
    Bzip2   = 1,

    /// Deflate compression using LZ77 and Huffman coding, widely supported across platforms. Use
    /// when you need reliable compression with broad compatibility and moderate performance.
    Deflate = 2,

    /// Gzip compression wrapping `Deflate` with checksums and headers for data integrity. Use when
    /// you need deflate compression with built-in error detection and standard tooling support.
    Gzip    = 3,

    /// LZ4 compression prioritizing extremely fast decompression over compression ratio. Use when
    /// you need real-time performance and can accept larger file sizes.
    Lz4     = 4,

    /// Zlib compression wrapping deflate with error detection and cross-platform reliability. Use
    /// when you need solid general-purpose compression with good library ecosystem support.
    Zlib    = 5,

    /// Zstandard compression offering configurable speed/ratio trade-offs with excellent
    /// performance. Use when you want modern compression with tunable characteristics for diverse
    /// workloads.
    Zstd    = 6,
}

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl TryFrom<&u8> for &'static Method {
    type Error = crate::layers::core::descriptors::Error;

    /// Converts a `&u8` word into a compression `&Method` enum.
    ///
    /// # Errors
    /// Returns an error for unrecognized values.
    #[inline]
    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(&Method::Brotli),
            1 => Ok(&Method::Bzip2),
            2 => Ok(&Method::Deflate),
            3 => Ok(&Method::Gzip),
            4 => Ok(&Method::Lz4),
            5 => Ok(&Method::Zlib),
            6 => Ok(&Method::Zstd),
            _ => Err(Self::Error::UnrecognizedCompressor(*value)),
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

    /// All variants to test
    #[test]
    fn test_method_roundtrip() {
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
            // Convert to u8
            let as_u8: u8 = **method as u8;

            // Convert back via TryFrom
            let recovered = <&Method>::try_from(&as_u8)
                .unwrap_or_else(|_| panic!("failed to convert back from u8: {as_u8}"));

            // Verify round-trip
            assert_eq!(
                method, &recovered,
                "round-trip failed for {:?}: {} -> {}",
                method, as_u8, *recovered as u8
            );
        }
    }

    /// Verify the expected bit-shifted values
    #[test]
    fn test_method_values() {
        assert_eq!(Method::Brotli as u8,   0);
        assert_eq!(Method::Bzip2 as u8,    1);
        assert_eq!(Method::Deflate as u8,  2);
        assert_eq!(Method::Gzip as u8,     3);
        assert_eq!(Method::Lz4 as u8,      4);
        assert_eq!(Method::Zlib as u8,     5);
        assert_eq!(Method::Zstd as u8,     6);
    }

    /// Test that invalid values return errors
    #[test]
    fn test_invalid_method() {
        let invalid_values = [7, 9, 15, 17, 23, 25, 31, 33, 39, 41, 47, 49, 255];

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
        assert!(error_msg.contains("unrecognized compression method"));
        assert!(error_msg.contains("99"));
    }
}