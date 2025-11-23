use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{domain::DatabaseError, domain::mock_data::User};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MyMap {
    pub map: HashMap<String, String>,
}

impl MyMap {
    pub fn insert(&mut self, user: User) -> Result<(), DatabaseError> {
        let record =
            serde_json::to_string(&user).map_err(|e| DatabaseError::JsonSerializationError(e))?;

        self.map.entry(user.name).or_insert(record);
        Ok(())
    }

    pub fn delete(&mut self, key: &str) {
        self.map.remove(key);
    }

    pub fn get(&self, key: &str) -> Result<Option<User>, DatabaseError> {
        if let Some(value) = self.map.get(key) {
            let user: User = serde_json::from_str(value)
                .map_err(|e| DatabaseError::JsonDeserializationError(e))?;

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
        let mut ds = MyMap::default();

        let user = User {
            name: "John".to_string(),
            id: "100".to_string(),
        };

        let response = ds.insert(user);

        assert!(response.is_ok())
    }

    #[test]
    fn should_return_record() {
        let mut ds = MyMap::default();

        let user = User {
            name: "John".to_string(),
            id: "100".to_string(),
        };

        let response = ds.insert(user).expect("Failed to insert record");
    }

    #[test]
    fn should_delete_record() {
        let ds = MyMap::default();

        let user = User {
            name: "John".to_string(),
            id: "100".to_string(),
        };
    }
}
