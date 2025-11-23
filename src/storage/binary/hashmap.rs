use std::collections::HashMap;

use bincode::{Decode, Encode, config};

use crate::{domain::DatabaseError, domain::mock_data::User};

#[derive(Debug, Default, Clone, Encode, Decode)]
pub struct MyMap {
    pub map: HashMap<String, usize>,
}

impl MyMap {
    pub fn insert(&mut self, user: User) -> Result<(), DatabaseError> {
        let config = config::standard();
        let mut dst = [];

        let bytes = bincode::encode_into_slice(user.clone().id, &mut dst, config)
            .map_err(|e| DatabaseError::BincodeEncodeError(e))?;

        self.map.entry(user.name).or_insert(bytes);
        Ok(())
    }

    pub fn delete(&mut self, key: &str) {
        self.map.remove(key);
    }

    pub fn get(&self, key: &str) -> Result<Option<User>, DatabaseError> {
        if let Some(value) = self.map.get(key) {
            let config = config::standard();

            let (user, _bytes_read): (User, usize) = bincode::decode_from_slice(value, config)
                .map_err(|e| DatabaseError::BincodeDecodeError(e))?;

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
