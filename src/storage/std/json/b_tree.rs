use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::{domain::DatabaseError, domain::mock_data::User};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct JsonBTree<T> {
    pub btree: BTreeMap<String, String>,
    _marker: core::marker::PhantomData<T>,
}

impl<T> StorageEngine for JsonBTree<T> {
    fn insert<Key: AsRef<T>>(&mut self, key: Key, data: T) -> Result<(), DatabaseError> {
        let record =
            serde_json::to_string(&data).map_err(|e| DatabaseError::JsonSerializationError(e))?;

        self.btree.entry(key).or_insert(record);
        Ok(())
    }

    fn get<Key: AsRef<str>>(&self, key: Key) -> Result<Option<T>, DatabaseError> {
        if let Some(value) = self.btree.get(key.as_ref()) {
            let record: T = serde_json::from_str(value)
                .map_err(|e| DatabaseError::JsonDeserializationError(e))?;

            return Ok(Some(record));
        }

        Ok(None)
    }

    fn delete<Key: AsRef<str>>(&mut self, key: Key) -> Result<Option<T>, DatabaseError> {
        if let Some(record) = self.get(key.as_ref())? {
            self.btree.remove(&key);

            return Ok(Some(record));
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
