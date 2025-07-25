//! Safety for [James Munns](https://github.com/jamesmunns)'
//! [postcard](https://crates.io/crates/postcard) crate.

// -------------------------------------------------------------------------------------------------
//
/// Marker trait indicating that a type is known to be safe for use with
/// [postcard](https://crates.io/crates/postcard)'s `serde` feature.
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
/// the trait `SafeForPostcardSerde` is not implemented for `MyType`
/// ```
///
/// You can resolve this in one of three ways:
///
/// 1. Validate compatibility: ensure that your type only uses `serde` features supported by
///    [`postcard`](https://docs.rs/postcard).
///
/// 2. Opt-in manually: after validating, implement the marker:
///
///    ```rust
///    # use atlatl::codecs::SafeForPostcardSerde;
///    # struct MyType {}
///    unsafe impl SafeForPostcardSerde for MyType {}
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
/// # use atlatl::codecs::SafeForPostcardSerde;
/// # struct ThatThingYouUseEverywhere {}
/// unsafe impl SafeForPostcardSerde for ThatThingYouUseEverywhere {}
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
/// * `SafeForBincodeSerde`
/// * `SafeForMessagePack`
/// * `SafeForPostcardSerde`
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
    message = "`SafeForPostcardSerde` is not implemented for `{Self}`",
    label = "This type must be manually marked as safe for use with the `postcard` serde feature",
    note = "Add `unsafe impl SafeForPostcardSerde for MyType {{}}` after validating compatibility"
)]
pub unsafe trait SafeForPostcardSerde {}

/// Marker trait indicating that `u8` types are safe to be encoded by the `rmp-serde` codec.
unsafe impl SafeForPostcardSerde for i16 {}
unsafe impl SafeForPostcardSerde for i32 {}
unsafe impl SafeForPostcardSerde for i64 {}
unsafe impl SafeForPostcardSerde for i8 {}
unsafe impl SafeForPostcardSerde for u16 {}
unsafe impl SafeForPostcardSerde for u32 {}
unsafe impl SafeForPostcardSerde for u64 {}
unsafe impl SafeForPostcardSerde for u8 {}
unsafe impl SafeForPostcardSerde for String {}