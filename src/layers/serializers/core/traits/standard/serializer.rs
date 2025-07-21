//! The `StandardSerializer` trait provides an interface for standard serialization schemes.

use crate::layers::core::Bytes;
use crate::layers::serializers::SerializeError;

// -------------------------------------------------------------------------------------------------
//
/// The `StandardSerializer` trait converts structured data types into a binary format suitable for
/// storage, transmission, or further processing through the pipeline stages.
///
/// Serializes data from owned values, typically producing owned output.
///
/// This trait consumes the input value and produces allocated serialized data, making it suitable
/// for traditional serialization workflows where data transformation is expected. Contrast with
/// `StandardSerializer`, which works with references and can avoid allocation through zero-copy
/// techniques.
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
#[diagnostic::on_unimplemented(
    message = "`Serializer<T>` is not implemented for `{Self}`",
    label = "This type cannot currently be serialized or deserialized",
    note = "To fix this, enable at least one serializer feature in your `Cargo.toml` under the `atlatl` crate:",
    note = "  Examples: `bitcode-native`, `musli-storage`, `rmp-serde`, etc.",
    note = "The selected serializer may not support the type you're using. Make sure the necessary serialization traits are implemented.",
    note = "Many serializers require your type to derive `serde::Serialize` and `serde::Deserialize`, or an equivalent trait.",
    note = "If needed, implement `Serializer` manually for full control, or wrap your type in a compatible struct."
)]
pub trait StandardSerializer<'b, T> {
    /// Serializes an owned value into its binary representation.
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
    /// * `b` lifetime represents bytes potentially being borrowed from the host application.
    fn serialize(
        self
    ) -> Result<Bytes<'b>, SerializeError>;

    /// Serializes a borrowed value into its binary representation.
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
    /// * `b` lifetime represents bytes potentially being borrowed from the host application.
    fn serialize_ref(
        &'b self
    ) -> Result<Bytes<'b>, SerializeError>;

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
        serialized_bytes: Bytes<'b>
    ) -> Result<crate::layers::core::Value<'b, T>, crate::layers::serializers::DeserializeError>;

    /// Returns the serialization method that the current `Serializer` trait implements.
    ///
    /// This enables runtime identification of the serialization method in use, allowing
    /// applications to log serialization details, or store metadata about how data was processed in
    /// the data pipeline.
    fn method() -> &'static crate::layers::serializers::Method;
}