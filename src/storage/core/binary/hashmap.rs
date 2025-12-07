use crate::{
    domain::{CoreError, DatabaseError},
    storage::StorageEngine,
};
use heapless::{String, Vec, index_map::FnvIndexMap};
use serde::{Serialize, de::DeserializeOwned};

#[derive(Debug, Default, Clone)]
pub struct BinaryHashMap<T> {
    pub map: FnvIndexMap<String<256>, Vec<u8, 512>, 64>,
    // Why PhantomData?
    // Because the map stores JSON strings, not the actual T
    // But you want JsonHashMap to logically handle type T
    // PhantomData tells Rust “this struct behaves as if it owns a T”
    _marker: core::marker::PhantomData<T>,
}

impl<T> StorageEngine<T> for BinaryHashMap<T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    fn insert(&mut self, key: &String<256>, data: T) -> Result<(), DatabaseError> {
        let mut buf = [0u8; 512];
        let encoded = postcard::to_slice(&data, &mut buf)
            .map_err(|e| DatabaseError::Core(CoreError::BinaryEncodingError(e)))?;

        let value = Vec::from_slice(encoded)
            .map_err(|e| DatabaseError::Core(CoreError::CapacityError(e)))?;

        self.map
            .insert(key.clone(), value)
            .map_err(|_| DatabaseError::Core(CoreError::InsertionFailed))?;

        Ok(())
    }

    fn get(&self, key: &String<256>) -> Result<Option<T>, DatabaseError> {
        if let Some(value) = self.map.get(key) {
            let record: T = postcard::from_bytes(value.as_slice())
                .map_err(|e| DatabaseError::Core(CoreError::BinaryDecodingError(e)))?;

            return Ok(Some(record));
        }

        Ok(None)
    }

    fn delete(&mut self, key: &String<256>) -> Result<Option<T>, DatabaseError> {
        if let Some(user) = self.get(key)? {
            self.map.remove(key);

            return Ok(Some(user));
        }

        Ok(None)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_insert_record() {}

    #[test]
    fn should_return_record() {
        todo!()
    }

    #[test]
    fn should_delete_record() {
        todo!()
    }
}
