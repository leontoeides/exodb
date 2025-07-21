mod bytes;
pub use crate::layers::core::bytes::Bytes;

pub(crate) mod descriptors;
pub use crate::layers::core::descriptors::Direction;
pub use crate::layers::core::descriptors::Layer;

pub(crate) mod tail_readers;

mod value;
pub use crate::layers::core::value::Value;

mod value_or_bytes;
pub use crate::layers::core::value_or_bytes::ValueOrBytes;