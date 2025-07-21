//! Types of layer metadata. Including the type of layer (compression, encryption, etc.), the
//! layer's implementation (Brotli, LZ4, etc.), and the direction application (read-only,
//! write-only, etc.).

pub mod direction;
pub use crate::layers::core::descriptors::direction::Direction;

pub mod error;
pub use crate::layers::core::descriptors::error::Error;

pub mod layer;
pub use crate::layers::core::descriptors::layer::Layer;