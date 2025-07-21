//! Support for [Tony Arcieri](https://github.com/tarcieri)'s
//! [aes-gcm](https://crates.io/crates/aes-gcm) crate.

mod encryptor;

// -------------------------------------------------------------------------------------------------
//
// Constants

/// `AesGcm`'s key size is `32`-bytes or `256`-bits.
pub const KEY_SIZE: usize = 32;

/// `AesGcm`'s nonce size is `12`-bytes or `96`-bits.
pub const NONCE_SIZE: usize = 12;

/// The digest to be used when working with the `ring` crate for SHA (Secure Hash Algorithm).
#[cfg(feature = "kdf-sha256")]
pub const RING_SHA256_DIGEST: &'static ring::digest::Algorithm = &ring::digest::SHA256;

// -------------------------------------------------------------------------------------------------
//
/// AES-GCM (Advanced Encryption Standard - Galois/Counter Mode) is a symmetric key encryption
/// algorithm that combines encryption with authentication.
///
/// It ensures data confidentiality and verifies that the data hasn't been tampered with. AES-GCM is
/// widely used in secure communication protocols like TLS. The benefits and features of AES-GCM
/// include:
///
/// * Confidentiality and Integrity: AES-GCM provides both data confidentiality and integrity,
///   making it suitable for secure communication.
///
/// * Performance: AES-GCM is known for its performance, as it can take full advantage of parallel
///   processing and implementing GCM can make efficient use of an instruction pipeline or a
///   hardware pipeline.
///
/// * Authenticated Encryption: AES-GCM belongs to the class of authenticated encryption with
///   associated data (AEAD) methods, which means it takes a key, some plaintext, and some
///   associated data to produce ciphertext and an authentication tag.
///
/// * Efficiency: AES-GCM requires one block cipher operation and one 128-bit multiplication in the
///   Galois field per each block of encrypted and authenticated data.
///
/// AES-GCM was designed by John Viega and David A. McGrew to be an improvement to Carter-Wegman
/// counter mode (CWC mode). It was officially standardized by NIST in November 2007 with the
/// release of NIST Special Publication 800-38D Recommendation for Block Cipher Modes of Operation:
/// Galois/Counter Mode (GCM) and GMAC.
#[allow(clippy::doc_markdown, reason = "respect David A. McGrew's name")]
pub struct AesGcm<V> {
    /// A marker to tie this `AesGcm` structure to a specific type `V` without storing any actual
    /// data.
    phantom_data: std::marker::PhantomData<V>,
}