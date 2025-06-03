/// A trait for encoding and decoding types to and from their binary representation.
///
/// This abstraction allows different serialization formats (such as `bitcode-native`, `rmp-serde`)
/// to be used interchangeably across the database.
///
/// Implementors of this trait must define how a type is serialized into bytes and deserialized back
/// into its native form.
///
/// # Ordering
///
/// To ensure range queries are meaningful, types that preserve order when encoded should also
/// implement [`OrderedWhenEncoded`].
///
/// # Type Migrations
///
/// `exodb` supports type-safe storage via the [`Codec`] trait. If your data structures evolve over
/// time, you can use the [serde_flow](https://crates.io/crates/serde_flow) crate to migrate stored
/// values between versions at runtime.
///
/// ```toml
/// [dependencies]
/// serde_flow = "1.1"
/// ```
///
/// Learn more: <https://docs.rs/serde_flow>
#[diagnostic::on_unimplemented(
    message = "`Codec<T>` is not implemented for `{Self}`",
    label = "This type cannot currently be encoded or decoded",
    note = "To fix this, enable at least one codec feature in your `Cargo.toml` under the `exodb` crate:",
    note = "  Examples: `bitcode-native`, `musli-storage`, `rmp-serde`, etc.",
    note = "The selected codec may not support the type you're using. Make sure the necessary serialization traits are implemented.",
    note = "Many codecs require your type to derive `serde::Serialize` and `serde::Deserialize`, or an equivalent trait.",
    note = "If needed, implement `Codec` manually for full control, or wrap your type in a compatible struct."
)]
pub trait Codec<T> {
    /// Encodes the value into a binary byte vector suitable for storage.
    ///
    /// # Errors
    ///
    /// Consult the documentation of the codec backend you are using for more detail on
    /// serialization behavior and potential limitations.
    fn encode(&self) -> Result<Vec<u8>, crate::codecs::Error> where Self: Sized;

    /// Decodes a value from its binary representation.
    ///
    /// # Errors
    ///
    /// This method may fail for several reasons, including:
    ///
    /// * Input bytes are corrupted or malformed,
    /// * Codec encountered unsupported `serde` attributes or structures, or
    /// * Bytes do not represent a valid value of the expected type.
    ///
    /// Consult the documentation of the codec backend you are using for more detail on
    /// deserialization behavior and potential limitations.
    fn decode(bytes: &[u8]) -> Result<T, crate::codecs::Error> where Self: Sized;
}