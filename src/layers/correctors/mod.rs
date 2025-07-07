//! Error correction mechanisms to detect and repair corrupted data.

mod correctable;
pub use crate::layers::correctors::correctable::Correctable;

mod corrector;
pub use crate::layers::correctors::corrector::Corrector;

mod error;
pub use crate::layers::correctors::error::Error;

mod metadata;
pub use crate::layers::correctors::metadata::Metadata;

mod method;
pub use crate::layers::correctors::method::Method;

mod protect_error;
pub use crate::layers::correctors::protect_error::ProtectError;

mod level;
pub use crate::layers::correctors::level::Level;

mod recover_error;
pub use crate::layers::correctors::recover_error::RecoverError;

// -------------------------------------------------------------------------------------------------
//
// Error Correction Implementations

#[cfg(feature = "ecc-reed-solomon")]
mod reed_solomon;