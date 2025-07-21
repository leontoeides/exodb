//! Safety for [Kornel Lesiński](https://github.com/kornelski) and
//! [Evgeny Safronov](https://github.com/3Hren)'s
//! [rmp-serde](https://crates.io/crates/rmp-serde) crate.

// -------------------------------------------------------------------------------------------------
//
/// Marker trait indicating that a type is known to be safe for use with
/// [rmp-serde](https://crates.io/crates/rmp-serde).
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
/// the trait `SafeForMessagePack` is not implemented for `MyType`
/// ```
///
/// You can resolve this in one of three ways:
///
/// 1. Validate compatibility: ensure that your type only uses `serde` features supported by
///    [`rmp-serde`](https://docs.rs/rmp-serde).
///
/// 2. Opt-in manually: after validating, implement the marker:
///
///    ```rust
///    # use atlatl::layers::serializers::impls::rmp_serde::SafeForMessagePack;
///    # struct MyType {}
///    unsafe impl SafeForMessagePack for MyType {}
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
/// # use atlatl::layers::serializers::impls::rmp_serde::SafeForMessagePack;
/// # struct ThatThingYouUseEverywhere {}
/// unsafe impl SafeForMessagePack for ThatThingYouUseEverywhere {}
/// ```
///
/// ...or if you've validated that a common type works great with `rmp-serde`, or `bitcode-serde`...
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
/// * `SafeForMessagePack`
/// * `SafeForPostcardSerde`
/// * `SafeForBitcodeSerde`
///
/// Mention any unsupported serde features your type avoids.
///
/// Bonus points for links to upstream issues or test coverage.
///
/// Let's chart the untyped marshes together, and leave the map clearer than we found it.
///
/// The `atlatl` Team
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
    message = "`SafeForMessagePack` is not implemented for `{Self}`",
    label = "This type must be manually marked as safe for use with `rmp-serde`",
    note = "Add `unsafe impl SafeForMessagePack for MyType {{}}` after validating compatibility"
)]
pub unsafe trait SafeForMessagePack {}

/// Marker trait indicating that `bool` values are safe to be serialized by `rmp-serde`.
unsafe impl SafeForMessagePack for bool {}

/// Marker trait indicating that `char` values are safe to be serialized by `rmp-serde`.
unsafe impl SafeForMessagePack for char {}

/// Marker trait indicating that `f32` values are safe to be serialized by `rmp-serde`.
unsafe impl SafeForMessagePack for f32 {}

/// Marker trait indicating that `f64` values are safe to be serialized by `rmp-serde`.
unsafe impl SafeForMessagePack for f64 {}

/// Marker trait indicating that `i8` values are safe to be serialized by `rmp-serde`.
unsafe impl SafeForMessagePack for i8 {}

/// Marker trait indicating that `i16` values are safe to be serialized by `rmp-serde`.
unsafe impl SafeForMessagePack for i16 {}

/// Marker trait indicating that `i32` values are safe to be serialized by `rmp-serde`.
unsafe impl SafeForMessagePack for i32 {}

/// Marker trait indicating that `i64` values are safe to be serialized by `rmp-serde`.
unsafe impl SafeForMessagePack for i64 {}

/// Marker trait indicating that `i128` values are safe to be serialized by `rmp-serde`.
unsafe impl SafeForMessagePack for i128 {}

/// Marker trait indicating that `isize` values are safe to be serialized by `rmp-serde`.
unsafe impl SafeForMessagePack for isize {}

/// Marker trait indicating that `u8` values are safe to be serialized by `rmp-serde`.
unsafe impl SafeForMessagePack for u8 {}

/// Marker trait indicating that `u16` values are safe to be serialized by `rmp-serde`.
unsafe impl SafeForMessagePack for u16 {}

/// Marker trait indicating that `u32` values are safe to be serialized by `rmp-serde`.
unsafe impl SafeForMessagePack for u32 {}

/// Marker trait indicating that `u64` values are safe to be serialized by `rmp-serde`.
unsafe impl SafeForMessagePack for u64 {}

/// Marker trait indicating that `u128` values are safe to be serialized by `rmp-serde`.
unsafe impl SafeForMessagePack for u128 {}

/// Marker trait indicating that `usize` values are safe to be serialized by `rmp-serde`.
unsafe impl SafeForMessagePack for usize {}

/// Marker trait indicating that `&str` values are safe to be serialized by `rmp-serde`.
unsafe impl SafeForMessagePack for &str {}

/// Marker trait indicating that `String` values are safe to be serialized by `rmp-serde`.
unsafe impl SafeForMessagePack for String {}

/// Marker trait indicating that `Vec` values are safe to be serialized by `rmp-serde`.
unsafe impl<T> SafeForMessagePack for Vec<T> {}

/// Marker trait indicating that `NonZeroI8` values are safe to be serialized by `rmp-serde`.
unsafe impl SafeForMessagePack for std::num::NonZeroI8 {}

/// Marker trait indicating that `NonZeroI16` values are safe to be serialized by `rmp-serde`.
unsafe impl SafeForMessagePack for std::num::NonZeroI16 {}

/// Marker trait indicating that `NonZeroI32` values are safe to be serialized by `rmp-serde`.
unsafe impl SafeForMessagePack for std::num::NonZeroI32 {}

/// Marker trait indicating that `NonZeroI64` values are safe to be serialized by `rmp-serde`.
unsafe impl SafeForMessagePack for std::num::NonZeroI64 {}

/// Marker trait indicating that `NonZeroI128` values are safe to be serialized by `rmp-serde`.
unsafe impl SafeForMessagePack for std::num::NonZeroI128 {}

/// Marker trait indicating that `NonZeroIsize` values are safe to be serialized by `rmp-serde`.
unsafe impl SafeForMessagePack for std::num::NonZeroIsize {}

/// Marker trait indicating that `NonZeroU8` values are safe to be serialized by `rmp-serde`.
unsafe impl SafeForMessagePack for std::num::NonZeroU8 {}

/// Marker trait indicating that `NonZeroU16` values are safe to be serialized by `rmp-serde`.
unsafe impl SafeForMessagePack for std::num::NonZeroU16 {}

/// Marker trait indicating that `NonZeroU32` values are safe to be serialized by `rmp-serde`.
unsafe impl SafeForMessagePack for std::num::NonZeroU32 {}

/// Marker trait indicating that `NonZeroU64` values are safe to be serialized by `rmp-serde`.
unsafe impl SafeForMessagePack for std::num::NonZeroU64 {}

/// Marker trait indicating that `NonZeroU128` values are safe to be serialized by `rmp-serde`.
unsafe impl SafeForMessagePack for std::num::NonZeroU128 {}

/// Marker trait indicating that `NonZeroUsize` values are safe to be serialized by `rmp-serde`.
unsafe impl SafeForMessagePack for std::num::NonZeroUsize {}