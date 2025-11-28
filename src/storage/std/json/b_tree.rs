use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::{domain::DatabaseError, domain::mock_data::User};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct JsonBTree {
    pub btree: BTreeMap<String, String>,
}

impl JsonBTree {
    pub fn insert(&mut self, user: User) -> Result<(), DatabaseError> {
        let record =
            serde_json::to_string(&user).map_err(|e| DatabaseError::JsonSerializationError(e))?;

        self.btree.entry(user.id).or_insert(record);
        Ok(())
    }

    pub fn get<Key: AsRef<str>>(&self, key: Key) -> Result<Option<User>, DatabaseError> {
        if let Some(value) = self.btree.get(key.as_ref()) {
            let user: User = serde_json::from_str(value)
                .map_err(|e| DatabaseError::JsonDeserializationError(e))?;

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
        let mut btree = JsonBTree::default();

        let user = create_mock_user();

        let response = btree.insert(user);

        assert!(response.is_ok())
    }

    #[test]
    fn should_return_record() {
        let mut btree = JsonBTree::default();

        let user = create_mock_user();

        let response = btree.insert(user.clone());

        assert!(response.is_ok());

        let response = btree.get(&user.id);

        assert!(response.is_ok())
    }

    #[test]
    fn should_delete_record() {
        let mut btree = JsonBTree::default();

        let user = create_mock_user();

        let response = btree.insert(user.clone());

        assert!(response.is_ok());

        let response = btree.delete(&user.id);

        assert!(response.is_ok());
    }
}
