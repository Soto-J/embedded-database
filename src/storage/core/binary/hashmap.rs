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

        let value = Vec::from_slice(encoded)?;

        self.map
            .insert(user.id, value)
            .map_err(|_| CoreError::InsertionFailed)?;

        Ok(())
    }

    pub fn get(&self, key: String<256>) -> Result<Option<User>, CoreError> {
        if let Some(user) = self.map.get(&key) {
            let user: User = postcard::from_bytes(user.as_slice())?;

            return Ok(Some(user));
        }

        Ok(None)
    }

    pub fn delete(&mut self, key: String<256>) -> Result<Option<User>, DatabaseError> {
        if let Some(user) = self.get(key)? {
            self.map.remove(&user.id);

            return Ok(Some(user));
        }

        Ok(None)
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
