use crate::{
    domain::{CoreError, DatabaseError},
    storage::StorageEngine,
};

use heapless::String;
use scapegoat::SgMap;
use serde::{Serialize, de::DeserializeOwned};

#[derive(Debug, Default, Clone)]
pub struct JsonBTree<T> {
    pub btree: SgMap<String<256>, String<512>, 64>,
    _marker: core::marker::PhantomData<T>,
}

impl<T> StorageEngine<T> for JsonBTree<T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    fn insert(&mut self, key: &String<256>, data: T) -> Result<(), DatabaseError> {
        // Serialize `data` into a fixed-size stack buffer (no heap allocations).
        let mut buf = [0u8; 512];
        let used = serde_json_core::ser::to_slice(&data, &mut buf)
            .map_err(|e| CoreError::JsonSerializationError(e))?;

        // JSON output is a UTF-8 byte slice. Convert &[u8] → &str safely.
        let uft8 = core::str::from_utf8(&buf[..used]).map_err(|e| CoreError::Utf8Error(e))?;

        // Copy UTF-8 JSON string into a fixed-capacity heapless String<512>.
        let mut value = String::<512>::new();
        value
            .push_str(uft8)
            .map_err(|e| CoreError::CapacityError(e))?;

        self.btree
            .insert(key.clone(), value)
            .ok_or_else(|| CoreError::InsertionFailed)?;

        Ok(())
    }

    fn get(&self, key: &String<256>) -> Result<Option<T>, DatabaseError> {
        if let Some(value) = self.btree.get(key) {
            let (record, _): (T, usize) = serde_json_core::from_str(&value)
                .map_err(|e| DatabaseError::Core(CoreError::JsonDeserializationError(e)))?;

            return Ok(Some(record));
        }

        Ok(None)
    }

    fn delete(&mut self, key: &String<256>) -> Result<Option<T>, DatabaseError> {
        if let Some(data) = self.get(key)? {
            self.btree.remove(key);

            return Ok(Some(data));
        }

        Ok(None)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_return_record() {
        todo!();
    }

    #[test]
    fn should_delete_record() {
        todo!()
    }
}
