//! LZ4 compression using [Pascal Seitz](https://github.com/PSeitz)'s
//! [lz4_flex](https://crates.io/crates/lz4_flex) crate.

#[cfg(feature = "compress-dictionaries")]
mod dictionary;

#[cfg(not(feature = "compress-dictionaries"))]
mod standard;

// -------------------------------------------------------------------------------------------------
//
/// LZ4 is a lossless data compression algorithm that focuses on providing high-speed compression
/// and decompression.
///
/// It is part of the LZ77 family of byte-oriented compression schemes and is designed to offer a
/// good trade-off between speed and compression ratio.
///
/// LZ4 is particularly popular in real-time applications where quick compression and decompression
/// are more important than achieving the highest possible compression ratio, such as network
/// traffic compression, real-time storage systems, and data transfer protocols.
///
/// The algorithm is known for its extremely fast decoder, with speeds in multiple GB/s per core,
/// and it can be scaled with multi-core CPUs. Additionally, LZ4 supports dictionary compression,
/// which can improve compression performance on small files.
pub struct Lz4Flex<V> {
    /// A marker to tie this `Lz4Flex` compressor to a specific type `V` without storing any actual
    /// data.
    phantom_data: std::marker::PhantomData<V>
}