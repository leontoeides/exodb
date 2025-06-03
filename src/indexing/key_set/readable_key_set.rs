//! Trait that provides read-only operations to a set of primary keys from an index entry.

/// Trait that provides read-only operations to a set of primary keys from an index entry.
///
/// This trait is implemented by both [`KeySet`] (owned and mutable) and [`ArchivedKeySet`]
/// (zero-copy view).
///
/// Used when performing operations that do not require mutation but may need to:
/// * Check membership,
/// * Compare two sets,
/// * Convert an ephemeral, zero-copy form into an owned set.
///
/// # What's a `ReadableKeySet`?
///
/// Let's say you have a database of creatures and their habitats:
///
/// ```rust
/// struct Creature {
///     id: u64,
///     species: String,
///     habitat: String,
/// }
/// ```
///
/// Now you want to look up all creatures who live in "Tide Pools."
///
/// Under the hood, the `ReadableKeySet` for `"Tide Pools"` might look like this:
///
/// ```text
/// "Tide Pools" → [5, 12, 98] // IDs of creatures found there
/// ```
///
/// These are primary keys (in serialized byte form) that point to full records like:
///
/// ```text
/// 5: Creature { id: 5, species: "Octopus", habitat: "Tide Pools" }
/// 12: Creature { id: 12, species: "Hermit Crab", habitat: "Tide Pools" }
/// 98: Creature { id: 98, species: "Starfish", habitat: "Tide Pools" }
/// ```
///
/// All of the methods below let you ask questions like:
/// * "Is the Hermit Crab in here?"
/// * "Are these habitats overlapping?"
/// * "Can I take ownership of this set for modification?"
pub trait ReadableKeySet {
    // +----------------------+
    // | Basic Set Operations |
    // +----------------------+

    /// Returns how many primary keys are in this index set.
    ///
    /// For example, `"Tide Pools"` might return `3` if there are three known creatures found there.
    #[must_use] fn len(&self) -> usize;

    /// Returns `true` if this index set is empty.
    ///
    /// For example, `"ISO Class 1 Cleanroom"` might return `true` because no creatures live in the
    /// habitat.
    #[must_use] fn is_empty(&self) -> bool;

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
    #[must_use] fn contains(&self, primary_key_bytes: &[u8]) -> bool;

    /// Returns `true` if this set is a subset of another.
    ///
    /// “Are all elements in `self` also in `other`?”
    #[must_use] fn is_subset(&self, other: &Self) -> bool;

    /// Returns `true` if this set is a superset of another.
    ///
    /// “Are all elements in `other` also in `self`?”
    #[must_use] fn is_superset(&self, other: &Self) -> bool;

    /// Returns `true` if this set and another intersect.
    ///
    /// “Do these sets share any elements?”
    #[must_use] fn intersects(&self, other: &Self) -> bool;
}