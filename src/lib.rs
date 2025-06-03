#![warn(
   clippy::all,
   clippy::cargo,
   clippy::nursery,
   clippy::pedantic,
   clippy::style,
)]

pub mod codecs;
pub use crate::codecs::Codec;

mod error;
pub use crate::error::Error;

pub mod indexing;

// pub mod store;
//pub mod typed;
// pub mod indexing;

// pub use codec::*;
// pub use store::*;
// pub use index::*;

// #[cfg(feature = "indicium")]
// pub mod indicium_support;

// Optional future extension for auto-complete + fuzzy search
// #[cfg(feature = "indicium")]
// pub use indicium_support::*;
