//! A layer descriptor is a type of metadata. It contains information on the type of layer
//! (compression, encryption, etc.), the layer's implementation (Brotli, LZ4, etc.), and the
//! direction application (apply in both directions, do not apply, apply on read, apply on write).

pub mod descriptor;
pub use crate::layers::descriptors::descriptor::Descriptor;

pub mod direction;
pub use crate::layers::descriptors::direction::Direction;

pub mod error;
pub use crate::layers::descriptors::error::Error;

pub mod layer;
pub use crate::layers::descriptors::layer::Layer;

// -------------------------------------------------------------------------------------------------
//
/// Bit-shift position of the `Method` field in a `u16` layer descriptor.
pub const LAYER_MTHD_SHIFT: u16 = 3;