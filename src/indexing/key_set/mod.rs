//! An index manages non-unique indexes & one-to-many index relationships.

// Trait representing read-only access to a set of primary keys in an index entry.

mod readable_key_set;
mod upgradable_key_set;

use crate::indexing::key_set::readable_key_set::ReadableKeySet;
use crate::indexing::key_set::upgradable_key_set::UpgradableKeySet;

// -------------------------------------------------------------------------------------------------
//
// Key Set Feature Guard

/// Helper macro: counts how many of the listed features are turned on.
macro_rules! count_features {
    ($($feat:literal),* $(,)?) => {
        0_usize $(+ cfg!(feature = $feat) as usize)*
    };
}

const _KEY_SET_FEATURE_COUNT: usize = count_features!(
    "ahash-key-set",
    "b-tree-key-set",
    "hash-key-set",
    "vec-key-set",
);

const _: () = {
    assert!(
        // Only one key-set index feature can be enabled. To fix: 1. open your `Cargo.toml` file, 2.
        // find `exodb` under `[dependencies]`, 3. ensure only one key-set index feature is enabled.
        !(_KEY_SET_FEATURE_COUNT > 1),
        "Multiple key-set features enabled! Please enable only one of: \
        `ahash-key-set`, \
        `b-tree-key-set`, \
        `hash-key-set`, or \
        `vec-key-set`",
    );
};

// -------------------------------------------------------------------------------------------------
//
// Key Set Implementations

// ahash-backed index sets

#[cfg(feature = "ahash-key-set")]
pub(super) mod ahash_set;

#[cfg(feature = "ahash-key-set")]
pub use crate::indexing::key_set::ahash_set::{ArchivedKeySet, KeySet};

// HashSet-backed index sets

#[cfg(feature = "hash-key-set")]
pub(super) mod hash_set;

#[cfg(feature = "hash-key-set")]
pub use crate::indexing::key_set::hash_set::{ArchivedKeySet, KeySet};

// BTreeSet-backed index sets

#[cfg(feature = "b-tree-key-set")]
pub(super) mod b_tree_set;

#[cfg(feature = "b-tree-key-set")]
pub use crate::indexing::key_set::b_tree_set::{ArchivedKeySet, KeySet};

// Vec-backed index sets

#[cfg(feature = "vec-key-set")]
pub(super) mod vec;

#[cfg(feature = "vec-key-set")]
pub use crate::indexing::key_set::vec::{ArchivedKeySet, KeySet};