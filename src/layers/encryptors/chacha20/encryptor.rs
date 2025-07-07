//! Support for [Artyom Pavlov](https://github.com/newpavlov)'s
/// [chacha20poly1305](https://crates.io/crates/chacha20poly1305) crate.

use chacha20poly1305::{aead::{Aead, AeadCore, KeyInit, OsRng}, ChaCha20Poly1305};
use crate::layers::descriptors::Direction;
use crate::layers::encryptors::{chacha20::ChaCha20, Encryptable, Encryptor, Parameters, Method};
use crate::layers::{TailReader, ValueBuf};

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl<'b, V: Encryptable> Encryptor<'b> for ChaCha20<V> {
    /// Transforms readable data into an unreadable form using a secret key and unique nonce,
    /// ensuring only authorized parties can access the original information.
    ///
    /// # Arguments
    ///
    /// * `unencrypted_bytes` · The original data to be encrypted, wrapped in a `ValueBuf` that may
    ///   reference borrowed database bytes.
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
    /// recovery behavior and potential limitations: <https://http://docs.rs/chacha20poly1305>
    ///
    /// # Generics & Lifetimes
    ///
    /// * `V` generic represents the user's value type, for example: `Creature`, `String`, etc.
    /// * `b` lifetime represents bytes potentially being borrowed from the `redb` database.
    #[inline]
    fn encrypt(
        plain_text: ValueBuf<'b>,
        nonce: Option<&[u8]>,
        key: &[u8]
    ) -> Result<ValueBuf<'b>, crate::layers::encryptors::EncryptError> {
        let cipher = ChaCha20Poly1305::new(key.into());
        if let Some(nonce) = nonce {
            let mut cipher_text = cipher.encrypt(nonce.into(), plain_text.as_slice())?;
            let metadata = Parameters::from_nonce(&nonce);
            metadata.to_data_buffer(&mut cipher_text)?;
            Ok(cipher_text.into())
        } else {
            let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bits; unique per message
            let mut cipher_text = cipher.encrypt((&nonce).into(), plain_text.as_slice())?;
            let metadata = Parameters::from_nonce(&nonce);
            metadata.to_data_buffer(&mut cipher_text)?;
            Ok(cipher_text.into())
        }
    }

    /// Reverses the encryption process using the same secret key, restoring the encrypted bytes
    /// back to their original readable form.
    ///
    /// # Arguments
    ///
    /// * `encrypted_bytes` · The encrypted data to be decrypted, wrapped in a `ValueBuf`.
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
    /// recovery behavior and potential limitations: <https://http://docs.rs/chacha20poly1305>
    ///
    /// # Generics & Lifetimes
    ///
    /// * `V` generic represents the user's value type, for example: `Creature`, `String`, etc.
    /// * `b` lifetime represents bytes potentially being borrowed from the `redb` database.
    #[inline]
    fn decrypt(
    	cipher_text: ValueBuf<'b>,
    	key: &[u8]
    ) -> Result<ValueBuf<'b>, crate::layers::encryptors::DecryptError> {
        // `TailReader` is used to read metadata from the end of the `ValueBuf` buffer:
        let mut tail_reader = TailReader::from_slice(cipher_text.as_slice());
        let metadata = Parameters::from_data_buffer(&mut tail_reader)?;

        // We have the `Parameters` now. It contains the nonce which can be used to decrypt with:
        let cipher = ChaCha20Poly1305::new(key.into());
        let plain_text = cipher.decrypt(metadata.nonce.into(), tail_reader.as_ref())?;

        Ok(plain_text.into())
    }

    /// Returns the encryption method that the current `Encryptor` trait implements.
    ///
    /// This enables runtime identification of the encryption algorithm in use, allowing
    /// applications to log compression details, or store metadata about how data was processed in
    /// the data pipeline.
    #[inline]
    fn method() -> &'static Method {
        &Method::ChaCha20
    }
}

