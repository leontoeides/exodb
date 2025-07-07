//! Safety for [Finn Bear](https://github.com/finnbear) and
//! [Cai Bear](https://github.com/caibear)'s [bitcode](https://crates.io/crates/bitcode) crate's
//! [serde](https://serde.rs/) implementation.

// -------------------------------------------------------------------------------------------------
//
/// Marker trait indicating that a type is known to be safe for use with
/// [bitcode](https://crates.io/crates/bitcode)'s `serde` feature.
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
/// the trait `SafeForBitcodeSerde` is not implemented for `MyType`
/// ```
///
/// You can resolve this in one of three ways:
///
/// 1. Validate compatibility: ensure that your type only uses `serde` features supported by
///    [`bitcode`](https://docs.rs/bitcode).
///
/// 2. Opt-in manually: after validating, implement the marker:
///
///    ```rust
///    # use atlatl::codecs::SafeForBitcodeSerde;
///    # struct MyType {}
///    unsafe impl SafeForBitcodeSerde for MyType {}
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
/// # use atlatl::codecs::SafeForBitcodeSerde;
/// # struct ThatThingYouUseEverywhere {}
/// unsafe impl SafeForBitcodeSerde for ThatThingYouUseEverywhere {}
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
/// * `SafeForRmpSerde`
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
    message = "`SafeForBitcodeSerde` is not implemented for `{Self}`",
    label = "This type must be manually marked as safe for use with the `bitcode` serde feature",
    note = "Add `unsafe impl SafeForBitcodeSerde for MyType {{}}` after validating compatibility"
)]
pub unsafe trait SafeForBitcodeSerde {}

/// Marker trait indicating that `bool` types are safe to be encoded by the `bitcode-serde` codec.
unsafe impl SafeForBitcodeSerde for bool {}

/// Marker trait indicating that `char` types are safe to be encoded by the `bitcode-serde` codec.
unsafe impl SafeForBitcodeSerde for char {}

/// Marker trait indicating that `f32` types are safe to be encoded by the `bitcode-serde` codec.
unsafe impl SafeForBitcodeSerde for f32 {}

/// Marker trait indicating that `f64` types are safe to be encoded by the `bitcode-serde` codec.
unsafe impl SafeForBitcodeSerde for f64 {}

/// Marker trait indicating that `i8` types are safe to be encoded by the `bitcode-serde` codec.
unsafe impl SafeForBitcodeSerde for i8 {}

/// Marker trait indicating that `i16` types are safe to be encoded by the `bitcode-serde` codec.
unsafe impl SafeForBitcodeSerde for i16 {}

/// Marker trait indicating that `i32` types are safe to be encoded by the `bitcode-serde` codec.
unsafe impl SafeForBitcodeSerde for i32 {}

/// Marker trait indicating that `i64` types are safe to be encoded by the `bitcode-serde` codec.
unsafe impl SafeForBitcodeSerde for i64 {}

/// Marker trait indicating that `i128` types are safe to be encoded by the `bitcode-serde` codec.
unsafe impl SafeForBitcodeSerde for i128 {}

/// Marker trait indicating that `isize` types are safe to be encoded by the `bitcode-serde` codec.
unsafe impl SafeForBitcodeSerde for isize {}

/// Marker trait indicating that `u8` types are safe to be encoded by the `bitcode-serde` codec.
unsafe impl SafeForBitcodeSerde for u8 {}

/// Marker trait indicating that `u16` types are safe to be encoded by the `bitcode-serde` codec.
unsafe impl SafeForBitcodeSerde for u16 {}

/// Marker trait indicating that `u32` types are safe to be encoded by the `bitcode-serde` codec.
unsafe impl SafeForBitcodeSerde for u32 {}

/// Marker trait indicating that `u64` types are safe to be encoded by the `bitcode-serde` codec.
unsafe impl SafeForBitcodeSerde for u64 {}

/// Marker trait indicating that `u128` types are safe to be encoded by the `bitcode-serde` codec.
unsafe impl SafeForBitcodeSerde for u128 {}

/// Marker trait indicating that `usize` types are safe to be encoded by the `bitcode-serde` codec.
unsafe impl SafeForBitcodeSerde for usize {}

/// Marker trait indicating that `()` types are safe to be encoded by the `bitcode-serde` codec.
unsafe impl SafeForBitcodeSerde for () {}

