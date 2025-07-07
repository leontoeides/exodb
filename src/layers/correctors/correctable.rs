//! Error correction configuration trait for database storage protection.

// -------------------------------------------------------------------------------------------------
//
/// Configures error correction protection level for a specific type.
///
/// This trait determines if any, and how much error correction protection should be applied when
/// storing and retrieving your type. Different types can have different protection levels based on
/// their importance and corruption tolerance requirements.
///
/// # Implementation
///
/// This trait is typically implemented automatically via derive macros, but can also be implemented
/// manually for custom protection strategies.
///
/// # Performance Considerations
///
/// Higher protection levels increase storage overhead and processing time. Choose the minimum level
/// that meets your data integrity requirements.
pub trait Correctable {
    /// Returns the protection conditions for this type.
    ///
    /// # Example Strategies
    ///
    /// This method determines when protection or recovery should be applied to a type. For example:
    /// * `None` · Never protect, check, or recover this type.
    /// * `ReadOnly` · Use this when the data being written is always already protected with parity
    ///   shards. Data will be checked and potentially recovered on read.
    /// * `WriteOnly` · Protect data on write, but return the data still protected by parity-shards
    ///   for furtherance to another node.
    /// * `Both` · Transparent & symmetric protection, checking, and recovery for the type.
    ///
    /// # Returns
    ///
    /// The [`Direction`] configuration for this type. The same directional setting is used for all
    /// instances of the implementing type.
    fn correction_direction() -> &'static crate::layers::descriptors::Direction;

    /// Returns the error correction protection level for this type.
    ///
    /// This method determines how much redundancy will be generated alongside the original data
    /// when storing instances of this type. The protection level directly impacts storage overhead
    /// and corruption recovery capabilities.
    ///
    /// # Returns
    ///
    /// The [`Level`] configuration for this type. The same protection level is used for
    /// all instances of the implementing type.
    fn level() -> &'static crate::layers::correctors::Level;
}