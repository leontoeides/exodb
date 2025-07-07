/// An error returned from the serialization layer.
///
/// This includes errors for out of memory, corrupted or malformed data, etc.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// An error was encountered while serializing data.
    #[error("serialization of data failed")]
    Serialize { #[from] #[source] source: crate::layers::serializers::SerializeError },

    /// An error was encountered while deserializing data.
    #[error("deserialization of data failed")]
    Deserialize { #[from] #[source] source: crate::layers::serializers::DeserializeError },
}