/// Marker trait indicating that `&str` types are safe to be encoded by the `bitcode-serde` codec.
unsafe impl SafeForBitcodeSerde for &str {}

/// Marker trait indicating that `String` types are safe to be encoded by the `bitcode-serde` codec.
unsafe impl SafeForBitcodeSerde for String {}

/// Marker trait indicating that `Vec` types are safe to be encoded by the `bitcode-serde` codec.
unsafe impl<T> SafeForBitcodeSerde for Vec<T> {}

/// Marker trait indicating that `IpAddr` types are safe to be encoded by the `bitcode-serde` codec.
unsafe impl SafeForBitcodeSerde for std::net::IpAddr {}

/// Marker trait indicating that `Ipv4Addr` types are safe to be encoded by the `bitcode-serde`
/// codec.
unsafe impl SafeForBitcodeSerde for std::net::Ipv4Addr {}

/// Marker trait indicating that `Ipv6Addr` types are safe to be encoded by the `bitcode-serde`
/// codec.
unsafe impl SafeForBitcodeSerde for std::net::Ipv6Addr {}

/// Marker trait indicating that `SocketAddr` types are safe to be encoded by the `bitcode-serde`
/// codec.
unsafe impl SafeForBitcodeSerde for std::net::SocketAddr {}

/// Marker trait indicating that `SocketAddrV4` types are safe to be encoded by the `bitcode-serde`
/// codec.
unsafe impl SafeForBitcodeSerde for std::net::SocketAddrV4 {}

/// Marker trait indicating that `SocketAddrV6` types are safe to be encoded by the `bitcode-serde`
/// codec.
unsafe impl SafeForBitcodeSerde for std::net::SocketAddrV6 {}

/// Marker trait indicating that `NonZeroI8` types are safe to be encoded by the `bitcode-serde`
/// codec.
unsafe impl SafeForBitcodeSerde for std::num::NonZeroI8 {}

/// Marker trait indicating that `NonZeroI16` types are safe to be encoded by the `bitcode-serde`
/// codec.
unsafe impl SafeForBitcodeSerde for std::num::NonZeroI16 {}

/// Marker trait indicating that `NonZeroI32` types are safe to be encoded by the `bitcode-serde`
/// codec.
unsafe impl SafeForBitcodeSerde for std::num::NonZeroI32 {}

/// Marker trait indicating that `NonZeroI64` types are safe to be encoded by the `bitcode-serde`
/// codec.
unsafe impl SafeForBitcodeSerde for std::num::NonZeroI64 {}

/// Marker trait indicating that `NonZeroI128` types are safe to be encoded by the `bitcode-serde`
/// codec.
unsafe impl SafeForBitcodeSerde for std::num::NonZeroI128 {}

/// Marker trait indicating that `NonZeroIsize` types are safe to be encoded by the `bitcode-serde`
/// codec.
unsafe impl SafeForBitcodeSerde for std::num::NonZeroIsize {}

/// Marker trait indicating that `NonZeroU8` types are safe to be encoded by the `bitcode-serde`
/// codec.
unsafe impl SafeForBitcodeSerde for std::num::NonZeroU8 {}

/// Marker trait indicating that `NonZeroU16` types are safe to be encoded by the `bitcode-serde`
/// codec.
unsafe impl SafeForBitcodeSerde for std::num::NonZeroU16 {}

/// Marker trait indicating that `NonZeroU32` types are safe to be encoded by the `bitcode-serde`
/// codec.
unsafe impl SafeForBitcodeSerde for std::num::NonZeroU32 {}

/// Marker trait indicating that `NonZeroU64` types are safe to be encoded by the `bitcode-serde`
/// codec.
unsafe impl SafeForBitcodeSerde for std::num::NonZeroU64 {}

/// Marker trait indicating that `NonZeroU128` types are safe to be encoded by the `bitcode-serde`
/// codec.
unsafe impl SafeForBitcodeSerde for std::num::NonZeroU128 {}

/// Marker trait indicating that `NonZeroUsize` types are safe to be encoded by the `bitcode-serde`
/// codec.
unsafe impl SafeForBitcodeSerde for std::num::NonZeroUsize {}

/// Marker trait indicating that `AtomicBool` types are safe to be encoded by the `bitcode-serde`
/// codec.
unsafe impl SafeForBitcodeSerde for std::sync::atomic::AtomicBool {}

