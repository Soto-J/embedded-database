use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{domain::DatabaseError, domain::mock_data::User};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MyBTree {
    pub map: HashMap<String, String>,
}

impl MyBTree {
    pub fn insert(&mut self, user: User) -> Result<(), DatabaseError> {
        todo!()
    }

    pub fn delete(&mut self, key: &str) {
        todo!()
    }

    pub fn get(&self, key: &str) -> Result<Option<User>, DatabaseError> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_insert_record() {
        let mut ds = MyBTree::default();

        let user = User {
            name: "John".to_string(),
            id: "100".to_string(),
        };

        let response = ds.insert(user);

        assert!(response.is_ok())
    }

    #[test]
    fn should_return_record() {
        let mut ds = MyBTree::default();

        let user = User {
            name: "John".to_string(),
            id: "100".to_string(),
        };

        let response = ds.insert(user).expect("Failed to insert record");
    }

    #[test]
    fn should_delete_record() {
        let ds = MyBTree::default();

        let user = User {
            name: "John".to_string(),
            id: "100".to_string(),
        };
    }
}
