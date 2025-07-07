/// An error returned from the compression layer.
///
/// This includes errors for out of memory, corrupted or malformed data, etc.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// An error was encountered while compressing data.
    #[error("compression of data failed")]
    Compress { #[from] #[source] source: crate::layers::compressors::CompressError },

    /// An error was encountered while decompressing data.
    #[error("decompression of data failed")]
    Decompress { #[from] #[source] source: crate::layers::compressors::DecompressError },
}