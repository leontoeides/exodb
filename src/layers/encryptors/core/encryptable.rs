//! Encryption configuration trait for database storage.

// -------------------------------------------------------------------------------------------------
//
/// Configures encryption for a specific type.
///
/// This trait determines if encryption should be applied when storing and retrieving data of this
/// type. Different types can have different encryption conditions based on their access patterns.
///
/// # Implementation
/// 
/// This trait is typically implemented automatically via derive macros, but can also be implemented
/// manually for custom encryption strategies.
pub trait Encryptable {
    /// Returns the encryption conditions for this type.
    ///
    /// # Example Strategies
    ///
    /// This method determines when encryption or decryption should be applied to a type. For
    /// example:
    /// * `None` · Never encrypt or decrypt this type.
    /// * `OnRead` · Use this when the data being written is always already encrypted. Data will
    ///   be decrypted on read.
    /// * `OnWrite` · Encrypt on write and return encrypted data for furtherance, for example, to
    ///   another node.
    /// * `Both` · Transparent & symmetric encryption for the type.
    ///
    /// # Returns
    ///
    /// The [`Direction`] configuration for this type. The same directional setting is used for all
    /// values of this type.
    const DIRECTION: crate::layers::core::descriptors::Direction;
}