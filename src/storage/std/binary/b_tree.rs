use crate::{domain::DatabaseError, domain::mock_data::User};

use bincode::config;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BinaryBTree<T> {
    pub btree: BTreeMap<String, Vec<u8>>,
    _marker: core::marker::PhantomData<T>,
}

impl<T> StorageEngine for BinaryBTree<T> {
    fn insert<Key: AsRef<str>>(&mut self, key: Key, data: T) -> Result<(), DatabaseError> {
        let bytes = bincode::encode_to_vec(&data, config::standard())
            .map_err(|e| DatabaseError::BincodeEncodeError(e))?;

        self.btree.entry(key).or_insert(bytes);
        Ok(())
    }

    fn get<Key: AsRef<str>>(&self, key: Key) -> Result<Option<T>, DatabaseError> {
        if let Some(bytes) = self.btree.get(key.as_ref()) {
            let (data, _bytes) = bincode::decode_from_slice(&bytes, config::standard())
                .map_err(|e| DatabaseError::BincodeDecodeError(e))?;

            return Ok(Some(data));
        }

        Ok(None)
    }

    fn delete<Key: AsRef<str>>(&mut self, key: Key) -> Result<Option<T>, DatabaseError> {
        if let Some(data) = self.get(key.as_ref())? {
            self.btree.remove(&key);

            return Ok(Some(data));
        }

        Ok(None)
    }
}

#[cfg(test)]
mod test {
    use crate::domain::mock_data::create_mock_user;

    use super::*;

    #[test]
    fn should_insert_record() {
        let mut btree = BinaryBTree::default();

        let user = create_mock_user();

        let response = btree.insert(user);

        assert!(response.is_ok())
    }

    #[test]
    fn should_return_record() {
        let mut btree = BinaryBTree::default();

        let user = create_mock_user();

        let response = btree.insert(user.clone());

        assert!(response.is_ok());

        let response = btree.get(&user.id);

        assert!(response.is_ok())
    }

    #[test]
    fn should_delete_record() {
        let mut btree = BinaryBTree::default();

        let user = create_mock_user();

        let response = btree.insert(user.clone());

        assert!(response.is_ok());

        let response = btree.delete(&user.id);

        assert!(response.is_ok());
    }
}
