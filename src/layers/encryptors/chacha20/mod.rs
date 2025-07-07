//! Support for [Artyom Pavlov](https://github.com/newpavlov)'s
//! [chacha20poly1305](https://crates.io/crates/chacha20poly1305) crate.

mod encryptor;

// -------------------------------------------------------------------------------------------------
//
/// ChaCha20Poly1305 ([RFC 8439](https://tools.ietf.org/html/rfc8439)): an
/// [Authenticated Encryption with Associated Data (AEAD)](https://en.wikipedia.org/wiki/Authenticated_encryption)
/// cipher amenable to fast, constant-time implementations in software, based on the
/// [ChaCha20](https://github.com/RustCrypto/stream-ciphers/tree/master/chacha20) stream cipher and
/// [Poly1305](https://github.com/RustCrypto/universal-hashes/tree/master/poly1305) universal hash
/// function.
pub struct ChaCha20<V> {
    /// A marker to tie this `ChaCha20` structure to a specific type `V` without storing any actual
    /// data.
    phantom_data: std::marker::PhantomData<V>,
}