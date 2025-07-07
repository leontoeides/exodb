//! An enumeration that allows the compression-level for each type to be set individually.

// -------------------------------------------------------------------------------------------------
//
/// Compression level settings for balancing speed vs. compression ratio.
///
/// These levels are implementation-dependent and provide different trade-offs between compression
/// speed and final size reduction. The optimal choice depends on your use case and performance
/// requirements.
#[derive(Copy, Clone, Default, Eq, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum Level {
    /// Prioritizes compression speed over ratio.
    ///
    /// * Speed: Fast
    /// * Compression ratio: Low to moderate
    /// * CPU usage: Low
    /// * Best for: Real-time applications or frequent compression operations
    #[default]
    Fast = 0,

    /// Balanced compression speed and ratio.
    ///
    /// * Speed: Moderate
    /// * Compression ratio: Good
    /// * CPU usage: Moderate
    /// * Best for: General-purpose compression where both speed and size matter
    Balanced = 1,

    /// Prioritizes maximum compression ratio over speed.
    ///
    /// * Speed: Slow
    /// * Compression ratio: Highest available
    /// * CPU usage: High
    /// * Best for: Archival storage or bandwidth-constrained scenarios
    Maximum = 2,
}