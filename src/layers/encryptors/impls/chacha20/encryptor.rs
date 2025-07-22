//! Support for [Artyom Pavlov](https://github.com/newpavlov)'s
//! [chacha20poly1305](https://crates.io/crates/chacha20poly1305) crate.

use chacha20poly1305::{aead::{Aead, AeadCore, AeadInPlace, KeyInit, OsRng}, ChaCha20Poly1305};
use crate::layers::core::{Bytes, tail_readers::{TailReader, TailReaderMut}};
use crate::layers::encryptors::core::{
    Encryptable,
    EncryptError,
    Encryptor,
    KeyBytes,
    Method,
    Nonce,
    Parameters
};
use crate::layers::encryptors::impls::chacha20::ChaCha20;
use std::borrow::Cow;

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl<'b, 'k, V: Encryptable> Encryptor<'b, 'k, V> for ChaCha20<V> {
    /// Returns the encryption method that the current `Encryptor` trait implements.
    ///
    /// This enables runtime identification of the encryption algorithm in use, allowing
    /// applications to log compression details, or store metadata about how data was processed in
    /// the data pipeline.
    const METHOD: Method = Method::ChaCha20;

    /// Transforms readable data into an unreadable form using a secret key and unique nonce,
    /// ensuring only authorized parties can access the original information.
    ///
    /// # Arguments
    ///
    /// * `plain_text` · The original data to be encrypted, wrapped in a `Bytes` that may
    ///   reference borrowed application bytes.
    ///
    /// * `nonce` · A unique value used once per encryption operation to ensure the same `plaintext`
    ///   produces different `ciphertext`.
    ///
    ///   If no nonce is provided, one will be randomly generated but this behaviour is not
    ///   recommended for tables with more than 4,294,967,296 entries. [NIST SP 800-38D] recommends
    ///   the following:
    ///
    ///   > The total number of invocations of the authenticated encryption function shall not
    ///   > exceed 2^32, including all IV lengths and all instances of the authenticated encryption
    ///   > function with the given key.
    ///
    ///   [NIST SP 800-38D]: https://csrc.nist.gov/publications/detail/sp/800-38d/final
    ///
    /// * `key` · The secret encryption key used to transform the data.
    ///
    /// # Errors
    ///
    /// Consult the documentation of the encryptor backend you are using for more detail on
    /// recovery behavior and potential limitations: <https://docs.rs/aes-gcm>
    ///
    /// # Generics & Lifetimes
    ///
    /// * `V` generic represents the user's value type, for example: `User`, `String`, etc.
    /// * `b` lifetime represents bytes potentially being borrowed from the host application.
    /// * 'k' lifetime represents a key potentially being borrowed from a `KeyRing` or
    ///   `KeyProvider`.
    #[inline]
    fn encrypt(
        plain_text: Bytes<'b>,
        key: KeyBytes<'k>,
        nonce: Option<Nonce<'k>>,
    ) -> Result<Bytes<'b>, EncryptError> {
        let cipher = ChaCha20Poly1305::new(key.as_ref().into());
        if let Some(nonce) = nonce {
            let mut cipher_text = cipher.encrypt(nonce.as_ref().into(), plain_text.as_slice())?;
            Parameters::from_nonce(nonce).into_data_buffer(&mut cipher_text);
            Ok(cipher_text.into())
        } else {
            let nonce = Nonce::from_array(ChaCha20Poly1305::generate_nonce(&mut OsRng).into());
            let mut cipher_text = cipher.encrypt(nonce.as_ref().into(), plain_text.as_slice())?;
            Parameters::from_nonce(nonce).into_data_buffer(&mut cipher_text);
            Ok(cipher_text.into())
        }
    }

    /// Reverses the encryption process using the same secret key, restoring the encrypted bytes
    /// back to their original readable form.
    ///
    /// # Arguments
    ///
    /// * `cipher_text` · The encrypted data to be decrypted, wrapped in a `Bytes`.
    ///
    /// * `key` · The same secret key used during encryption, required to reverse the
    ///   transformation.
    ///
    /// # Errors
    ///
    /// This method may fail for several reasons, including:
    ///
    /// * Invalid key, or
    /// * Input bytes are corrupted or malformed.
    ///
    /// Consult the documentation of the encryptor backend you are using for more detail on
    /// recovery behavior and potential limitations: <https://docs.rs/aes-gcm>
    ///
    /// # Generics & Lifetimes
    ///
    /// * `V` generic represents the user's value type, for example: `User`, `String`, etc.
    /// * `b` lifetime represents bytes potentially being borrowed from the `redb` database.
    /// * 'k' lifetime represents a key potentially being borrowed from a `KeyRing` or
    ///   `KeyProvider`.
    #[inline]
    fn decrypt(
        cipher_text: Bytes<'b>,
        key: KeyBytes<'k>
    ) -> Result<Bytes<'b>, crate::layers::encryptors::DecryptError> {
        let (metadata, cipher_text) = cipher_text.into_parts();
        let cipher = ChaCha20Poly1305::new(key.as_ref().into());

        match cipher_text {
            Cow::Borrowed(slice) => {
                // Borrowed: use out-of-place decryption (allocates only for plaintext)
                let mut tail_reader = TailReader::from_slice(slice);
                let parameters = Parameters::from_data_buffer(&mut tail_reader)?;
                let plain_text = cipher.decrypt(parameters.nonce.as_ref().into(), tail_reader.close())?;
                Ok(Bytes::from_parts(metadata, plain_text.into()))
            }
            Cow::Owned(vec) => {
                // Owned: decrypt in-place (no extra allocation)
                let mut tail_reader_mut = TailReaderMut::from_vec(vec);
                let nonce = *Parameters::from_data_buffer_mut(&mut tail_reader_mut)?.nonce;
                let mut bytes_buf: Vec<u8> = tail_reader_mut.close();
                cipher.decrypt_in_place(nonce.as_ref().into(), &[], &mut bytes_buf)?;
                Ok(Bytes::from_parts(metadata, bytes_buf.into()))
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------
//
// Tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layers::core::Direction;

    // Test types implementing Encryptable with different directions
    struct AlwaysEncrypt;
    impl Encryptable for AlwaysEncrypt {
        const DIRECTION: Direction = Direction::Both;
    }

    #[test]
    fn test_symmetric_encryption_both_encryption_direction() {
        let key = b"an example very very secret key."; // 32 bytes
        let original_data = b"Hello, World! This is a test message.";
        let value_buf = Bytes::from(original_data.as_slice());

        // Encrypt
        let encrypted = ChaCha20::<AlwaysEncrypt>::encrypt(value_buf, key.into(), None)
            .expect("Encryption should succeed");

        // Verify data is actually encrypted (different from original)
        assert_ne!(encrypted.as_slice(), original_data);

        // Decrypt
        let decrypted = ChaCha20::<AlwaysEncrypt>::decrypt(encrypted, key.into())
            .expect("Decryption should succeed");

        // Verify decrypted data matches original
        assert_eq!(decrypted.as_slice(), original_data);
    }

    #[test]
    fn test_nonce_uniqueness() {
        let key = b"an example very very secret key."; // 32 bytes
        let original_data = b"Hello, World! This is a test message.";
        let value_buf = Bytes::from(original_data.as_slice());

        // Encrypt the same data multiple times
        let encrypted1 = ChaCha20::<AlwaysEncrypt>::encrypt(value_buf.clone(), key.into(), None)
            .expect("First encryption should succeed");
        let encrypted2 = ChaCha20::<AlwaysEncrypt>::encrypt(value_buf.clone(), key.into(), None)
            .expect("Second encryption should succeed");

        // Encrypted data should be different due to different nonces
        assert_ne!(encrypted1.as_slice(), encrypted2.as_slice());

        // But both should decrypt to the same original data
        let decrypted1 = ChaCha20::<AlwaysEncrypt>::decrypt(encrypted1, key.into())
            .inspect_err(|error| println!("{error:#?}"))
            .expect("First decryption should succeed");
        let decrypted2 = ChaCha20::<AlwaysEncrypt>::decrypt(encrypted2, key.into())
            .inspect_err(|error| println!("{error:#?}"))
            .expect("Second decryption should succeed");

        assert_eq!(decrypted1.as_slice(), original_data);
        assert_eq!(decrypted2.as_slice(), original_data);
    }

    #[test]
    fn test_empty_data() {
        let key = b"an example very very secret key."; // 32 bytes
        let empty_data = b"";
        let value_buf = Bytes::from(empty_data.as_slice());

        let encrypted = ChaCha20::<AlwaysEncrypt>::encrypt(value_buf, key.into(), None)
            .inspect_err(|error| println!("{error:#?}"))
            .expect("Encryption of empty data should succeed");

        let decrypted = ChaCha20::<AlwaysEncrypt>::decrypt(encrypted, key.into())
            .inspect_err(|error| println!("{error:?}"))
            .expect("Decryption of empty data should succeed");

        assert_eq!(decrypted.as_slice(), empty_data);
    }

    #[test]
    fn test_large_data() {
        let key = b"an example very very secret key."; // 32 bytes
        let large_data = vec![0x42u8; 10000]; // 10KB of data
        let value_buf = Bytes::from(large_data.as_slice());

        let encrypted = ChaCha20::<AlwaysEncrypt>::encrypt(value_buf, key.into(), None)
            .expect("Encryption of large data should succeed");

        let decrypted = ChaCha20::<AlwaysEncrypt>::decrypt(encrypted, key.into())
            .expect("Decryption of large data should succeed");

        assert_eq!(decrypted.as_slice(), large_data.as_slice());
    }

    #[test]
    fn test_wrong_key_fails() {
        let key1: KeyBytes = b"an example very very secret key.".into(); // 32 bytes
        let key2: KeyBytes = b"another example very secret key.".into(); // 32 bytes
        let original_data = b"Hello, World! This is a test message.";
        let value_buf = Bytes::from(original_data.as_slice());

        // Encrypt with key1
        let encrypted = ChaCha20::<AlwaysEncrypt>::encrypt(value_buf, key1, None)
            .expect("Encryption should succeed");

        // Try to decrypt with key2 (should fail)
        let result = ChaCha20::<AlwaysEncrypt>::decrypt(encrypted, key2);
        assert!(result.is_err(), "Decryption with wrong key should fail");
    }

    #[test]
    fn test_corrupted_data_fails() {
        let key = b"an example very very secret key."; // 32 bytes
        let original_data = b"Hello, World! This is a test message.";
        let value_buf = Bytes::from(original_data.as_slice());

        // Encrypt data
        let mut encrypted = ChaCha20::<AlwaysEncrypt>::encrypt(value_buf, key.into(), None)
            .expect("Encryption should succeed")
            .to_vec();

        // Corrupt the encrypted data
        if let Some(byte) = encrypted.get_mut(0) {
            *byte = byte.wrapping_add(1);
        }

        // Try to decrypt corrupted data (should fail)
        let result = ChaCha20::<AlwaysEncrypt>::decrypt(encrypted.into(), key.into());
        assert!(result.is_err(), "Decryption of corrupted data should fail");
    }

    #[test]
    fn test_method_returns_correct_value() {
        assert_eq!(ChaCha20::<AlwaysEncrypt>::METHOD, Method::ChaCha20);
    }
}