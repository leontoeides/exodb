use crate::layers::core::Bytes;

// -------------------------------------------------------------------------------------------------
//
/// The `Corrector` trait provides error correction and data recovery functionality as part of an
/// optional data processing pipeline.
///
/// This pipeline can include: disk storage → ECC repair → decryption → decompression →
/// deserialization (for reads) or the reverse for writes, with each stage being optional and
/// potentially zero-copy.
///
/// The trait enables automatic detection and correction of data corruption that can occur during
/// storage, transmission, or due to hardware failures.
///
/// # Generics & Lifetimes
///
/// * `V` generic represents the user's value type, for example: `Creature`, `User`, `String`, etc.
/// * `b` lifetime represents bytes potentially being borrowed from the host application or the
///   `redb` database.
///
/// # Key Concepts
///
/// ## Pipeline Integration
///
/// This trait operates within a larger data processing pipeline where error correction is just one
/// optional stage. The zero-copy design allows for efficient processing of large datasets while
/// maintaining security properties throughout the pipeline.
///
/// ## `Bytes` and Zero-Copy Design
///
/// The `Bytes<'b>` wrapper allows the error correction layer to work with borrowed data from the
/// database without unnecessary copying, improving performance for large datasets.
pub trait Corrector<'b, V: crate::layers::correctors::Correctable> {
    /// Returns the error correction method that the current `Corrector` trait implements.
    ///
    /// This enables runtime identification of the error correction algorithm in use, allowing
    /// applications to log compression details, or store metadata about how data was processed in
    /// the data pipeline.
    const METHOD: crate::layers::correctors::Method;

    /// ECC error protection works by adding parity data, which are extra symbols calculated from
    /// the original data, to enable the recovery of lost or corrupted data symbols during
    /// transmission or storage.
    ///
    /// If protection is unnecessary or cannot be applied, the original `Bytes` will be returned
    /// untouched.
    ///
    /// # Arguments
    ///
    /// * `unprotected_bytes` · The original data to be protected against corruption, wrapped in a
    ///   `Bytes` that may reference borrowed application bytes.
    ///
    /// # Errors
    ///
    /// Consult the documentation of the corrector backend you are using for more detail on
    /// protection behavior and potential limitations.
    ///
    /// # Generics & Lifetimes
    ///
    /// * `V` generic represents the user's value type, for example: `User`, `String`, etc.
    /// * `b` lifetime represents bytes potentially being borrowed from the host application.
    fn protect(
        unprotected_bytes: Bytes<'b>
    ) -> Result<Bytes<'b>, crate::layers::correctors::ProtectError>;

    /// ECC error recovery works by encoding data into blocks with additional parity blocks,
    /// allowing the reconstruction of lost or corrupted data as long as the number of errors does
    /// not exceed the number of parity blocks.
    ///
    /// CRC-32 is used to detect data corruption by generating a 32-bit checksum that can identify
    /// changes in the data during transmission or storage.
    ///
    /// # Arguments
    ///
    /// * `protected_bytes` · Data that has been previously protected and may contain corruption,
    ///   which the method will attempt to detect and repair
    ///
    /// # Errors
    ///
    /// This method may fail for several reasons, including:
    ///
    /// * Input bytes are corrupted or malformed.
    ///
    /// Consult the documentation of the corrector backend you are using for more detail on recovery
    /// behavior and potential limitations.
    ///
    /// # Generics & Lifetimes
    ///
    /// * `V` generic represents the user's value type, for example: `User`, `String`, etc.
    /// * `b` lifetime represents bytes potentially being borrowed from the `redb` database.
    fn recover(
        protected_bytes: Bytes<'b>
    ) -> Result<Bytes<'b>, crate::layers::correctors::RecoverError>;
}