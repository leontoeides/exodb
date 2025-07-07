use crate::layers::Compressible;
use crate::layers::Compressor;
use crate::layers::Correctable;
use crate::layers::Corrector;
use crate::layers::Descriptor;
use crate::layers::Direction;
use crate::layers::Encryptable;
use crate::layers::Encryptor;
use crate::layers::Layer;
use crate::layers::Serializable;
use crate::layers::Serializer;
use crate::layers::Value;
use crate::layers::value_buf::Error;
use crate::layers::value_buf::Metadata;
use std::borrow::Cow;

// -------------------------------------------------------------------------------------------------
//
/// Clone-on-write smart pointer for `redb` byte buffers.
///
/// Acts like `std::borrow::Cow`, managing borrowed or owned data with through a pipeline of
/// optional step.
///
/// For example, data may flow through stages like this:
///
/// ```text
/// [redb::Value (&[u8])]
///    ↓
/// [Error Correction (optional: Reed-Solomon, etc.)]
///    ↓
/// [Decompression (optional: zlib, zstd, bzip2, etc.)]
///    ↓
/// [Deserialization (bitcode-serde, rmp-serde, musli-storage, borsh, etc.)]
///    ↓
/// [Value]
/// ```
///
/// This structure and arrangement allows for efficient management of the data buffers, as well as
/// potentially zero-copy error-correction and deserialization.
///
/// # Lifetimes
///
/// * The `b` lifetime represents bytes potentially being borrowed from the `redb` database.
#[derive(Clone, Debug, Default)]
pub struct ValueBuf<'b> {
    metadata: Metadata,
    pub(crate) data: std::borrow::Cow<'b, [u8]>
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl<'b> ValueBuf<'b> {
    /// Returns the number of bytes in the buffer.
    #[inline]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns `true` if the buffer is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns an immutable slice of the bytes. Does not allocate.
    #[inline]
    pub fn as_slice(&'b self) -> &'b [u8] {
        self.data.as_ref()
    }

    /// Instantiates a `ValueBuf` from an owned `Vec<u8>` and marks the data as "recovered."
    #[inline]
    pub(crate) fn from_recovered_data(recovered_data: Vec<u8>) -> Self {
        ValueBuf {
            metadata: Metadata::default(),
            data: recovered_data.into()
        }
    }

    /// Reads a little-endian `u16` from the end of the value buffer, shortening the buffer by `2`
    /// bytes. This effectively pops a word from the end of the `ValueBuf`.
    #[inline]
    fn pop_u16_le(&'b mut self) -> Result<u16, Error> {
        const U16_SIZE: usize = std::mem::size_of::<u16>();
        let len = self.len();
        if len < U16_SIZE {
            Err(Error::EndOfBuffer {
                bytes_read: U16_SIZE,
                bytes_remaining: len
            })
        } else {
            let position = len - U16_SIZE;
            let bytes = [self.data[position], self.data[position + 1]];
            let word = u16::from_le_bytes(bytes);
            match &mut self.data {
                Cow::Borrowed(slice) => *slice = &slice[..position],
                Cow::Owned(array) => array.truncate(position),
            }
            Ok(word)
        }
    }

    /// Pops a descriptor from the end of the `ValueBuf` by reading a little-endian `u16` from the
    /// end of the value buffer, shortening the buffer by `2` bytes, and converts the word into a
    /// descriptor.
    #[inline]
    fn pop_layer_descriptor(&'b mut self) -> Result<Descriptor, Error> {
        let word = self.pop_u16_le()?;
        let descriptor = Descriptor::try_from(word)?;
        Ok(descriptor)
    }

    #[inline]
    fn deserialize<V: Serializer::<'b, V>>(
        self,
        descriptor: &Descriptor
    ) -> Result<Value<'b, V>, Error> {
        if descriptor.direction()?.is_read() {
            let layer_method = descriptor.serialization_method()?;
            if layer_method == V::method() {
                Ok(Value::Value(V::deserialize(self)?))
            } else {
                Err(Error::Other)
            }
        } else {
            Ok(self.into())
        }
    }

