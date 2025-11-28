use crate::domain::{DatabaseError, mock_data::User};
use bincode::{Decode, Encode};

#[derive(Debug, Default, Clone, Encode, Decode)]
pub struct BinaryHashMap {
    // pub map: HashMap<String, Vec<u8>>,
}

impl BinaryHashMap {
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
