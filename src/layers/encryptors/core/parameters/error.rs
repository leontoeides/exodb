//! Contains the error type returned from the encryption layer's parameters module.

// -------------------------------------------------------------------------------------------------
//
/// An error returned from the encryption layer's parameters module.
///
/// Parameters provide the information necessary to process encryption layers.
#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum Error {
    /// Not enough data for parameter.
    ///
    /// Expected a certain number of bytes for a parameter but not enough data was available.
    /// Indicates corrupted data or an incomplete buffer.
    #[error("not enough data to read encryption layer parameter {parameter:?}")]
    InsufficientData {
        parameter: &'static str,
        error: crate::layers::core::tail_readers::Error,
    },

    /// Invalid parameter.
    ///
    /// Failed to parse parameter from the buffer. This may indicate corrupted data in the database
    /// or may also indicate cross-platform integer-size incompatibilities.
    #[error("invalid data for encryption layer parameter {0:?}")]
    InvalidParameter(&'static str),

    /// Invalid nonce.
    ///
    /// Failed to parse the
    /// [cryptographic nonce](https://en.wikipedia.org/wiki/Cryptographic_nonce). This is likely an
    /// an internal misconfiguration.
    #[error(
        "invalid nonce: \
        expected {expected_size:?} bytes \
        but {provided_size:?} bytes were provided"
    )]
    InvalidNonce {
        expected_size: usize,
        provided_size: usize,
    },
}