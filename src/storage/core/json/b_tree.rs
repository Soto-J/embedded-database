use crate::{
    domain::{DatabaseError, mock_data::User},
    storage::StorageEngine,
};
use heapless::String;
use serde::{Deserialize, Serialize, de::DeserializeOwned};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct JsonBTree<T> {
    // pub btree: BTreeMap<String, String>,
    _marker: core::marker::PhantomData<T>,
}

impl<T> StorageEngine<T> for JsonBTree<T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    fn insert(&mut self, key: &String<256>, data: T) -> Result<(), DatabaseError> {
        todo!();
    }

    fn get(&self, key: &String<256>) -> Result<Option<T>, DatabaseError> {
        todo!()
    }

    fn delete(&mut self, key: &String<256>) -> Result<Option<T>, DatabaseError> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_return_record() {
        todo!();
    }

    #[test]
    fn should_delete_record() {
        todo!()
    }
}
