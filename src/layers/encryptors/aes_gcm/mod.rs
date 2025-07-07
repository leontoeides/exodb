//! Support for [Tony Arcieri](https://github.com/tarcieri)'s
//! [aes-gcm](https://crates.io/crates/aes-gcm) crate.

mod encryptor;

// -------------------------------------------------------------------------------------------------
//
/// Pure Rust implementation of the AES-GCM
/// [Authenticated Encryption with Associated Data (AEAD)](https://en.wikipedia.org/wiki/Authenticated_encryption)
/// cipher.
///
/// All implementations contained in the crate are designed to execute in constant time, either by
/// relying on hardware intrinsics (i.e. AES-NI and CLMUL on x86/x86_64), or using a portable
/// implementation which is only constant time on processors which implement constant-time
/// multiplication.
///
/// It is not suitable for use on processors with a variable-time multiplication operation (e.g.
/// short circuit on multiply-by-zero / multiply-by-one, such as certain 32-bit PowerPC CPUs and
/// some non-ARM microcontrollers).
pub struct AesGcm<V> {
    /// A marker to tie this `AesGcm` structure to a specific type `V` without storing any actual
    /// data.
    phantom_data: std::marker::PhantomData<V>,
}