//! Error correction algorithms for detecting and repairing corrupted data.

mod core;
pub use crate::layers::correctors::core::Correctable;
pub use crate::layers::correctors::core::Corrector;
pub use crate::layers::correctors::core::Error;
pub use crate::layers::correctors::core::Level;
pub use crate::layers::correctors::core::Metadata;
pub use crate::layers::correctors::core::Method;
pub use crate::layers::correctors::core::ProtectError;
pub use crate::layers::correctors::core::RecoverError;

mod impls;
pub use crate::layers::correctors::impls::ActiveCorrector;