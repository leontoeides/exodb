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
#![allow(
    clippy::multiple_crate_versions, // Due to upstream crates, can't do much about this
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

// pub mod db;

mod error;
pub use crate::error::Error;

// pub mod indexing;
// pub mod querying;

// pub mod typed;