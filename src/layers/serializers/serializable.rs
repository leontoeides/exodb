//! Serialization configuration trait for database storage.

// -------------------------------------------------------------------------------------------------
//
/// Configures serialization for a specific type.
///
/// This trait determines if serialization should be applied when storing and retrieving data of 
/// this type. Different types can have different serialization conditions based on their access 
/// patterns.
///
/// # Implementation
/// 
/// This trait is typically implemented automatically via derive macros, but can also be implemented
/// manually for custom serialization strategies.
pub trait Serializable {
    /// Returns the serialization conditions for this type.
    ///
    /// # Example Strategies
    ///
    /// This method determines when serialization or decryption should be applied to a type. For
    /// example:
    /// * `None` · Never serialize or deserialize this type.
    /// * `ReadOnly` · Use this when the data being written is always already serialized. Data will
    ///   be deserialized on read.
    /// * `WriteOnly` · Serialize on write and return serialized data for furtherance, for example,
    ///   to another node.
    /// * `Both` · Transparent & symmetric serialization for the type.
    ///
    /// # Returns
    ///
    /// The [`Direction`] configuration for this type. The same directional setting is used for all
    /// instances of the implementing type.
    fn serialization_direction() -> &'static crate::layers::descriptors::Direction;
}