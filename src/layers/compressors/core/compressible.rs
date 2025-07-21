//! Compression configuration trait for database storage optimization.

// -------------------------------------------------------------------------------------------------
//
/// Configures compression behavior for a specific type.
///
/// This trait determines whether and how compression should be applied when storing and retrieving
/// data of this type. Different types can have different compression strategies based on their
/// size, characteristics, access patterns, and performance requirements.
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
    /// The compression direction for this type.
    ///
    /// This constant determines when compression and decompression should be applied to values of
    /// this type. The direction affects both storage efficiency and runtime performance.
    ///
    /// # Direction Options
    ///
    /// * `None` · Never compress or decompress this type (useful for already-compressed data).
    /// * `OnRead` · Decompress on read only (data is stored pre-compressed).
    /// * `OnWrite` · Compress on write only (useful for passing compressed data to clients).
    /// * `Both` · Transparent bidirectional compression (most common for general storage).
    ///
    /// # Returns
    ///
    /// The [`Direction`] configuration applied to all values of this type.
    const DIRECTION: crate::layers::core::descriptors::Direction;

    /// The compression level for this type.
    ///
    /// This constant determines how aggressively data should be compressed when storing instances
    /// of this type. The compression level directly impacts CPU usage, storage requirements, and
    /// access performance.
    ///
    /// # Compression Levels
    ///
    /// * `Fast` · Prioritizes compression speed over ratio.
    /// * `Balanced` · Balanced compression speed and ratio.
    /// * `Maximum` · Prioritizes maximum compression ratio over speed.
    ///
    /// # Returns
    ///
    /// The [`Level`] configuration applied to all values of this type.
    const LEVEL: crate::layers::compressors::Level;
}