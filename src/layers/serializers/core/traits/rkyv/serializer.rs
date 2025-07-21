//! The `SerializerRkyv` trait provides an interface specifically for the
//! [rkyv](https://crates.io/crates/rkyv) crate.

// -------------------------------------------------------------------------------------------------
//
/// The `SerializerRkyv` trait converts structured data types into a binary format suitable for
/// storage, transmission, or further processing through the pipeline stages.
///
/// Serializes data from borrowed references, enabling zero-copy operations.
///
/// This trait operates on borrowed data and can produce output that references the original input,
/// making it ideal for high-performance scenarios where memory allocation should be minimized.
/// Contrast with `Serializer`, which requires taking ownership and typically produces
/// allocated output.
///
/// # Interface Pattern
///
/// Serialize: `&T` → `&[u8]` (borrowed in, borrowed out)
/// Deserialize: `&[u8]` → `&T` (borrowed in, borrowed out)
///
/// # Ordering
///
/// To ensure range queries are meaningful, types that preserve order when serialized should also
/// implement [`OrderedWhenSerialized`].
///
/// # Type Migrations
///
/// If you expect your data structures change and evolve over time, you can use the
/// [serde_flow](https://crates.io/crates/serde_flow) crate to migrate stored values between
/// versions at runtime.
///
/// This will only work for serializers that rely on `serde`.
///
/// ```toml
/// [dependencies]
/// serde_flow = "1.1"
/// ```
///
/// Learn more: <https://docs.rs/serde_flow>
///
/// # Generics & Lifetimes
///
/// * `V` generic represents the user's value type, for example: `User`, `String`, etc.
/// * `b` lifetime represents bytes potentially being borrowed from the host application or the
///   `redb` database.
///
/// # Key Concepts
///
/// ## Pipeline Integration
///
/// This trait operates within a larger data processing pipeline where serialization is just one
/// optional stage. The zero-copy design allows for efficient processing of large datasets while
/// maintaining security properties throughout the pipeline.
///
/// ## `ValueBuf` and Zero-Copy Design
///
/// The `ValueBuf<'b>` wrapper allows the serialization layer to work with borrowed data from the
/// database without unnecessary copying, improving performance for large datasets.
#[diagnostic::on_unimplemented(
    message = "`SerializerRkyv<T>` is not implemented for `{Self}`",
    label = "This type cannot currently be serialized or deserialized",
    note = "To fix this, enable at least one serializer feature in your `Cargo.toml` under the `atlatl` crate:",
    note = "  Examples: `bitcode-native`, `musli-storage`, `rmp-serde`, etc.",
    note = "The selected serializer may not support the type you're using. Make sure the necessary serialization traits are implemented.",
    note = "Many serializers require your type to derive `serde::Serialize` and `serde::Deserialize`, or an equivalent trait.",
    note = "If needed, implement `SerializerRkyv` manually for full control, or wrap your type in a compatible struct."
)]
pub trait SerializerRkyv<'b, T> {
    /// Serializes the value into a binary byte vector suitable for storage.
    ///
    /// # Arguments
    ///
    /// * `&self` · A reference to the data structure to be converted into binary format.
    ///
    /// # Errors
    ///
    /// Consult the documentation of the serializer backend you are using for more detail on
    /// serialization behavior and potential limitations.
    ///
    /// # Generics & Lifetimes
    ///
    /// * `V` generic represents the user's value type, for example: `User`, `String`, etc.
    /// * `b` lifetime represents bytes potentially being borrowed from the `redb` database.
    fn serialize(
        &self
    ) -> Result<Vec<u8>, crate::layers::serializers::SerializeError> where Self: Sized;

    /// Deserializes a value from its binary representation.
    ///
    /// # Arguments
    ///
    /// * `serialized_bytes` · The binary representation of data to be converted back into the
    ///   original structured type `T`.
    ///
    /// # Errors
    ///
    /// This method may fail for several reasons, including:
    ///
    /// * Input bytes are corrupted or malformed,
    /// * Deserializer encountered unsupported `serde` attributes or structures, or
    /// * Bytes do not represent a valid value of the expected type.
    ///
    /// Consult the documentation of the serializer backend you are using for more detail on
    /// deserialization behavior and potential limitations.
    ///
    /// # Generics & Lifetimes
    ///
    /// * `V` generic represents the user's value type, for example: `User`, `String`, etc.
    /// * `b` lifetime represents bytes potentially being borrowed from the `redb` database.
    fn deserialize(
        serialized_bytes: &'b [u8]
    ) -> Result<&'b T, crate::layers::serializers::DeserializeError> where Self: Sized;

    /// Returns the serialization method that the current `SerializerRkyv` trait implements.
    ///
    /// This enables runtime identification of the serialization method in use, allowing
    /// applications to log serialization details, or store metadata about how data was processed in
    /// the data pipeline.
    fn method() -> &'static crate::layers::serializers::Method;
}