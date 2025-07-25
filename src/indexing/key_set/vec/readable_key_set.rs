//! `ReadableKeySet` implementation for `vec::Keyset`

use crate::indexing::key_set::vec::KeySet;

// -------------------------------------------------------------------------------------------------
//
// Readable Key-Set Implementation

impl crate::indexing::key_set::ReadableKeySet for KeySet {
    // +----------------------+
    // | Basic Set Operations |
    // +----------------------+

    /// Returns how many primary keys are in this index set.
    ///
    /// For example, `"Tide Pools"` might return `3` if there are three known creatures found there.
    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if this index set is empty.
    ///
    /// For example, `"ISO Class 1 Cleanroom"` might return `true` because no creatures live in the
    /// habitat.
    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    // +---------------------------+
    // | Set Membership Operations |
    // +---------------------------+

    /// Returns `true` if the index contains the given primary key.
    ///
    /// For example: `key_set.contains(&hermit_crab_id_bytes)`
    ///
    /// # Notes
    ///
    /// * The primary key must be in serialized form (as raw bytes).
    #[inline]
    fn contains(&self, primary_key_bytes: &[u8]) -> bool {
        self.0.iter().any(|member| member == primary_key_bytes)
    }

    /// Returns `true` if this set is a subset of another.
    ///
    /// “Are all elements in `self` also in `other`?”
    #[inline]
    fn is_subset(&self, other: &Self) -> bool {
        self.0.iter().all(|member| other.contains(member))
    }

    /// Returns `true` if this set is a superset of another.
    ///
    /// “Are all elements in `other` also in `self`?”
    #[inline]
    fn is_superset(&self, other: &Self) -> bool {
        other.0.iter().all(|member| self.contains(member))
    }

    /// Returns `true` if this set and another intersect.
    ///
    /// “Do these sets share any elements?”
    #[inline]
    fn intersects(&self, other: &Self) -> bool {
        self.0.iter().any(|member| other.contains(member))
    }
}