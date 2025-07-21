//! The `Value` type contains Rust-native type (i.e. a value in deserialized form), that may either
//! be borrowed or owned.

// -------------------------------------------------------------------------------------------------
//
/// Contains Rust-native type (i.e. a value in deserialized form), that may either be borrowed or
/// owned.
///
/// This type helps avoid unnecessary clones when working with borrowed data from the host
/// application or from storage.
///
/// This `Value` type is very similar to `std::borrow::Cow` but it does not require the `Clone`
/// trait to be implemented for `V`.
///
/// # Generics & Lifetimes
///
/// * `V` generic represents the user's value type, for example: `User`, `Customer`, `String`, etc.
/// * `b` lifetime represents bytes potentially being borrowed from storage or from the host
///   application.
#[derive(Debug)]
pub enum Value<'b, V> {
    /// A reference to the value.
    ///
    /// When reading from storage, this variant represents a successfully deserialized value. When
    /// writing, this represents the initial Rust-native typed value when preparing to write.
    ///
    /// This `Borrowed` variant avoids cloning when the value lifetime allows it.
    Borrowed(&'b V),

    /// An owned value.
    ///
    /// When reading from storage, this variant represents a successfully deserialized value. When
    /// writing, this represents the initial Rust-native typed value when preparing to write.
    Owned(V),
}

// -------------------------------------------------------------------------------------------------

impl<'b, V> Value<'b, V> {
    /// Returns `true` if the value is borrowed.
    pub const fn is_borrowed(&self) -> bool {
        matches!(self, Self::Borrowed(_))
    }

    /// Returns `true` if the value is owned.
    pub const fn is_owned(&self) -> bool {
        matches!(self, Self::Owned(_))
    }

    // Conversions: To

    /// Unwraps a `Value` type, returning the contained `V` type. If the contained value is not
    /// owned, a `None` will be returned.
    pub fn into_owned(self) -> Option<V> {
        self.into()
    }

    // Conversions: From

    /// Instantiates a `Value` type from an owned `V` Rust-native typed value.
    pub fn from_value(value: V) -> Self {
        value.into()
    }

    /// Instantiates a `Value` type from a borrowed `&V` Rust-native typed value.
    pub fn from_value_ref(value: &'b V) -> Self {
        value.into()
    }
}

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

// Conversions

impl<V> std::convert::AsRef<V> for Value<'_, V> {
    /// Returns a reference to the value.
    fn as_ref(&self) -> &V {
        match self {
            Self::Borrowed(borrowed_value) => borrowed_value,
            Self::Owned(owned_value) => owned_value,
        }
    }
}

impl<'b, V> std::convert::From<&'b V> for Value<'b, V> {
    /// Converts a borrowed `&V` value into a `Value` type.
    fn from(borrowed_value: &'b V) -> Self {
        Value::Borrowed(borrowed_value)
    }
}

impl<V> std::convert::From<V> for Value<'_, V> {
    /// Converts an owned `V` value into a `Value` type.
    fn from(owned_value: V) -> Self {
        Value::Owned(owned_value)
    }
}

impl<'b, V> std::convert::From<Value<'b, V>> for Option<V> {
    /// Converts an owned `Value` type into an `Option<V>` type.
    ///
    /// This effectively unwraps a `Value` type, returning the contained `V` type. If the contained
    /// value is not owned, a `None` will be returned.
    fn from(value: Value<'b, V>) -> Self {
        match value {
            Value::Borrowed(_borrowed_value) => None,
            Value::Owned(owned_value) => Some(owned_value)
        }
    }
}