// -------------------------------------------------------------------------------------------------
//
// Tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layers::descriptors::Direction;

    // Test types implementing Encryptable with different directions
    struct AlwaysEncrypt;
    impl Encryptable for AlwaysEncrypt {
        fn encryption_direction() -> &'static Direction {
            &Direction::Both
        }
    }

    #[test]
    fn test_symmetric_encryption_both_encryption_direction() {
        let key = b"an example very very secret key."; // 32 bytes
        let original_data = b"Hello, World! This is a test message.";
        let value_buf = ValueBuf::from(original_data.as_slice());

        // Encrypt
        let encrypted = AesGcm::<AlwaysEncrypt>::encrypt(value_buf, None, key)
            .expect("Encryption should succeed");

        // Verify data is actually encrypted (different from original)
        assert_ne!(encrypted.as_slice(), original_data);

        // Decrypt
        let decrypted = AesGcm::<AlwaysEncrypt>::decrypt(encrypted, key)
            .expect("Decryption should succeed");

        // Verify decrypted data matches original
        assert_eq!(decrypted.as_slice(), original_data);
    }

    #[test]
    fn test_nonce_uniqueness() {
        let key = b"an example very very secret key."; // 32 bytes
        let original_data = b"Hello, World! This is a test message.";
        let value_buf = ValueBuf::from(original_data.as_slice());

        // Encrypt the same data multiple times
        let encrypted1 = AesGcm::<AlwaysEncrypt>::encrypt(value_buf.clone(), None, key)
            .expect("First encryption should succeed");
        let encrypted2 = AesGcm::<AlwaysEncrypt>::encrypt(value_buf.clone(), None, key)
            .expect("Second encryption should succeed");

        // Encrypted data should be different due to different nonces
        assert_ne!(encrypted1.as_slice(), encrypted2.as_slice());

        // But both should decrypt to the same original data
        let decrypted1 = AesGcm::<AlwaysEncrypt>::decrypt(encrypted1, key)
            .inspect_err(|error| println!("{:#?}", error))
            .expect("First decryption should succeed");
        let decrypted2 = AesGcm::<AlwaysEncrypt>::decrypt(encrypted2, key)
            .inspect_err(|error| println!("{:#?}", error))
            .expect("Second decryption should succeed");

        assert_eq!(decrypted1.as_slice(), original_data);
        assert_eq!(decrypted2.as_slice(), original_data);
    }

    #[test]
    fn test_empty_data() {
        let key = b"an example very very secret key."; // 32 bytes
        let empty_data = b"";
        let value_buf = ValueBuf::from(empty_data.as_slice());

        let encrypted = AesGcm::<AlwaysEncrypt>::encrypt(value_buf, None, key)
            .inspect_err(|error| println!("{:#?}", error))
            .expect("Encryption of empty data should succeed");

        let decrypted = AesGcm::<AlwaysEncrypt>::decrypt(encrypted, key)
            .inspect_err(|error| println!("{:?}", error))
            .expect("Decryption of empty data should succeed");

        assert_eq!(decrypted.as_slice(), empty_data);
    }

    #[test]
    fn test_large_data() {
        let key = b"an example very very secret key."; // 32 bytes
        let large_data = vec![0x42u8; 10000]; // 10KB of data
        let value_buf = ValueBuf::from(large_data.as_slice());

        let encrypted = AesGcm::<AlwaysEncrypt>::encrypt(value_buf, None, key)
            .expect("Encryption of large data should succeed");

        let decrypted = AesGcm::<AlwaysEncrypt>::decrypt(encrypted, key)
            .expect("Decryption of large data should succeed");

        assert_eq!(decrypted.as_slice(), large_data.as_slice());
    }

    #[test]
    fn test_wrong_key_fails() {
        let key1 = b"an example very very secret key."; // 32 bytes
        let key2 = b"another example very secret key."; // 32 bytes
        let original_data = b"Hello, World! This is a test message.";
        let value_buf = ValueBuf::from(original_data.as_slice());

        // Encrypt with key1
        let encrypted = AesGcm::<AlwaysEncrypt>::encrypt(value_buf, None, key1)
            .expect("Encryption should succeed");

        // Try to decrypt with key2 (should fail)
        let result = AesGcm::<AlwaysEncrypt>::decrypt(encrypted, key2);
        assert!(result.is_err(), "Decryption with wrong key should fail");
    }

    #[test]
    fn test_corrupted_data_fails() {
        let key = b"an example very very secret key."; // 32 bytes
        let original_data = b"Hello, World! This is a test message.";
        let value_buf = ValueBuf::from(original_data.as_slice());

        // Encrypt data
        let mut encrypted = AesGcm::<AlwaysEncrypt>::encrypt(value_buf, None, key)
            .expect("Encryption should succeed")
            .to_vec();

        // Corrupt the encrypted data
        if let Some(byte) = encrypted.get_mut(0) {
            *byte = byte.wrapping_add(1);
        }

        // Try to decrypt corrupted data (should fail)
        let result = AesGcm::<AlwaysEncrypt>::decrypt(encrypted.into(), key);
        assert!(result.is_err(), "Decryption of corrupted data should fail");
    }

    #[test]
    fn test_method_returns_correct_value() {
        assert_eq!(AesGcm::<AlwaysEncrypt>::method(), &Method::AesGcm);
    }
}