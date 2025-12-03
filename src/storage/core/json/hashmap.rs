use crate::domain::{CoreError, DatabaseError, mock_data::User};

use heapless::{String, index_map::FnvIndexMap};
use serde::{Deserialize, Serialize};

// use serde_json_core::{self, heapless::FnvIndexMap};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct JsonHashMap {
    pub map: FnvIndexMap<String<256>, String<512>, 64>,
}

impl JsonHashMap {
    pub fn insert(&mut self, user: User) -> Result<(), DatabaseError> {
        let mut buf = [0u8; 512];

        // Serialize user into the buffer
        let used = serde_json_core::ser::to_slice(&user, &mut buf)
            .map_err(|e| CoreError::JsonSerializationError(e))?;

        // Convert &[u8] -> heapless::String<512>
        let mut value: heapless::String<512> = heapless::String::new();
        value
            .push_str(core::str::from_utf8(&buf[..used]).unwrap())
            .map_err(|_| CoreError::InsertionFailed)?;

        self.map
            .insert(user.id, value)
            .map_err(|_| CoreError::InsertionFailed)?;

        Ok(())
    }

    pub fn get(&self, key: String<256>) -> Result<Option<User>, DatabaseError> {
        if let Some(value) = self.map.get(&key) {
            let (user, _): (User, usize) = serde_json_core::from_str(value)
                .map_err(|e| CoreError::JsonDeserializationError(e))?;

            return Ok(Some(user));
        };

        Ok(None)
    }

    pub fn delete(&mut self, key: String<256>) -> Result<Option<User>, DatabaseError> {
        if let Some(user) = self.get(key.clone())? {
            self.map.remove(&key);

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
        todo!()
    }

    #[test]
    fn should_return_record() {
        todo!()
    }

    #[test]
    fn should_delete_record() {
        todo!()
    }
}
