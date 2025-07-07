//! Implementations for `vec::ArchivedKeySet`, the zero-copy key-set view.

use crate::indexing::key_set::{vec::{ArchivedKeySet, KeySet}, ReadableKeySet};
use rkyv::vec::ArchivedVec;

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl ArchivedKeySet {
    // +---------------+
    // | Basic Methods |
    // +---------------+

    /// Returns a borrowed iterator over the primary keys in the index set.
    ///
    /// # Notes
    ///
    /// * The primary keys will be returned in serialized form, as raw bytes. If needed, each key
    ///   can be deserialized into its full form by using `K::deserialize(item)`
    ///
    /// * Primary keys are used to get actual records from the database.
    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, ArchivedVec<u8>> {
        self.0.iter()
    }

    /// Returns the owned, inner collection of primary keys.
    #[inline]
    #[must_use]
    pub const fn into_inner(self) -> ArchivedVec<ArchivedVec<u8>> {
        self.0
    }

    /// Instantiates an `ArchivedKeySet` from its binary representation.
    ///
    /// # Errors
    ///
    /// * This method will return an error if `rkyv`'s access check of the `ArchivedKeySet` fails.
    #[inline]
    pub fn from_bytes(bytes: &[u8]) -> Result<&Self, crate::Error> {
        let archived = rkyv::access::<Self, rkyv::rancor::Error>(bytes)?;
        Ok(archived)
    }
}

// -------------------------------------------------------------------------------------------------
//
// Readable Key-Set Implementation

impl ReadableKeySet for &ArchivedKeySet {
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

// -------------------------------------------------------------------------------------------------
//
// Upgradable Key-Set Implementation

impl crate::indexing::key_set::UpgradableKeySet for &ArchivedKeySet {
    /// Upgrades the [`ArchivedKeySet`] into an owned & mutable [`KeySet`] by completing the
    /// `rkyv` deserialization process, if necessary.
    ///
    /// This method is typically used when write access is required to complete a set operation.
    ///
    /// # Errors
    ///
    /// * This method will return an error if deserialization of the underlying [`ArchivedKeySet`]
    ///   fails.
    #[inline]
    fn upgrade(self) -> Result<KeySet, crate::Error> {
        let deserialized = rkyv::deserialize::<KeySet, rkyv::rancor::Error>(self)?;
        Ok(deserialized)
    }
}

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl std::ops::Deref for ArchivedKeySet {
    type Target = ArchivedVec<ArchivedVec<u8>>;

    /// Dereferences an `ArchivedKeySet` into its underlying `ArchivedVec<ArchivedVec<u8>>`
    /// collection.
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for ArchivedKeySet {
    /// Dereferences an `ArchivedKeySet` into its underlying `ArchivedVec<ArchivedVec<u8>>`
    /// collection.
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'i> IntoIterator for &'i ArchivedKeySet {
    type Item = &'i ArchivedVec<u8>;
    type IntoIter = std::slice::Iter<'i, ArchivedVec<u8>>;

    /// Returns an borrowed iterator over the primary keys in the index set.
    ///
    /// # Notes
    ///
    /// * The primary keys will be returned in serialized form, as raw bytes. If needed, each key
    ///   can be deserialized into its full form by using `K::deserialize(item)`
    ///
    /// * Primary keys are used to get actual records from the database.
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}