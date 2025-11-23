use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct User {
    name: String,
    age: u32,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Database {
    map: HashMap<String, serde_json::Value>,
}

impl Database {
    pub fn insert(&mut self, user: User) -> Result<(), DatabaseError> {
        let record = serde_json::to_string(&user)?;

        self.map.entry(user.name.clone()).or_insert(record);
        Ok(())
    }

    pub fn delete(&mut self, key: &str) {
        self.map.remove(key);
    }

    pub fn get(&self, key: &str) -> Option<&User> {
        if let Some(value) = self.map.get(key) {
            let user = serde_json::from_value(value.clone());
        }
        match self.map.get(key) {
            Some(user) => Some(serde_json::from_value(user)),
            None => None,
        }
    }
}

pub enum DatabaseError {
    SerializationError(serde_json::Error),
}

impl From<serde_json::Error> for DatabaseError {
    fn from(value: serde_json::Error) -> Self {
        Self(value)
    }
}