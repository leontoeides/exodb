use crate::layers::core::{bytes::Error, Bytes};
use crate::layers::encryptors::{ActiveEncryptor, KeyBytes, Nonce};
use crate::layers::{Encryptable, Encryptor};

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl Bytes<'_> {
    /// Transforms readable data into an unreadable form using a secret key and unique nonce,
    /// ensuring only authorized parties can access the original information.
    ///
    /// # Arguments
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
    /// encryption and potential limitations.
    ///
    /// # Generics & Lifetimes
    ///
    /// * `V` generic represents the user's value type, for example: `User`, `String`, etc.
    /// * `b` lifetime represents bytes potentially being borrowed from the host application
    /// * 'k' lifetime represents a key potentially being borrowed from a `KeyRing` or
    ///   `KeyProvider`.
    #[inline]
    pub fn encrypt<V: Encryptable>(
        self,
        key: KeyBytes<'_>,
        nonce: Option<Nonce<'_>>
    ) -> Result<Self, Error> {
        if V::DIRECTION.is_write() {
            Ok(ActiveEncryptor::<V>::encrypt(self, key, nonce)?)
        } else {
            Ok(self)
        }
    }

    /// Reverses the encryption process using the same secret key, restoring the encrypted bytes
    /// back to their original readable form.
    ///
    /// # Arguments
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
    /// decryption and potential limitations.
    ///
    /// # Generics & Lifetimes
    ///
    /// * `V` generic represents the user's value type, for example: `User`, `String`, etc.
    /// * `b` lifetime represents bytes potentially being borrowed from the `redb` database.
    /// * 'k' lifetime represents a key potentially being borrowed from a `KeyRing` or
    ///   `KeyProvider`.
    #[inline]
    pub fn decrypt<V: Encryptable>(
        self,
        key: KeyBytes<'_>
    ) -> Result<Self, Error> {
        if V::DIRECTION.is_read() {
        	Ok(ActiveEncryptor::<V>::decrypt(self, key)?)
        } else {
            Ok(self)
        }
    }
}