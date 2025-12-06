use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{domain::DatabaseError, domain::mock_data::User};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct JsonHashMap<T> {
    pub map: HashMap<String, String>,
    _marker: core::marker::PhantomData<T>,
}

impl<T> StorageEngine for JsonHashMap<T> {
    fn insert<Key: AsRef<str>>(&mut self, key: Key, data: T) -> Result<(), DatabaseError> {
        let record =
            serde_json::to_string(&data).map_err(|e| DatabaseError::JsonSerializationError(e))?;

        self.map.entry(key).or_insert(record);
        Ok(())
    }

    fn get<Key: AsRef<str>>(&self, key: Key) -> Result<Option<T>, DatabaseError> {
        if let Some(data) = self.map.get(key.as_ref()) {
            let record: T = serde_json::from_str(data)
                .map_err(|e| DatabaseError::JsonDeserializationError(e))?;

            return Ok(Some(record));
        }

        Ok(None)
    }

    fn delete<Key: AsRef<str>>(&mut self, key: Key) -> Result<Option<T>, DatabaseError> {
        if let Some(user) = self.get(key.as_ref())? {
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
        let mut map = JsonHashMap::default();

        let user = User {
            name: "John".to_string(),
            id: "100".to_string(),
        };

        let response = map.insert(user);

        assert!(response.is_ok())
    }

    #[test]
    fn should_return_record() {
        let mut map = JsonHashMap::default();

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
        let mut map = JsonHashMap::default();

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
