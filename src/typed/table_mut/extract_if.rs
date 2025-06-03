//! An iterator that removes entries from a table where a predicate applied to decoded keys and
//! values returns `true`.

use std::marker::PhantomData;
use crate::codecs::Codec;

// -------------------------------------------------------------------------------------------------

/// A type alias for the underlying redb iterator used by extract-if operations.
///
/// This iterator yields key-value pairs where the user-defined predicate evaluates to `true`.
type RedbExtractIf<'e> = redb::ExtractIf<
    'e,                                         // Lifetime
    &'static [u8],                              // Key
    &'static [u8],                              // Value
    Box<dyn FnMut(&[u8], &[u8]) -> bool + 'e>   // Function
>;

/// An iterator that removes entries from a table where a predicate applied to decoded keys and
/// values returns `true`.
///
/// This is a typed wrapper over redb's [`ExtractIf`] that decodes key-value pairs using [`Codec`].
pub struct ExtractIf<'e, K, V, F>
where
    F: for<'f> FnMut(&K, &V) -> bool,
{
    inner: RedbExtractIf<'e>,
    _phantom: std::marker::PhantomData<(K, V, F)>,
}

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl<K, V, F> Iterator for ExtractIf<'_, K, V, F>
where
    K: Codec<K>,
    V: Codec<V>,
    F: for<'f> FnMut(&K, &V) -> bool
{
    type Item = Result<(K, V), crate::Error>;

    /// Advances the iterator and returns the next key-value pair where the predicate matches.
    ///
    /// If a matching entry is returned, it is also removed from the table.
    ///
    /// # Errors
    ///
    /// * Returns an error if decoding the key or value fails.
    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .next()
            .map(|entry| entry
                .map_err(Into::into)
                .and_then(|(k, v)| Ok((
                    K::decode(k.value())?,
                    V::decode(v.value())?,
                )))
            )
    }
}

impl<'e, K, V, F> From<RedbExtractIf<'e>> for ExtractIf<'e, K, V, F>
where
    K: Codec<K>,
    V: Codec<V>,
    F: for<'f> FnMut(&K, &V) -> bool
{
    /// Converts a redb-based extract-if iterator into a typed [`ExtractIf`] with decoding logic.
    ///
    /// This allows `ExtractIf` to be used seamlessly with redb’s native iterator interface.
    fn from(inner: RedbExtractIf<'e>) -> Self {
        Self { inner, _phantom: PhantomData }
    }
}