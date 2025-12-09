use crate::{
    domain::{CoreError, DatabaseError},
    storage::StorageEngine,
};

use heapless::{String, Vec};
use scapegoat::SgMap;
use serde::{Serialize, de::DeserializeOwned};

#[derive(Debug, Default, Clone)]
pub struct BinaryBTree<T> {
    pub btree: SgMap<String<256>, Vec<u8, 512>, 64>,
    _marker: core::marker::PhantomData<T>,
}

impl<T> StorageEngine<T> for BinaryBTree<T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    fn insert(&mut self, key: &String<256>, data: T) -> Result<(), DatabaseError> {
        // Serialize `data` into a fixed-size stack buffer using postcard.
        // `to_slice` guarantees no heap allocation and returns only the
        // portion of the buffer containing valid binary-encoded data.
        let mut buf = [0u8; 512];
        let used = postcard::to_slice(&data, &mut buf)
            .map_err(|e| DatabaseError::Core(CoreError::BinaryEncodingError(e)))?;

        // Copy the encoded bytes into a fixed-capacity heapless Vec<u8, 512>.
        // This enforces strict memory bounds: if the encoded form exceeds 512 bytes,
        // `extend_from_slice` returns an error instead of overflowing.
        let mut value: Vec<u8, 512> = Vec::new();
        value
            .extend_from_slice(used)
            .map_err(|e| DatabaseError::Core(CoreError::CapacityError(e)))?;

        self.btree.insert(key.clone(), value);

        Ok(())
    }

    fn get(&self, key: &String<256>) -> Result<Option<T>, DatabaseError> {
        if let Some(bytes) = self.btree.get(key) {
            let data: T = postcard::from_bytes(&bytes)
                .map_err(|e| DatabaseError::Core(CoreError::BinaryDecodingError(e)))?;

            return Ok(Some(data));
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
