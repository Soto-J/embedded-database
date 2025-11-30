use crate::domain::{DatabaseError, mock_data::User};

use heapless::{String, index_map::FnvIndexMap};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct JsonHashMap {
    pub map: FnvIndexMap<String<32>, String<32>, 64>,
}

impl JsonHashMap {
    pub fn insert(&mut self, user: User) -> Result<(), DatabaseError> {
        let value: String<32> =
            serde_json::to_string(&user).map_err(|e| DatabaseError::JsonSerializationError(e))?;

        self.map.insert(user.id, value);
        todo!()
    }

    pub fn get<Key: AsRef<str>>(&self, key: Key) -> Result<Option<User>, DatabaseError> {
        todo!()
    }

    pub fn delete<Key: AsRef<str>>(&mut self, key: Key) -> Result<Option<User>, DatabaseError> {
        todo!()
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
