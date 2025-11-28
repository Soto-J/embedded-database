use crate::{domain::DatabaseError, domain::mock_data::User};

use bincode::config;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BinaryBTree {
    pub btree: BTreeMap<String, Vec<u8>>,
}

impl BinaryBTree {
    pub fn insert(&mut self, user: User) -> Result<(), DatabaseError> {
        let bytes = bincode::encode_to_vec(&user, config::standard())
            .map_err(|e| DatabaseError::BincodeEncodeError(e))?;

        self.btree.entry(user.id).or_insert(bytes);
        Ok(())
    }

    pub fn get<Key: AsRef<str>>(&self, key: Key) -> Result<Option<User>, DatabaseError> {
        if let Some(bytes) = self.btree.get(key.as_ref()) {
            let (user, _bytes) = bincode::decode_from_slice(&bytes, config::standard())
                .map_err(|e| DatabaseError::BincodeDecodeError(e))?;

            return Ok(Some(user));
        }

        Ok(None)
    }

    pub fn delete<Key: AsRef<str>>(&mut self, key: Key) -> Result<Option<User>, DatabaseError> {
        if let Some(user) = self.get(key.as_ref())? {
            self.btree.remove(&user.id);

            return Ok(Some(user));
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
