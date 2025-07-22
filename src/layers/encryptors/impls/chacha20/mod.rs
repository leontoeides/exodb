//! Support for [Artyom Pavlov](https://github.com/newpavlov)'s
//! [chacha20poly1305](https://crates.io/crates/chacha20poly1305) crate.

mod encryptor;

// -------------------------------------------------------------------------------------------------
//
// Constants

/// `ChaCha20Poly1305`'s key size is `32`-bytes or `256`-bits.
pub const KEY_SIZE: usize = 32;

/// `ChaCha20Poly1305`'s nonce size is `12`-bytes or `96`-bits.
pub const NONCE_SIZE: usize = 12;

/// The digest to be used when working with the `ring` crate for SHA (Secure Hash Algorithm).
#[cfg(feature = "kdf-sha256")]
pub const RING_SHA256_DIGEST: &'static ring::digest::Algorithm = &ring::digest::SHA256;

// -------------------------------------------------------------------------------------------------
//
/// ChaCha20-Poly1305 is an authenticated encryption with associated data (AEAD) algorithm that
/// combines the ChaCha20 stream cipher with the Poly1305 message authentication code.
///
/// It provides both confidentiality and integrity of data, ensuring that the encrypted data cannot
/// be accessed or tampered with by unauthorized parties. ChaCha20-Poly1305 was first introduced by
/// Daniel J. Bernstein in 2008 as a replacement for the RC4 stream cipher.
///
/// The algorithm has several key features and benefits, including high-speed encryption, with
/// speeds of up to several gigabits per second. It is designed to be resistant to side-channel
/// attacks, such as timing and power analysis attacks. Additionally, it provides authenticated
/// encryption, making it suitable for applications where data authenticity is critical. The
/// algorithm is also relatively simple to implement compared to other authenticated encryption
/// algorithms, making it easier to integrate into various cryptographic protocols and applications.
///
/// ChaCha20-Poly1305 has been extensively analyzed and tested for its security properties. The
/// algorithm has been shown to be secure against various types of attacks, including brute-force
/// attacks, side-channel attacks, and cryptanalysis. It is designed to be implemented in constant
/// time, making it resistant to timing attacks, and it uses a secure key management system to
/// ensure the secret key is properly generated, stored, and used.
///
/// ChaCha20-Poly1305 is widely used in various cryptographic protocols and applications, including
/// TLS (Transport Layer Security), WireGuard, and SSH (Secure Shell). It is also used in secure
/// messaging applications, such as Signal and WhatsApp, and for disk encryption. The algorithm is
/// known for its performance, often outperforming AES-GCM, especially on systems without AES-NI
/// instruction set extensions.
///
/// ChaCha20-Poly1305 is used in various secure communication protocols, including WireGuard, SSH,
/// and TLS 1.2, DTLS 1.2, and TLS 1.3. It is also implemented in OpenSSL and libsodium, and it is
/// used in the backup software Borg and the copy-on-write filesystem Bcachefs for optional whole
/// filesystem encryption. The algorithm is generally secure in the standard model and the ideal
/// permutation model, for the single- and multi-user setting, but it relies on choosing a unique
/// nonce for every message encrypted.
///
/// Daniel J. Bernstein designed both the `ChaCha20` stream cipher and the Poly1305 message
/// authentication code. `ChaCha20` was introduced in 2008, while Poly1305 was published in 2004.
/// The combination of these two algorithms into ChaCha20-Poly1305 was standardized in RFC 7539 and
/// later updated in RFC 8439.
#[allow(clippy::doc_markdown, reason = "it's fine")]
pub struct ChaCha20<V> {
    /// A marker to tie this `ChaCha20` structure to a specific type `V` without storing any actual
    /// data.
    phantom_data: std::marker::PhantomData<V>,
}