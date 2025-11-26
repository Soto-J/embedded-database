use std::collections::HashMap;

use bincode::{Decode, Encode, config};

use crate::{domain::DatabaseError, domain::mock_data::User};

#[derive(Debug, Default, Clone, Encode, Decode)]
pub struct BinaryHashMap {
    pub map: HashMap<String, Vec<u8>>,
}

impl BinaryHashMap {
    pub fn insert(&mut self, user: User) -> Result<(), DatabaseError> {
        let bytes = bincode::encode_to_vec(user.clone(), config::standard())
            .map_err(|e| DatabaseError::BincodeEncodeError(e))?;

        self.map.entry(user.id).or_insert(bytes);
        Ok(())
    }

    pub fn get<Key: AsRef<str>>(&self, key: Key) -> Result<Option<User>, DatabaseError> {
        if let Some(bytes) = self.map.get(key.as_ref()) {
            let (user, _): (User, usize) = bincode::decode_from_slice(&bytes, config::standard())
                .map_err(|e| DatabaseError::BincodeDecodeError(e))?;

            return Ok(Some(user));
        }

        Ok(None)
    }

    pub fn delete<Key: AsRef<str>>(&mut self, key: Key) -> Result<Option<User>, DatabaseError> {
        if let Some(user) = self.get(key.as_ref())? {
            self.map.remove(&user.id);

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

        let response = map.insert(user);

        assert!(response.is_ok())
    }

    #[test]
    fn should_return_record() {
        let mut map = BinaryHashMap::default();

        let user = User {
            name: "John".to_string(),
            id: "100".to_string(),
        };

        let response = map.insert(user.clone());

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