/// Marker trait indicating that `AtomicI16` types are safe to be encoded by the `bitcode-serde`
/// codec.
unsafe impl SafeForBitcodeSerde for std::sync::atomic::AtomicI16 {}

/// Marker trait indicating that `AtomicI32` types are safe to be encoded by the `bitcode-serde`
/// codec.
unsafe impl SafeForBitcodeSerde for std::sync::atomic::AtomicI32 {}

/// Marker trait indicating that `AtomicI64` types are safe to be encoded by the `bitcode-serde`
/// codec.
unsafe impl SafeForBitcodeSerde for std::sync::atomic::AtomicI64 {}

/// Marker trait indicating that `AtomicI8` types are safe to be encoded by the `bitcode-serde`
/// codec.
unsafe impl SafeForBitcodeSerde for std::sync::atomic::AtomicI8 {}

/// Marker trait indicating that `AtomicIsize` types are safe to be encoded by the `bitcode-serde`
/// codec.
unsafe impl SafeForBitcodeSerde for std::sync::atomic::AtomicIsize {}

/// Marker trait indicating that `AtomicU8` types are safe to be encoded by the `bitcode-serde`
/// codec.
unsafe impl SafeForBitcodeSerde for std::sync::atomic::AtomicU8 {}

/// Marker trait indicating that `AtomicU16` types are safe to be encoded by the `bitcode-serde`
/// codec.
unsafe impl SafeForBitcodeSerde for std::sync::atomic::AtomicU16 {}

/// Marker trait indicating that `AtomicU32` types are safe to be encoded by the `bitcode-serde`
/// codec.
unsafe impl SafeForBitcodeSerde for std::sync::atomic::AtomicU32 {}

/// Marker trait indicating that `AtomicU64` types are safe to be encoded by the `bitcode-serde`
/// codec.
unsafe impl SafeForBitcodeSerde for std::sync::atomic::AtomicU64 {}

/// Marker trait indicating that `AtomicUsize` types are safe to be encoded by the `bitcode-serde`
/// codec.
unsafe impl SafeForBitcodeSerde for std::sync::atomic::AtomicUsize {}

/// Marker trait indicating that `Duration` types are safe to be encoded by the `bitcode-serde`
/// codec.
unsafe impl SafeForBitcodeSerde for std::time::Duration {}

/// Marker trait indicating that `BinaryHeap` types are safe to be encoded by the `bitcode-serde`
/// codec.
unsafe impl<T> SafeForBitcodeSerde for std::collections::BinaryHeap<T> {}

/// Marker trait indicating that `BTreeMap` types are safe to be encoded by the `bitcode-serde`
/// codec.
unsafe impl<K, V> SafeForBitcodeSerde for std::collections::BTreeMap<K, V> {}

/// Marker trait indicating that `BTreeSet` types are safe to be encoded by the `bitcode-serde`
/// codec.
unsafe impl<T> SafeForBitcodeSerde for std::collections::BTreeSet<T> {}

/// Marker trait indicating that `HashMap` types are safe to be encoded by the `bitcode-serde`
/// codec.
unsafe impl<K, V, S: ::std::hash::BuildHasher> SafeForBitcodeSerde for std::collections::HashMap<K, V, S> {}

/// Marker trait indicating that `HashSet` types are safe to be encoded by the `bitcode-serde`
/// codec.
unsafe impl<T, S: ::std::hash::BuildHasher> SafeForBitcodeSerde for std::collections::HashSet<T, S> {}

/// Marker trait indicating that `VecDeque` types are safe to be encoded by the `bitcode-serde`
/// codec.
unsafe impl<T> SafeForBitcodeSerde for std::collections::VecDeque<T> {}

/// Marker trait indicating that `PhantomData` types are safe to be encoded by the `bitcode-serde`
/// codec.
unsafe impl<T> SafeForBitcodeSerde for std::marker::PhantomData<T> {}

/// Marker trait indicating that `Rc` types are safe to be encoded by the `bitcode-serde` codec.
unsafe impl<T> SafeForBitcodeSerde for std::rc::Rc<T> {}

/// Marker trait indicating that `Arc` types are safe to be encoded by the `bitcode-serde` codec.
unsafe impl<T> SafeForBitcodeSerde for std::sync::Arc<T> {}