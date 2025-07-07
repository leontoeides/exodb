
use crate::typed::TableRef;
pub use crate::typed::table_ref::ordered_table::OrderedTable;

use crate::{Codec, Error, typed::table_ref::range::Range};
use redb::{ReadableTableMetadata, TableHandle};

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl<K, V> TableRef<K, V>
where
    K: Codec<K>,
    V: Codec<V>
{

    pub(crate) fn get_by_key_bytes(
        &self,
        key_bytes: &[u8],
    ) -> Result<V, Error> {
        self.redb_table
            .get(key_bytes)
            .map_err(Error::from)
            .and_then(|result| result
                .ok_or_else(|| Error::NotFound {
                    table_name: self.redb_table.name().to_string(),
                    key: key_bytes.to_vec(),
                })
                .and_then(|serialized| V::deserialize(serialized.value()).map_err(Error::from))
            )
    }




    pub(crate) fn get_many_by_key_bytes(
        &self,
        keys: impl Iterator<Item = Vec<u8>>,
    ) -> impl Iterator<Item = Result<V, Error>> {
        keys
            .map(|key_bytes| {
                self.redb_table
                    .get(key_bytes.as_slice())
                    .map_err(Error::from)
                    .and_then(|result| result
                        .ok_or_else(|| Error::NotFound {
                            table_name: self.redb_table.name().to_string(),
                            key: key_bytes.clone(),
                        })
                        .and_then(|serialized| V::deserialize(serialized.value()).map_err(Error::from))
                    )
            })
    }
}