use crate::layers::core::Bytes;
use crate::layers::encryptors::{KeyBytes, Nonce};

// -------------------------------------------------------------------------------------------------
//
/// The `Encryptor` trait provides symmetric encryption and decryption functionaliy as part of an
/// data processing pipeline.
///
/// This pipeline can include: disk storage → ECC repair → decryption → decompression →
/// deserialization (for reads) or the reverse for writes, with each stage being optional and
/// potentially zero-copy.
///
/// Implementations operate on serialized and/or compressed byte slices, returning encrypted bytes
/// or decrypted bytes (original data).
///
/// # Generics & Lifetimes
///
/// * `V` generic represents the user's value type, for example: `Creature`, `User`, `String`, etc.
/// * `b` lifetime represents bytes potentially being borrowed from the host application or the
///   `redb` database.
/// * 'k' lifetime represents a key potentially being borrowed from a `KeyRing` or `KeyProvider`.
///
/// # Key Concepts
///
/// ## Pipeline Integration
///
/// This trait operates within a larger data processing pipeline where encryption is just one
/// optional stage. The zero-copy design allows for efficient processing of large datasets while
/// maintaining security properties throughout the pipeline.
///
/// ## `Bytes` and Zero-Copy Design
///
/// The `Bytes<'b>` wrapper allows the encryption layer to work with borrowed data from the
/// database without unnecessary copying, improving performance for large datasets.
///
/// # Considerations
///
/// ## Key Management
///
/// * Generate keys securely: Use cryptographically secure random number generators.
/// * Store keys safely: Never store keys alongside encrypted data; use dedicated key management
///   systems.
/// * Rotate keys regularly: Implement key rotation policies to limit exposure from potential
///   compromises.
/// * Secure key transmission: Use secure channels when distributing keys.
///
/// ## Nonce Handling
///
/// * Uniqueness is critical: Never reuse a nonce with the same key, as this can leak information
///   about the __plaintext__.
/// * Storage: Nonces can be stored alongside encrypted data since they don't need to remain secret.
/// * Generation: Use secure random generation or deterministic counters to ensure uniqueness.
///
/// ## Implementation Security
///
/// * Memory safety: Ensure sensitive data is cleared from memory when no longer needed.
/// * Error handling: Avoid revealing information through error messages that could aid attackers.
///
/// # Operational Security
///
/// * Authentication: Consider using authenticated encryption modes that detect tampering.
/// * Key derivation: Use proper key derivation functions when deriving encryption keys from
///   passwords.
/// * Audit trail: Log encryption/decryption operations for security monitoring (without logging
///   sensitive data)
pub trait Encryptor<'b, 'k, V> {
    /// Returns the encryption method that the current `Encryptor` trait implements.
    ///
    /// This enables runtime identification of the encryptor algorithm in use, allowing
    /// applications to log compression details, or store metadata about how data was processed in
    /// the data pipeline.
    const METHOD: crate::layers::encryptors::Method;

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
    /// encryption and potential limitations.
    ///
    /// # Generics & Lifetimes
    ///
    /// * `V` generic represents the user's value type, for example: `User`, `String`, etc.
    /// * `b` lifetime represents bytes potentially being borrowed from the host application.
    /// * 'k' lifetime represents a key potentially being borrowed from a `KeyRing` or
    ///   `KeyProvider`.
    fn encrypt(
        plain_text: Bytes<'b>,
        key: KeyBytes<'k>,
        nonce: Option<Nonce<'k>>
    ) -> Result<Bytes<'b>, crate::layers::encryptors::EncryptError>;

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
    /// decryption and potential limitations.
    ///
    /// # Generics & Lifetimes
    ///
    /// * `V` generic represents the user's value type, for example: `User`, `String`, etc.
    /// * `b` lifetime represents bytes potentially being borrowed from the `redb` database.
    /// * 'k' lifetime represents a key potentially being borrowed from a `KeyRing` or
    ///   `KeyProvider`.
    fn decrypt(
        cipher_text: Bytes<'b>,
        key: KeyBytes<'k>
    ) -> Result<Bytes<'b>, crate::layers::encryptors::DecryptError>;
}