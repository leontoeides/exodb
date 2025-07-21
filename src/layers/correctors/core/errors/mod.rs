//! Error types used across the various error correction implementations.

mod protect;
pub use crate::layers::correctors::core::errors::protect::Error as ProtectError;

mod recover;
pub use crate::layers::correctors::core::errors::recover::Error as RecoverError;

mod correction;
pub use crate::layers::correctors::core::errors::correction::Error;