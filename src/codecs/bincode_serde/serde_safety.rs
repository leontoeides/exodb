//! Safety for [Finn Bear](https://github.com/finnbear) and
//! [Cai Bear](https://github.com/caibear)'s [bincode](https://crates.io/crates/bincode) crate's
//! [serde](https://serde.rs/) implementation.

// -------------------------------------------------------------------------------------------------
//
/// Marker trait indicating that a type is known to be safe for use with
/// [bincode](https://crates.io/crates/bincode)'s `serde` feature.
///
/// This trait exists to protect against silent data corruption when using `serde` features that are
/// not always supported. For example, `#[serde(flatten)]`, `#[serde(tag)]`, complex enum variants,
/// or conditional skips.
///
/// You must manually implement this trait on your types when the `serde-safety` feature is turned
/// on. This feature is enabled by default.
///
/// ## Compiler Errors
///
/// If you see an error like:
///
/// ```text
/// the trait `SafeForBincodeSerde` is not implemented for `MyType`
/// ```
///
/// You can resolve this in one of three ways:
///
/// 1. Validate compatibility: ensure that your type only uses `serde` features supported by
///    [`bincode`](https://docs.rs/bincode).
///
/// 2. Opt-in manually: after validating, implement the marker:
///
///    ```rust
///    # use exodb::codecs::SafeForBincodeSerde;
///    # struct MyType {}
///    unsafe impl SafeForBincodeSerde for MyType {}
///    ```
///
/// 3. Bypass the check (not recommended unless you're sure):
///     * Disable the `serde-safety` feature in your `Cargo.toml`, or
///     * Enable the override feature:
///       ```toml
///       i-know-what-im-doing = []
///       ```
///
/// ## Submissions
///
/// Hello, weary travellers of the serialization wasteland.
///
/// You've wandered long through the valley of `#[serde(flatten)]`, fled the eldritch horrors of
/// tagged enums, and survived the cryptic warnings of missing trait bounds.
///
/// If you've safely implemented:
///
/// ```rust
/// # use exodb::codecs::SafeForBincodeSerde;
/// # struct ThatThingYouUseEverywhere {}
/// unsafe impl SafeForBincodeSerde for ThatThingYouUseEverywhere {}
/// ```
///
/// ...or if you've validated that a common type works great with `rmp-serde`, or `bincode-serde`...
///
/// We welcome your contributions!
///
/// Even small unsafe impls for common types (from crates like `chrono`, `uuid`, `url`, `smol_str`,
/// etc.) help everyone avoid silent corruption and `#[derive(GoodLuck)]` debugging.
///
/// To contribute:
///
/// Submit a PR adding your safe impl to:
///
/// * `SafeForRmpSerde`
/// * `SafeForPostcardSerde`
/// * `SafeForBincodeSerde`
///
/// Mention any unsupported serde features your type avoids.
///
/// Bonus points for links to upstream issues or test coverage.
///
/// Let's chart the untyped marshes together, and leave the map clearer than we found it.
///
/// The `exodb` Team
///
/// # Safety
///
/// Some serializers do not produce errors when given unsupported attributes. They silently skip
/// or misinterpret fields.
///
/// This trait enforces a clear boundary between safe and unsafe types to prevent accidental
/// data loss or misbehavior.
#[cfg_attr(docsrs, doc(cfg(feature = "serde-safety")))]
#[diagnostic::on_unimplemented(
    message = "`SafeForBincodeSerde` is not implemented for `{Self}`",
    label = "This type must be manually marked as safe for use with the `bincode` serde feature",
    note = "Add `unsafe impl SafeForBincodeSerde for MyType {{}}` after validating compatibility"
)]
pub unsafe trait SafeForBincodeSerde {}

/// Marker trait indicating that `bool` types are safe to be encoded by the `bincode-serde` codec.
unsafe impl SafeForBincodeSerde for bool {}

/// Marker trait indicating that `char` types are safe to be encoded by the `bincode-serde` codec.
unsafe impl SafeForBincodeSerde for char {}

/// Marker trait indicating that `f32` types are safe to be encoded by the `bincode-serde` codec.
unsafe impl SafeForBincodeSerde for f32 {}

/// Marker trait indicating that `f64` types are safe to be encoded by the `bincode-serde` codec.
unsafe impl SafeForBincodeSerde for f64 {}

/// Marker trait indicating that `i8` types are safe to be encoded by the `bincode-serde` codec.
unsafe impl SafeForBincodeSerde for i8 {}

/// Marker trait indicating that `i16` types are safe to be encoded by the `bincode-serde` codec.
unsafe impl SafeForBincodeSerde for i16 {}

/// Marker trait indicating that `i32` types are safe to be encoded by the `bincode-serde` codec.
unsafe impl SafeForBincodeSerde for i32 {}

/// Marker trait indicating that `i64` types are safe to be encoded by the `bincode-serde` codec.
unsafe impl SafeForBincodeSerde for i64 {}

/// Marker trait indicating that `i128` types are safe to be encoded by the `bincode-serde` codec.
unsafe impl SafeForBincodeSerde for i128 {}

/// Marker trait indicating that `isize` types are safe to be encoded by the `bincode-serde` codec.
unsafe impl SafeForBincodeSerde for isize {}

/// Marker trait indicating that `u8` types are safe to be encoded by the `bincode-serde` codec.
unsafe impl SafeForBincodeSerde for u8 {}

/// Marker trait indicating that `u16` types are safe to be encoded by the `bincode-serde` codec.
unsafe impl SafeForBincodeSerde for u16 {}

/// Marker trait indicating that `u32` types are safe to be encoded by the `bincode-serde` codec.
unsafe impl SafeForBincodeSerde for u32 {}

/// Marker trait indicating that `u64` types are safe to be encoded by the `bincode-serde` codec.
unsafe impl SafeForBincodeSerde for u64 {}

/// Marker trait indicating that `u128` types are safe to be encoded by the `bincode-serde` codec.
unsafe impl SafeForBincodeSerde for u128 {}

/// Marker trait indicating that `usize` types are safe to be encoded by the `bincode-serde` codec.
unsafe impl SafeForBincodeSerde for usize {}

/// Marker trait indicating that `String` types are safe to be encoded by the `bincode-serde` codec.
unsafe impl SafeForBincodeSerde for String {}

/// Marker trait indicating that `Vec` types are safe to be encoded by the `bincode-serde` codec.
unsafe impl<T> SafeForBincodeSerde for Vec<T> {}