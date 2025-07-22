//! BLAKE3 Key Derivation Function (KDF) using [Jack O'Connor](https://github.com/oconnor663)'s
//! [blake3](https://crates.io/crates/blake3) crate.

use crate::layers::encryptors::{kdf::Key, KEY_SIZE};
use std::borrow::Cow;

// -------------------------------------------------------------------------------------------------
//
// Constants

/// # [Context](https://docs.rs/blake3/latest/blake3/fn.derive_key.html)
///
/// > The context string should be hardcoded, globally unique, and application-specific. A good
/// > default format for such strings is `"[application] [commit timestamp] [purpose]"`, e.g.,
/// > `"example.com 2019-12-25 16:18:03 session tokens v1`.
///
/// **Warning**: This context string must never change. It is permanently bound to encrypted data
/// and cannot be rotated without full decryption of all data with original keys. Changing it would
/// render all existing encrypted data unrecoverable.
const CONTEXT: &'static str = "atlatl:encryption:kdf";

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl<'k> Key<'k> {
    /// Converts a `Key` into a fixed-length `&[u8; KEY_SIZE]` array that can be provided to
    /// encryption ciphers.
    ///
    /// If the initially provided key was a string, the string will be hashed into a digest value
    /// using [Jack O'Connor](https://github.com/oconnor663)'s
    /// [blake3](https://crates.io/crates/blake3) crate at this stage.
    pub fn into_array(&'k self) -> Cow<'k, [u8; KEY_SIZE]> {
        self.into()
    }
}

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl<'k> From<&'k Key<'k>> for Cow<'k, [u8; KEY_SIZE]> {
    /// Converts a `Key` into a fixed-length `&[u8; KEY_SIZE]` array that can be provided to
    /// encryption ciphers.
    ///
    /// If the initially provided key was a string, the string will be hashed into a digest value
    /// using [Jack O'Connor](https://github.com/oconnor663)'s
    /// [blake3](https://crates.io/crates/blake3) crate at this stage.
    fn from(key: &'k Key<'k>) -> Cow<'k, [u8; KEY_SIZE]> {
        match key {
            Key::String(string) => {
                let key = blake3::derive_key(CONTEXT, string.as_bytes());
                Cow::Owned(key.as_ref().try_into().unwrap())
            },
            Key::Bytes(bytes) => {
                Cow::Borrowed(bytes)
            }
        }
    }
}