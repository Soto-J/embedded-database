use crate::domain::{CoreError, DatabaseError, mock_data::User};

use serde::{Deserialize, Serialize};
use serde_json_core::{
    self,
    heapless::{FnvIndexMap, String},
};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct JsonHashMap {
    pub map: FnvIndexMap<String<256>, String<512>, 64>,
}

impl JsonHashMap {
    pub fn insert(&mut self, user: User) -> Result<(), DatabaseError> {
        let value: String<512> =
            serde_json_core::to_string(&user).map_err(|e| CoreError::JsonSerializationError(e))?;
        let key = user.id;

        self.map
            .insert(key, value)
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
