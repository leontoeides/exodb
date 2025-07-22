//! Byte-buffer readers that read right-to-left from mutable or immutable slices of bytes. This
//! helps read any metadata stored in encryption and ECC error correction layers.

mod error;
pub use crate::layers::core::tail_readers::error::Error;

pub mod tail_reader;
pub use crate::layers::core::tail_readers::tail_reader::TailReader;

pub mod tail_reader_bytes;
pub use crate::layers::core::tail_readers::tail_reader_bytes::TailReaderBytes;

pub mod tail_reader_mut;
pub use crate::layers::core::tail_readers::tail_reader_mut::TailReaderMut;