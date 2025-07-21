//! Support for [Brian Smith](https://github.com/briansmith)'s [ring](https://crates.io/crates/ring)
//! crate.

use crate::layers::encryptors::{kdf::Key, KEY_SIZE};
use std::borrow::Cow;

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl<'k> Key<'k> {
    /// Converts a `Key` into a fixed-length `&[u8; KEY_SIZE]` array that can be provided to
    /// encryption ciphers.
    ///
    /// If the initially provided key was a string, the string will be hashed into a digest value
    /// using [Brian Smith](https://github.com/briansmith)'s [ring](https://crates.io/crates/ring)
    /// at this stage.
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
    /// using [Brian Smith](https://github.com/briansmith)'s [ring](https://crates.io/crates/ring)
    /// at this stage.
    fn from(key: &'k Key<'k>) -> Cow<'k, [u8; KEY_SIZE]> {
        match key {
            Key::String(string) => {
                let digest = ring::digest::digest(
                    crate::layers::encryptors::RING_SHA256_DIGEST,
                    string.as_bytes()
                );
                Cow::Owned(digest.as_ref().try_into().unwrap()) // SHA-256 is always 32 bytes
            },
            Key::Bytes(bytes) => {
                Cow::Borrowed(bytes)
            }
        }
    }
}