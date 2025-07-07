use crate::layers::ValueBuf;

// -------------------------------------------------------------------------------------------------
//
/// The `Corrector` trait provides error correction and data recovery functionality as part of an
/// optional data processing pipeline. This pipeline can include: disk storage → ECC repair →
/// decryption → decompression → deserialization (for reads) or the reverse for writes, with each
/// stage being optional and potentially zero-copy.
///
/// The trait enables automatic detection and correction of data corruption that can occur during
/// storage, transmission, or due to hardware failures.
///
/// # Generics & Lifetimes
///
/// * `V` generic represents the user's value type, for example: `Creature`, `String`, etc.
/// * `b` lifetime represents bytes potentially being borrowed from the `redb` database.
///
/// # Key Concepts
///
/// ## Pipeline Integration
///
/// This trait operates within a larger data processing pipeline where error correction is just one
/// optional stage. The zero-copy design allows for efficient processing of large datasets while
/// maintaining security properties throughout the pipeline.
///
/// ## ValueBuf and Zero-Copy Design
///
/// The `ValueBuf<'b>` wrapper allows the error correction layer to work with borrowed data from the
/// database without unnecessary copying, improving performance for large datasets.
pub trait Corrector<'b, V: crate::layers::correctors::Correctable> {
    /// Adds parity data to the original bytes, creating redundant information that enables
    /// automatic detection and correction of future corruption.
    ///
    /// If protection is unnecessary or cannot be applied, the original `ValueBuf` will be returned
    /// untouched.
    ///
    /// # Arguments
    ///
    /// * `unprotected_bytes` · The original data to be protected against corruption, wrapped in a
    ///   `ValueBuf` that may reference borrowed database bytes.
    ///
    /// # Errors
    ///
    /// Consult the documentation of the corrector backend you are using for more detail on
    /// protection behavior and potential limitations.
    ///
    /// # Generics & Lifetimes
    ///
    /// * `V` generic represents the user's value type, for example: `Creature`, `String`, etc.
    /// * `b` lifetime represents bytes potentially being borrowed from the `redb` database.
    fn protect(
        unprotected_bytes: ValueBuf<'b>
    ) -> Result<ValueBuf<'b>, crate::layers::correctors::ProtectError>;

    /// Analyzes protected data for corruption and automatically repairs any detected errors using
    /// the embedded parity information, or returns the original data if no corruption is found.
    ///
    /// This method may return the original buffer untouched if no corruption is detected, or may
    /// allocate a repaired version if any issues are found.
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
    /// * `V` generic represents the user's value type, for example: `Creature`, `String`, etc.
    /// * `b` lifetime represents bytes potentially being borrowed from the `redb` database.
    fn recover(
        protected_bytes: ValueBuf<'b>
    ) -> Result<ValueBuf<'b>, crate::layers::correctors::RecoverError>;

    /// Returns the error correction method that the current `Corrector` trait implements.
    ///
    /// This enables runtime identification of the error correction algorithm in use, allowing
    /// applications to log compression details, or store metadata about how data was processed in
    /// the data pipeline.
    fn method() -> &'static crate::layers::correctors::Method;
}