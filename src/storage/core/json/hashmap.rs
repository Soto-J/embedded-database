use crate::{
    domain::{CoreError, DatabaseError},
    storage::StorageEngine,
};

use heapless::{String, index_map::FnvIndexMap};
use serde::{Deserialize, Serialize, de::DeserializeOwned};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct JsonHashMap<T> {
    pub map: FnvIndexMap<String<256>, String<512>, 64>,
    // Why PhantomData?
    // Because the map stores JSON strings, not the actual T
    // But you want JsonHashMap to logically handle type T
    // PhantomData tells Rust “this struct behaves as if it owns a T”
    _marker: core::marker::PhantomData<T>,
}

impl<T> StorageEngine<T> for JsonHashMap<T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    fn insert(&mut self, key: &String<256>, record: T) -> Result<(), DatabaseError> {
        let mut buf = [0u8; 512];

        // Serialize T to buffer
        let used = serde_json_core::ser::to_slice(&record, &mut buf)
            .map_err(|e| CoreError::JsonSerializationError(e))?;

        // Convert &[u8] -> heapless::String<512>
        let mut value: heapless::String<512> = heapless::String::new();
        value
            .push_str(core::str::from_utf8(&buf[..used]).unwrap())
            .map_err(|_| CoreError::InsertionFailed)?;

        self.map
            .insert(key.clone(), value)
            .map_err(|_| CoreError::InsertionFailed)?;

        Ok(())
    }

    fn get(&self, key: &String<256>) -> Result<Option<T>, DatabaseError> {
        if let Some(value) = self.map.get(key) {
            let (record, _): (T, usize) = serde_json_core::from_str(value)
                .map_err(|e| CoreError::JsonDeserializationError(e))?;

            return Ok(Some(record));
        };

        Ok(None)
    }

    fn delete(&mut self, key: &String<256>) -> Result<Option<T>, DatabaseError> {
        if let Some(data) = self.get(key)? {
            self.map.remove(key);

            return Ok(Some(data));
        }

        Ok(None)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_insert_record() {
        todo!()
    }

    #[test]
    fn should_return_record() {
        todo!()
    }

    #[test]
    fn should_delete_record() {
        todo!()
    }
}