/*
    pub fn apply_layers_for_read<V>(
        self,
        key: Option<&[u8]>,
    ) -> Result<Self, Error>
    where
        V:
            Correctable + Corrector::<'b, V> +
            Encryptable + Encryptor::<'b, V> +
            Compressible + Compressor::<'b, V> +
            Serializable + Serializer::<'b, V>,
    {
        let descriptor = self.pop_layer_descriptor()?;
        let layer = descriptor.layer()?;
        let direction = descriptor.direction()?;

        let value_buf =
            if layer == &Layer::Correction && matches!(V::correction_direction(), Direction::ReadOnly | Direction::Both) {
                V::recover(self)?
            } else {
                self
            };

        let value_buf =
            if layer == &Layer::Encryption && matches!(V::encryption_direction(), Direction::ReadOnly | Direction::Both) {
                V::decrypt(self, key.unwrap())?
            } else {
                self
            };

        let value_buf =
            if layer == &Layer::Compression && matches!(V::compression_direction(), Direction::ReadOnly | Direction::Both) {
                V::decompress(self)?
            } else {
                self
            };

        let value_buf =
            if layer == &Layer::Serialization && matches!(V::serialization_direction(), Direction::ReadOnly | Direction::Both) {
                V::deserialize(self)?
            } else {
                self
            };

        Ok(value_buf)
    } */
}

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl<'b> AsRef<[u8]> for ValueBuf<'b> {
    /// Returns a reference to the bytes in the buffer. Does not allocate.
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.data.as_ref()
    }
}

impl<'b> std::ops::Deref for ValueBuf<'b> {
    type Target = [u8];

    /// Returns a reference to the bytes in the buffer. Does not allocate.
    #[inline]
    fn deref(&self) -> &Self::Target {
        self.data.deref()
    }
}

impl<'b> Eq for ValueBuf<'b> {}

impl<'b> PartialEq for ValueBuf<'b> {
    /// Compares two `ValueBuf`s for equality based on their raw bytes.
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.data.as_ref() == other.data.as_ref()
    }
}

impl<'b> From<&'b [u8]> for ValueBuf<'b> {
    /// Wraps a borrowed `&[u8]` slice into a `ValueBuf` type.
    ///
    /// This allows us to access a borrowed `&[u8]` slice in the same way an owned `Vec<u8>` can be.
    #[inline]
    fn from(borrowed_slice_of_bytes: &'b [u8]) -> ValueBuf<'b> {
        ValueBuf {
            metadata: Metadata::default(),
            data: borrowed_slice_of_bytes.into()
        }
    }
}

impl<'b> From<&'b mut [u8]> for ValueBuf<'b> {
    /// Wraps a borrowed `&mut [u8]` slice into a `ValueBuf` type.
    ///
    /// This allows us to access a borrowed `&[u8]` slice in the same way an owned `Vec<u8>` can be.
    #[inline]
    fn from(borrowed_slice_of_bytes: &'b mut [u8]) -> ValueBuf<'b> {
        ValueBuf {
            metadata: Metadata::default(),
            data: (&*borrowed_slice_of_bytes).into()
        }
    }
}

impl<'b> From<&'b str> for ValueBuf<'b> {
    /// Wraps a borrowed `&str` string into a `ValueBuf` type.
    ///
    /// This allows us to access a borrowed `&str` slice in the same way an owned `Vec<u8>` can be.
    #[inline]
    fn from(string: &'b str) -> ValueBuf<'b> {
        ValueBuf {
            metadata: Metadata::default(),
            data: std::borrow::Cow::Borrowed(string.as_bytes())
        }
    }
}

impl<'b> From<&'b Vec<u8>> for ValueBuf<'b> {
    /// Wraps a borrowed `&Vec<u8>` vector collection into a `ValueBuf` type.
    ///
    /// This allows us to access a borrowed `&Vec<u8>` in the same way an owned `Vec<u8>` can be.
    #[inline]
    fn from(borrowed_vec_of_bytes: &'b Vec<u8>) -> ValueBuf<'b> {
        ValueBuf {
            metadata: Metadata::default(),
            data: borrowed_vec_of_bytes.into()
        }
    }
}

impl<'b> From<Vec<u8>> for ValueBuf<'b> {
    /// Wraps an owned `Vec<u8>` into a `ValueBuf` type.
    ///
    /// This allows us to access an owned `Vec<u8>` in the same way a borrowed `&[u8]` slice of
    /// bytes can be.
    #[inline]
    fn from(owned_vec_of_bytes: Vec<u8>) -> ValueBuf<'b> {
        ValueBuf {
            metadata: Metadata::default(),
            data: owned_vec_of_bytes.into()
        }
    }
}