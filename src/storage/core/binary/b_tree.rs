use crate::domain::{DatabaseError, mock_data::User};

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BinaryBTree {
    // pub btree: BTreeMap<String, Vec<u8>>,
}

impl BinaryBTree {
    pub fn insert(&mut self, user: User) -> Result<(), DatabaseError> {
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
