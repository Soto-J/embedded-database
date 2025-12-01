use crate::domain::{DatabaseError, mock_data::User};
use heapless::{String, index_map::FnvIndexMap};

#[derive(Debug, Default, Clone)]
pub struct BinaryHashMap {
    pub map: FnvIndexMap<String<256>, String<512>, 64>,
}

impl BinaryHashMap {
    pub fn insert(&mut self, user: User) -> Result<(), DatabaseError> {}

    pub fn get(&self, key: String<256>) -> Result<Option<User>, DatabaseError> {
        todo!()
    }

    pub fn delete(&mut self, key: String<256>) -> Result<Option<User>, DatabaseError> {
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
