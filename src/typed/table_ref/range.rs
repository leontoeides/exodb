use crate::{codecs::Codec, typed::{RedbRange, ResultEntry}};
use std::marker::PhantomData;

// -------------------------------------------------------------------------------------------------
//
/// A double-ended iterator over a range of decoded key-value pairs in a table.
///
/// This wrapper decodes raw byte slices from a redb [`Range`] using [`crate::Codec`]
/// implementations for key and value types.
#[derive(Clone)]
pub struct Range<'r, K, V> {
    inner: RedbRange<'r>,
    _phantom: PhantomData<(K, V)>,
}

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl<K, V> Iterator for Range<'_, K, V>
where
    K: Codec<K>,
    V: Codec<V>
{
    type Item = ResultEntry<K, V>;

    /// Advances the iterator and returns the next key-value pair in ascending key order.
    ///
    /// # Errors
    ///
    /// * Returns an error if decoding the key or value fails.
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|entry| entry
            .map_err(Into::into)
            .and_then(|(k_guard, v_guard)| {
                let key   = K::deserialize(k_guard.value())?;
                let value = V::deserialize(v_guard.value())?;
                Ok((key, value))
            })
        )
    }
}

impl<K, V> DoubleEndedIterator for Range<'_, K, V>
where
    K: Codec<K>,
    V: Codec<V>
{
    /// Advances the iterator from the end and returns the next key-value pair in descending key
    /// order.
    ///
    /// # Errors
    ///
    /// * Returns an error if decoding the key or value fails.
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|entry| entry
            .map_err(Into::into)
            .and_then(|(k_guard, v_guard)| {
                let key   = K::deserialize(k_guard.value())?;
                let value = V::deserialize(v_guard.value())?;
                Ok((key, value))
            })
        )
    }
}

impl<'r, K, V> From<RedbRange<'r>> for Range<'r, K, V>
where
    K: Codec<K>,
    V: Codec<V>
{
    /// Converts a raw `redb` range iterator into a typed [`Range`] with decoding support.
    ///
    /// This allows decoding key-value pairs using the [`Codec`] trait for ergonomic access to typed
    /// data.
    fn from(range: RedbRange<'r>) -> Self {
        Self {
            inner: range,
            _phantom: PhantomData::<(K, V)>,
        }
    }
}