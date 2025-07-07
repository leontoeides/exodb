//! Compression configuration trait for database storage optimization.

// -------------------------------------------------------------------------------------------------
//
/// Configures compression level for a specific type.
///
/// This trait determines if any, and how much compression should be applied when storing and
/// retrieving data of this type. Different types can have different conditions & compression levels
/// based on their size, characteristics, access patterns, and performance requirements.
///
/// # Implementation
/// 
/// This trait is typically implemented automatically via derive macros, but can also be implemented
/// manually for custom compression strategies.
///
/// # Performance Considerations
///
/// Higher compression levels increase CPU usage and processing time but reduce storage requirements
/// and I/O overhead. Choose the level that best balances your performance and storage constraints.
pub trait Compressible {
    /// Returns the compression conditions for this type.
    ///
    /// # Example Strategies
    ///
    /// This method determines when compression or decompression should be applied to a type. For
    /// example:
    /// * `None` · Never compress or decompress this type.
    /// * `ReadOnly` · Use this when the data being written is always already compressed. Data will
    ///   be decompressed on read.
    /// * `WriteOnly` · Compress on write, and return compressed data for furtherance to HTTP client
    ///   or another node.
    /// * `Both` · Transparent & symmetric compression for the type.
    ///
    /// # Returns
    ///
    /// The [`Direction`] configuration for this type. The same directional setting is used for all
    /// instances of the implementing type.
    fn compression_direction() -> &'static crate::layers::descriptors::Direction;

    /// Returns the compression level for this type.
    ///
    /// This method determines how aggressively data should be compressed when storing instances of
    /// this type. The compression level directly impacts CPU usage, storage requirements, and
    /// access performance.
    ///
    /// # Returns
    /// 
    /// The [`Level`] configuration for this type. The same compression level is used for all
    /// instances of the implementing type.
    fn compression_level() -> &'static crate::layers::compressors::Level;
}