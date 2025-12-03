use crate::domain::{CoreError, DatabaseError, mock_data::User};
use heapless::{String, Vec, index_map::FnvIndexMap};

#[derive(Debug, Default, Clone)]
pub struct BinaryHashMap {
    pub map: FnvIndexMap<String<256>, Vec<u8, 512>, 64>,
}

impl BinaryHashMap {
    pub fn insert(&mut self, user: User) -> Result<(), CoreError> {
        let mut buf = [0u8; 512];
        let encoded = postcard::to_slice(&user, &mut buf)?;

        let bytes = Vec::from_slice(encoded)?;

        self.map
            .insert(user.id, bytes)
            .map_err(|_| CoreError::InsertionFailed)?;

        Ok(())
    }

    pub fn get(&self, key: String<256>) -> Result<Option<User>, DatabaseError> {
        let user = self
            .map
            .get(&key)
            .ok_or_else(|| CoreError::InsertionFailed)?;
        
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
    fn should_insert_record() {}

    #[test]
    fn should_return_record() {
        todo!()
    }

    #[test]
    fn should_delete_record() {
        todo!()
    }
}
