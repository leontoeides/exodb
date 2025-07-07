   /*
                                          ░░
                                        ░░▓▓░░
                                  ░▒▓██▓▓▒▒▒▒▓▓██▓▒░
                               ░▒█▓▒▒▒▒▒▓▒░█▒▓▒▒▒▒▒▓█▒░
                             ░▓▓▒▒▒▓▒▓▒▓▓▒░█▒▓▓▓▓▓▒▒▓▒▓▓▒
                           ░██▒▓▒▒▒▓█▓▒░░▒▒▒▓░░▒▓█▓▒▒▒▒▒▓█░
                          ▒█▒▒▓▓▓▓▓░     ▒▓█▒     ░▓▓▓▓▒▒▒█▒
                         ▒█▒▓▒▓▓▓░      ▒▓▓█▓▒      ░▓▓▒▒▓▒█▒
                        ░█▒▓▒▓▓▒       ▒█▓▓█▓█▒       ▒█▓▓▒▒█░
                       ░▓▓▒▒▓▓▒       ▒▓▓▓▓▓▓▓▓▒       ▒▓▓▒▒▓█░
                       ░█▒▒▓▒█░      ▒▓█▓█▓██▓█▓▒      ░█▒▓▒▒█▒
                       ▒▓▒▒▒▓▒      ░▓██▓▒▒█▒▓██▓░      ▒▓▒▓▒▓▒
                       ▒▓▒▒▒█▒     ░█▓██▓▓▒█▓▓███▓░     ▒█▒▓▒▓▒
                       ▒▓▒▒▓▓▓    ░▓▓█▓█▒▒▒█▒▒█▓█▓▓░    ▓▓▓▒▒▓▒
                       ░█▒▒▓▒█░  ░▓██▓▒  ░▒█░  ▒▓██▓░  ░█▒▒▒▒█░
                        ▓█▒▒▒▓▓░░▓▒▓▒░   ░▒█░   ░▒▓▒▓░░▓▓▓▒▒▓▓
                         █▓▒▒▒▓▓▓▒▓░     ░▒█░     ░▓▒▒▓▓▓▒▓▓█
                         ░▓▓▒▒▒▒█▒░      ░▒█░      ░▒█▓▒▒▒▓▓░
                          ░▓▓▒▒▒▓▓▓▓░    ░▒█░    ░▓▓▓▓▒▒▒▓▓░
                            ░█▓▒▓▒▓▓▒██▓▒▒▒█▒▒▓██▒▓▓▒▓▒▓█░
                             ░▒▓▓▓▒▒▒▒▓▒▒▒▒█▓▒▒▓▒▒▒▒▓▓▓▒░
                                ░▒▓█▓▒▒▒▒▒▒█▒▒▒▒▒▓██▒░
                                    ░▒▓███▓████▓▒░
                                          ▒▓░
                                          ░▒

                                      A T L A T L
*/

#![warn(
   clippy::all,
   clippy::cargo,
   clippy::nursery,
   clippy::pedantic,
   clippy::style,
)]

// #[cfg(debug_assertions)]
// debug_assert_eq!(cfg!(target_endian = "little"), true, "Atlatl only supports little-endian targets.");

#[cfg(any(
   feature = "serializers",
   feature = "compressors",
   feature = "correctors",
   feature = "encryptors",
))]
pub mod layers;

// #[cfg(feature = "redb-pass-through")]
// pub mod redb;

// mod error;
// pub use crate::error::Error;

// pub mod indexing;
// pub mod querying;

// mod data_buf;
// pub use crate::data_buf::ValueBuf;

// pub mod typed;