use heapless::String;
use serde::{Serialize, de::DeserializeOwned};

use crate::domain::DatabaseError;

#[cfg(feature = "std")]
pub mod std;

pub mod core;

pub trait StorageEngine<T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    fn insert(&mut self, key: &String<256>, data: T) -> Result<(), DatabaseError>;
    fn get(&self, key: &String<256>) -> Result<Option<T>, DatabaseError>;
    fn delete(&mut self, key: &String<256>) -> Result<Option<T>, DatabaseError>;
}
