use crate::{domain::DatabaseError, domain::mock_data::User};

use bincode::config;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BinaryHashMap<T> {
    pub map: HashMap<String, Vec<u8>>,
    _marker: core::marker::PhantomData<T>,
}

impl<T> BinaryHashMap<T> {
    pub fn insert<Key: AsRef<str>>(&mut self, key: Key, record: T) -> Result<(), DatabaseError> {
        let bytes = bincode::encode_to_vec(record.clone(), config::standard())
            .map_err(|e| DatabaseError::BincodeEncodeError(e))?;

        self.map.entry(user.id).or_insert(bytes);
        Ok(())
    }

    pub fn get<Key: AsRef<str>>(&self, key: Key) -> Result<Option<T>, DatabaseError> {
        if let Some(bytes) = self.map.get(key.as_ref()) {
            let (record, _): (T, usize) = bincode::decode_from_slice(&bytes, config::standard())
                .map_err(|e| DatabaseError::BincodeDecodeError(e))?;

            return Ok(Some(record));
        }

        Ok(None)
    }

    pub fn delete<Key: AsRef<str>>(&mut self, key: Key) -> Result<Option<T>, DatabaseError> {
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
    fn should_insert_record() {
        let mut map = BinaryHashMap::default();

        let user = User {
            name: "John".to_string(),
            id: "100".to_string(),
        };

        let response = map.insert(user.id, user);

        assert!(response.is_ok())
    }

    #[test]
    fn should_return_record() {
        let mut map = BinaryHashMap::default();

        let user = User {
            name: "John".to_string(),
            id: "100".to_string(),
        };

        let response = map.insert(&user.id, user.clone());

        assert!(response.is_ok());

        let response = map.get(&user.id);

        assert!(response.is_ok())
    }

    #[test]
    fn should_delete_record() {
        let mut map = BinaryHashMap::default();

        let user = User {
            id: "100".to_string(),
            name: "John".to_string(),
        };

        let response = map.insert(user.clone());

        assert!(response.is_ok());

        let response = map.delete(&user.id);

        assert!(response.is_ok());
    }
}
