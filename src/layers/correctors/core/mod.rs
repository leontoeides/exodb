//! Common types and traits that are used across the various error correction implementations.

mod correctable;
pub use crate::layers::correctors::core::correctable::Correctable;

mod corrector;
pub use crate::layers::correctors::core::corrector::Corrector;

mod errors;
pub use crate::layers::correctors::core::errors::Error;
pub use crate::layers::correctors::core::errors::ProtectError;
pub use crate::layers::correctors::core::errors::RecoverError;

mod level;
pub use crate::layers::correctors::core::level::Level;

mod metadata;
pub use crate::layers::correctors::core::metadata::Metadata;

mod method;
pub use crate::layers::correctors::core::method::Method;